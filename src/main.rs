use std::fmt::{Display, Formatter, Result as FmtResult};

use finitelib::prelude::*;


// Bit integer type
type BigInt = bigi_of_bits!(8192);


// Decimal datatype that keeps N digits after the dot
#[derive(Debug, Clone)]
pub struct Decimal<const N: usize>(BigInt);


// Display for Decimal<N>
impl<const N: usize> Display for Decimal<N> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut r = self.0.clone();
        let q = r.divide(&Self::order()).unwrap();
        write!(f, "{}.{}", q.to_decimal(), r.to_decimal())
    }
}


// Convert from u64
impl<const N: usize> From<u64> for Decimal<N> {
    fn from(x: u64) -> Self {
        Self::new(&BigInt::from(x))
    }
}


impl<const N: usize> Decimal<N> {
    // Constructor from Bigi
    pub fn new(x: &BigInt) -> Self {
        Self(x * &Self::order())
    }

    // 10 power N value
    pub fn order() -> BigInt {
        let mut x = BigInt::from(1);
        for _ in 0..N {
            x.mul_unit(10);
        }
        x
    }

    // Operation add
    pub fn add(&self, rhs: &Self) -> Self {
        Self(&self.0 + &rhs.0)
    }

    // Operation sub
    pub fn sub(&self, rhs: &Self) -> Self {
        Self(&self.0 - &rhs.0)
    }

    // Operation mul
    pub fn mul(&self, rhs: &Self) -> Self {
        let x = &self.0 * &rhs.0;
        Self(&x / &Self::order())
    }

    // Operation div
    pub fn div(&self, rhs: &Self) -> Self {
        let x = &self.0 * &Self::order();
        Self(&x / &rhs.0)
    }
}


fn main() {
    // Number of iterations
    const SQRT2_ITERATIONS: u64 = 20;
    const PI_ITERATIONS: u64 = 1500;

    // Decimal for 1000 digits
    type Dec = Decimal<1000>;

    // Calculate sqrt2 by the Newton's method
    let sqrt2 = {
        let mut x = Dec::from(1);
        for _ in 0..SQRT2_ITERATIONS {
            x = Dec::from(2).div(&x).add(&x).div(&Dec::from(2));
        }
        x
    };
    println!("sqrt2 = {}", sqrt2);

    // Calculate pi as arctan series of (sqrt2 - 1) that is equal to pi/8
    let z = sqrt2.sub(&Dec::from(1));
    let mut sum = Dec::from(0);
    let mut zk = z.clone();
    let mut sign = 1;
    for k in 0..PI_ITERATIONS {
        if sign == 1 {
            sum = sum.add(&zk.div(&Dec::from(2 * k + 1)));
        } else {
            sum = sum.sub(&zk.div(&Dec::from(2 * k + 1)));
        }
        zk = zk.mul(&z).mul(&z);
        sign = -sign;
    }
    let pi = sum.mul(&Dec::from(8));
    println!("pi = {}", pi);
}
