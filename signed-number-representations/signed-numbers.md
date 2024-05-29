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
- addition and subtraction require different behavior depending on the **sign bit**;
- comparison also requires inspecting the sign bit;

<br>

# Ones' complement
The **ones' complement** (aka **1’s complement**) of a binary number is the value obtained by **inverting** (**flipping**) all the bits in the binary representation of the number.<br>
Zero in **ones' complement** is represented as `11111111`.<br>
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
There is a simple algorithm to convert a binary number into 2’s complement:
1. Take **1’s complement** of number.
2. Add `1` to the **1’s complement**.

<br>

Consider **decimal numbers**: `0`, `1`, `8`, `127`, then in **ones' complement**:
- `+0` is represented as `00000000`;
- `+1` is represented as `00000001`;
- `-1` is represented as `11111111`;
- `+8` is represented as `00000111`;
- `-8` is represented as `11111001`;
- `+127` is represented as `01111111`;
- `-127` is represented as `10000001`;

<br>

**2’s complement** representation is **unambiguous**, for instance, it has **only one zero**, represented as `00000000`.<br>
