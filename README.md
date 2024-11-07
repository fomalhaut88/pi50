# Digits of PI

This program calculate digits of the number `pi`. The result is:

```
pi = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491...
```

## Run

`cargo run --release`

## Math

1. First the program calculates `sqrt2` by the Newton's iterative method:

```
x -> (x + 2/x) / 2
```

2. The value of `pi` is calculated as the arctan series for `(sqrt2 - 1)`, because `tan(pi/8) = sqrt2 - 1`:

```
pi/8 = arctan(z) = z - z^3/3 + z^5/5 - z^7/7 + ..., z = sqrt2 - 1 < 1
```
