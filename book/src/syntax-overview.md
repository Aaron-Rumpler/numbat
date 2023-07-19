# Syntax overview

``` numbat
# This is a line comment. It can span over
# multiple lines

# 1. Imports

use prelude       # This is not necessary. The 'prelude'
                  # module will always be loaded upon startup

use physics::temperature_conversion   # Load a specific module

# 2. Numbers

12345       # integer notation
12_345      # optional decimal separators

0.234       # floating point notation
.234        # without the leading zero

1.234e15    # scientific notation
1.234e+15
1e-9
1.0e-9

0x2A        # hexadecimal
0o52        # octal
0b101010    # binary

# 3. Simple expressions

3 + (4 - 3)       # Addition and subtraction

1920 / 16 * 9     # Multiplication, division
1920 ÷ 16 × 9     # Unicode-style, '·' is also multiplication
2 pi              # Whitespace is implicit multiplication
meter per second  # 'per' keyword can be used for division

2^3               # Exponentiation
2**3              # Python-style
2³                # Unicode exponents

17 % 4            # Modulo

3 in -> cm        # Unit conversion, can also be → or ➞
3 in to cm        # Unit conversion with the 'to' keyword

cos(pi/3)         # Call mathematical functions

# 4. Variable definitions

let x = 4                          # Simple numerical variable
let y1 = 2 m/s                     # Right hand side can be any expression
let y2: Speed = 2 m/s              # With optional type annotation
let y3: Length / Time = 2 m/s      # more complex type annotation

# 5. Function definitions

fn foo(x: Scalar) -> Scalar = 2 * x + 3         # A simple function
fn speed(s: Length, t: Time) -> Speed = s / t   # Two parameters
fn my_sqrt<T>(x: T^2) -> T = x^(1/2)            # A generic function

# 6. Dimension definitions

dimension Fame                            # A new base dimension
dimension Deceleration = Length / Time^2  # A new derived dimension

# 7. Unit definitions

@aliases(quorks)                 # Optional aliases-decorator
unit quork = 0.35 meter          # A new derived unit

@metric_prefixes                 # Optional decorator to allow 'milliclonk', etc.
@aliases(ck: short)              # short aliases can be used with short prefixes (mck)
unit clonk: Time = 0.2 seconds   # Optional type annotation

@metric_prefixes
@aliases(wh: short)
unit warhol: Fame                # New base unit for phys. dimension "Fame"

unit thing                       # New base unit with automatically generated
                                 # base dimension "Thing"

# 8. Procedures

print(2 kilowarhol)              # Print a quantity
assert_eq(1 ft, 12 in)           # Assert that two quantities are equal
```