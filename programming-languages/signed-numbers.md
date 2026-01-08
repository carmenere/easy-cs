# Table of contents
- [Table of contents](#table-of-contents)
- [Signed number representation](#signed-number-representation)
- [Sign–magnitude](#signmagnitude)
- [Ones' complement](#ones-complement)
- [Two's complement](#twos-complement)
- [Overflow arithmetic](#overflow-arithmetic)
  - [Signed integers](#signed-integers)
  - [Unsigned integers](#unsigned-integers)

<br>

# Signed number representation
**Signed number representation** is an approach to represent **signed numbers** in binary format.

<br>

The most popular methods to represent **signed numbers**: 
- **sign–magnitude**;
- **ones' complement**;
- **two's complement**

<br>

# Sign–magnitude
In **sign–magnitude** the number consists of the **most significant digit** (**MSD**) and the **magnitude** (**absolute value**).<br>
The **magnitude** is an **absolute value** of the number.<br>
The **MSD** is the **sign bit**:
- if the **MSD** is a `0`, this indicates that the number is **positive**;
- if the **MSD** is a `1`, this indicates that the number is **negative**;

<br>

Consider **decimal numbers**: `0`, `1`, `8`, `127`, then in **sign–magnitude**:
- `+0` is represented as `00000000`;
- `-0` is represented as `10000000`;
- `+1` is represented as `00000001`;
- `-1` is represented as `10000001`;
- `+8` is represented as `00000111`;
- `-8` is represented as `10000111`;
- `+127` is represented as `01111111`;
- `-127` is represented as `11111111`;

<br>

**Drawbacks**:
- there are two ways to represent zero, `00000000` (`+0`) and `10000000` (`−0`);
- **addition** and **subtraction** require **different behavior** depending on the **sign bit**;
- **comparison** also requires inspecting the **sign bit**;

<br>

# Ones' complement
The **ones' complement** (aka **1’s complement**) of a binary number is the value obtained by **inverting** (**flipping**) all the bits in the binary representation of the number.<br>
Zero in **ones' complement** is represented as `11111111` (`-0`).<br>
The term **ones'** refers to the fact that such an **inverted value**, if added to the **original**, would always produce a **zero**, which consists of ones: `11111111`.<br>
The term **complement** refers to such pairs of **mutually additive inverse** numbers: `n + (-n) = 0`.<br>

Consider **decimal numbers**: `0`, `1`, `8`, `127`, then in **ones' complement**:
- `+0` is represented as `00000000`;
- `-0` is represented as `11111111`;
- `+1` is represented as `00000001`;
- `-1` is represented as `11111110`;
- `+8` is represented as `00000111`;
- `-8` is represented as `11111000`;
- `+127` is represented as `01111111`;
- `-127` is represented as `10000000`;

<br>

**Drawbacks**:
- there are two ways to represent zero, `00000000` (`+0`) and `10000000` (`−0`);

<br>

# Two's complement
Almost all modern computers use **two’s complement arithmetic** to represent **positive** and **negative** numbers.<br>

There is a simple algorithm to convert a binary number into **two’s complement**:
1. Take **ones' complement** of number.
2. Add `1` to the **ones' complement**.

<br>

Consider **decimal numbers**: `0`, `1`, `8`, `127`, then in **two's complement**:
- `+0` is represented as `00000000`;
- `+1` is represented as `00000001`;
- `-1` is represented as `11111111`;
- `+8` is represented as `00000111`;
- `-8` is represented as `11111001`;
- `+127` is represented as `01111111`;
- `-127` is represented as `10000001`;

<br>

**Two's complement** representation is **unambiguous**, for instance, it has **only one zero**, represented as `00000000`.<br>

<br>

# Overflow arithmetic
**Fixed-precision arithmetic** is arithmetic on numbers that are represented in a **fixed number of digits**.<br>
**Arbitrary-precision arithmetic** is used to **avoid overflow**, but it is slower than **fixed-precision arithmetic**.<br>

**Fixed-precision arithmetic** is susceptible to **overflow**. There are 2 ways to handle **overflow**:
- **wraparound** means if an **overflow occurs** (the resulting value $R$ exeeds $n$ bits), the resulting number $R$ is **wrapped around** to $R \space \text {mod} \space 2^{n}$;
- **saturation** means if an **overflow occurs** (the resulting value $R$ exeeds $n$ bits), the resulting number $R$ is **clamped** to the maximum or to the minimum value;

<br>

In CS, **clamping** is the process of limiting a value to a range between a minimum and a maximum value.<br>

Example of `clamp` function:
```python
def clamp(x, minimum, maximum)
    if x < minimum:
        return minimum
    if x > maximum:
        return maximum
    return x
```

<br>

**Code**:
```rust
fn main() {
    println!("MIN and MAX:");
    println!("  i8::MIN={}", i8::MIN);
    println!("  i8::MAX={}", i8::MAX);
    println!("  u8::MIN={}", u8::MIN);
    println!("  u8::MAX={}", u8::MAX);

    println!("Wraparound, signed integers:");
    println!("  i8::MAX + 1: {}", i8::MAX + 1);
    println!("  i8::MAX + 2: {}", i8::MAX + 2);
    println!("  i8::MIN - 1: {}", i8::MIN - 1);
    println!("  i8::MIN - 2: {}", i8::MIN - 2);
    
    println!("Wraparound, unsigned integers:");
    println!("  u8::MAX + 1: {}", u8::MAX + 1);
    println!("  u8::MAX + 2: {}", u8::MAX + 2);
    println!("  u8::MIN - 1: {}", u8::MIN - 1);
    println!("  u8::MIN - 2: {}", u8::MIN - 2);
    
    println!("Saturation, signed integers:");
    println!("  i8::MAX + 1: {}", i8::MAX.saturating_add(1));
    println!("  i8::MAX + 2: {}", i8::MAX.saturating_add(2));
    println!("  i8::MIN - 1: {}", i8::MIN.saturating_sub(1));
    println!("  i8::MIN - 2: {}", i8::MIN.saturating_sub(1));
    
    println!("Saturation, unsigned integers:");
    println!("  u8::MAX + 1: {}", u8::MAX.saturating_add(1));
    println!("  u8::MAX + 2: {}", u8::MAX.saturating_add(2));
    println!("  u8::MIN - 1: {}", u8::MIN.saturating_sub(1));
    println!("  u8::MIN - 2: {}", u8::MIN.saturating_sub(1));
}
```

**Output**:
```shell
RUSTFLAGS="$RUSTFLAGS -A arithmetic_overflow" cargo run --release
   Compiling foo v0.1.0 (/private/tmp/foo)
    Finished `release` profile [optimized] target(s) in 0.19s
     Running `target/release/foo`
MIN and MAX:
  i8::MIN=-128
  i8::MAX=127
  u8::MIN=0
  u8::MAX=255

Wraparound, signed integers:
  i8::MAX + 1: -128
  i8::MAX + 2: -127
  i8::MIN - 1: 127
  i8::MIN - 2: 126

Wraparound, unsigned integers:
  u8::MAX + 1: 0
  u8::MAX + 2: 1
  u8::MIN - 1: 255
  u8::MIN - 2: 254

Saturation, signed integers:
  i8::MAX + 1: 127
  i8::MAX + 2: 127
  i8::MIN - 1: -128
  i8::MIN - 2: -128

Saturation, unsigned integers:
  u8::MAX + 1: 255
  u8::MAX + 2: 255
  u8::MIN - 1: 0
  u8::MIN - 2: 0
```

<br>

## Signed integers
**Two's complement** encoding with $n$ bits gives to ranges of numbers:
- **negative**: $[−2^{𝑛−1}, \space −1]$
- **positive**: $[0, \space 2^{𝑛−1}−1]$
- **negative** + **positive**:$[−2^{𝑛−1}, \space 2^{𝑛−1}−1]$

<br>

In *two's complement arithmetic* the **sign** of number is determined by **MSB**:
- `0` MSB for **positive**;
- `1` MSB for **negative**;

<br>

**Properties** of *two's complement*:
- the **most negative value**, $100...00$ **has no positive counterpart**:
  - flip $100...00$ -> $011...11$;
  - adding 1 results $100...00$ again;
- in *two's complement arithmetic* **addition** or **subtraction** of two numbers with **different** signs **can never produce overflow**;
- **addition** or **subtraction** can cause to overflow only if 2 numbers have the **same sign**, both positive or both negative;
  - to detect overflow in *two's complement arithmetic* it is enough to compare sign of operands and the sign of result: result has the **opposite sign** it means overflow occured;

<br>

**Two's complement wraparound arithmetic** means that `MAX+1` wraps around to `MIN` and vice versa `MIN-1` wraps around to `MAX`:
- `INT_MIN` ia a `MIN` for **signed** integer **min**;
- `INT_MAX` is a `MAX` for **signed** integer **max**;

<br>

**Two's complement wraparound arithmetic** for **unsigned** integers:
- *addition*:
  - `INT_MAX + 1` wraps around to `INT_MIN`;
  - `INT_MAX + 2` wraps around to `INT_MIN+1`;
- *sabtraction*:
  - `INT_MIN - 1` wraps around to `INT_MAX`;
  - `INT_MIN - 1` wraps around to `INT_MAX-1`;

<br>

Example of **4-bit two's complement arithmetic**:<br>
![signed_overflow](/img/signed_overflow.png)

<br>

## Unsigned integers
For **unsigned** integers *two's complement* is **not** used.<br>

**Wraparound arithmetic** means that `MAX+1` wraps around to `MIN` and vice versa `MIN-1` wraps around to `MAX`:
- `UINT_MIN` is a `MIN` for **signed** integer **min**;
- `UINT_MAX` is a `MAX` for **signed** integer **max**;

<br>

**Wraparpund arithmetic** for **unsigned** integers:
- *addition*:
  - `UINT_MAX + 1` wraps around to `0`;
  - `UINT_MAX + 2` wraps around to `1`;
- *sabtraction*:
  - `UINT_MIN - 1` wraps around to `UINT_MAX`;
  - `UINT_MIN - 2` wraps around to `UINT_MAX-1`;
