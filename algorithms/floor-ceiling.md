# Table of contents
- [Table of contents](#table-of-contents)
- [Absolute value](#absolute-value)
- [Sign function](#sign-function)
- [Integer part](#integer-part)
      - [Examples](#examples)
- [Floor and ceiling functions](#floor-and-ceiling-functions)
      - [Examples](#examples-1)
      - [Examples](#examples-2)
  - [Equations](#equations)
- [Fractional part](#fractional-part)
      - [Examples](#examples-3)

<br>

# Absolute value
The **absolute value** or **modulus** (denoted `|x|` or `abs(x)`) is the **non-negative** value of `x` without regard to its sign:
- `|x| = x` if `x > 0`;
- `|x| = -x` if `x < 0` (here `-x` becomes **positive** value);
- `|0| = 0` if `x = 0`;

<br>

# Sign function
The **sign function** or **signum function** (`sign(x) or sgn(x)`) extracts the **sign of a number**:
- `sign(x) = 1` if `x > 0`;
- `sign(x) = -1` if `x < 0`;
- `sign(0) = 0` if `x = 0`;

<br>

# Integer part
The **integer part** (aka **truncation function**) (denoted `[x]` or `truncation(x)` or `trunc(x)`) takes as *input* a *real number* `x`, and *returns* **integer ∈** `[0, x]` that is **closest** to `x`.<br>

*More formal*: `trunc(x) = sign(x)·⌊|x|⌋`.<br>

<br>

#### Examples
- `[23.7]` = `23`
- `[−23.7]` = `−23`

<br>

<br>

# Floor and ceiling functions
The **floor function** (denoted `⌊x⌋` or `floor(x)`) takes as *input* a *real number* `x`, and *returns* the **greatest** *integer* ≤ `x`.<br>

<br>

#### Examples
- `⌊2.4⌋` = `2`;
- `⌊−2.4⌋` = `−3`;

<br>

The **ceiling function** (denoted `⌈x⌉` or `ceil(x)`) takes as *input* a *real number* `x`, and *returns* the **smallest** *integer* ≥ `x`.<br>

<br>

#### Examples
- `⌈2.4⌉` = `3`;
- `⌈−2.4⌉` = `−2`;

<br>

## Equations
Let `x` and `y` are **real numbers**.<br>
Let `m` and `n` are **integer numbers**.<br>

Then:
- `⌊x⌋` ≤ `x` ≤ `⌈x⌉`;
- `x - 1` < `⌊x⌋` ≤ `x`;
- `x` ≤ `⌈x⌉` < `x + 1`;
- `⌊x + n⌋` = `⌊x⌋ + n`;
- `⌈x + n⌉` = `⌈x⌉ + n`;
- `⌊x⌋ + ⌊y⌋` ≤ `⌊x + y⌋` ≤ `⌊x⌋ + ⌊y⌋ + 1`;
- `⌈x⌉ + ⌈y⌉ - 1` ≤ `⌈x + y⌉` ≤ `⌈x⌉ + ⌈y⌉`;

<br>

If `n` is a **positive integer**:
- `n` = `⌈n/2⌉ + ⌊n/2⌋`

<br>

# Fractional part
The **fractional part** (aka **decimal part**) (denoted `{x}` or `frac(x)`) is the function that takes as *input* a *real number* `x`, and *returns* the **excess** beyond `x` integer part:
- `{x} = x - ⌊x⌋` if `x ≥ 0`;
- `{x} = x - ⌈x⌉` if `x < 0`;

<br>

#### Examples
- `[2.7]` = `0.7`
- `[-2.7]` = `-0.7`

<br>
