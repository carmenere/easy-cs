# Fractions
The **fraction** consists of **two integers** distinguished by **line** (aka **fraction bar**): **numerator** and a *non-zero* **denominator**.<br>
A **numerator** is the number **above** the line in a fraction and a **denominator** is the number **below** the line in a fraction.<br>
Example: a **fraction** ⁠of two integers `p/q`:
- `p` ia the **numerator**;
- `q` is **non-zero denominator**;

The **mediant** of **2** *fractions* $\dfrac{a}{c}$ and $\dfrac{b}{d}$ is **new** *fraction*: $\dfrac{a+b}{c+d}$.<br>

An **important property** of the *mediant* is that **it lies strictly between** the two fractions of which it is the mediant: if $\dfrac{a}{c} \lt \dfrac{b}{d}$ and $c \cdot d \gt 0$, then <br>
$\dfrac{a}{c} \lt \dfrac{a+b}{c+d} \lt \dfrac{b}{d}$.<br>

The **reducible fraction** is a *fraction* that can be can be **reduced** (aka **simplified**) by dividing both the *numerator* and *denominator* by a **common factor**: `2/4` simplifies to `1/2`.<br>
The **irreducible fraction** is a *fraction* where the *numerator* and *denominator* **share no common factors** other than **1**, meaning they are **coprime**, and the fraction **cannot** be **reduced** (aka **simplified**) further.<br>

To get *irreducible* **from** *reducible* **divide both** *numerator* and *denominator* **by their GCD**.<br>

A **rational number** or just **rational** is a number that can be expressed as a **fraction** ⁠of two integers `p/q`.<br>
**Every rational number** may be expressed in a unique way as an **irreducible fraction** ⁠$\dfrac{a}{b}$⁠ where $a$ and $b$ are **coprime** integers and $b \gt 0$.<br>
This is often called the **canonical form** of the *rational number*.<br>

<br>

# Stern-Brocot tree
The **Stern-Brocot tree** represents the **set of all positive fractions**.<br>

**Properties**:
- all fractions are **unique**;
- all fractions are **irreducible**;
- all fractions are **sorted in ascending order**;

<br>

There is an algorithm that builds **Stern-Brocot tree**: it starts with 2 fractions $\dfrac{0}{1}$ and $\dfrac{1}{0}$ and at every next iteration it adds all possible **mediants** between fractions. Continuing this process to infinity this covers all positive fractions.<br>

- $\dfrac{0}{1}$ represents **zero**: $0$;

- $\dfrac{1}{0}$ represent **infinity**: $\infty$;

<br>

The **first 3 iterations**:
- **start frations**: $\dfrac{0}{1}, \dfrac{1}{0}$

- iteration 1: $\dfrac{0}{1}, \dfrac{1}{1}, \dfrac{1}{0}$

- iteration 2: $\dfrac{0}{1}, \dfrac{1}{2}, \dfrac{1}{1}, \dfrac{2}{1}, \dfrac{1}{0}$

- iteration 3: $\dfrac{0}{1}, \dfrac{1}{3}, \dfrac{1}{2}, \dfrac{2}{3}, \dfrac{1}{1}, \dfrac{3}{2}, \dfrac{2}{1}, \dfrac{3}{1}, \dfrac{1}{0}$

<br>

The visualization of the **Stern-Brocot tree**:<br>
![stern_brocot_tree](/img/stern_brocot_tree.png)

<br>

Every path in the **Stern-Brocot tree** can be represented as sequence of **L** and **R** from the top node $\dfrac{0}{1}$ or $\dfrac{1}{0}$:
- **L** means **left** turn;
- **R** means **right** turn;

<br>

Continually **reversing directions** and walking down the tree wil produce path `LRLRLR...` or `RLRLRL...` and **increases** the **denominators** in a **Fibonacci sequence**:
- $\dfrac{1}{1}, \dfrac{1}{2}, \dfrac{2}{3}, \dfrac{3}{5}, \dfrac{5}{8}, \dfrac{8}{13},$ ...

<br>

The **n-th** Fibonacci number is denoted as $F(n)$:
- $F(47) = 2.971.215.073$ is the first Fibonacci number that exceeds `i32::MAX=2.147.483.647`;
- $F(48) = 4.807.526.976$ is the first Fibonacci number that exceeds `u32::MAX=4.294.967.295`;
- $F(93) = 12.200.160.415.121.876.738$ is the first Fibonacci number that exceeds `i64::MAX=9.223.372.036.854.775.807`;
- $F(94) = 19.740.274.219.868.223.167$ is the first Fibonacci number that exceeds `u64::MAX=18.446.744.073.709.551.615`;

<br>

# Farey sequence
The **left subtree** of the *Stern–Brocot tree*, containing the rational numbers in the closed interval `[0,1]`, is called the **Farey tree**.<br>
The **Farey sequence of order** $n$ (denoted as $F_{n}$) is the **sorted** sequence of **irreducible fractions** in the closed interval [0,1] whose denominators **do not exceed** $n$.<br>

The *Farey sequences* can be constructed using **mediants**: the Farey sequence of order $n + 1$ is formed from the Farey sequence of order $n$ by computing the mediant of each two consecutive values in the Farey sequence of order $n$.<br>

The first few are:
- $F_{1} = \{\dfrac{0}{1}, \dfrac{1}{1}\}$;

- $F_{2} = \{\dfrac{0}{1}, \dfrac{1}{2}, \dfrac{1}{1}\}$;

- $F_{3} = \{\dfrac{0}{1}, \dfrac{1}{3}, \dfrac{1}{2}, \dfrac{2}{3}, \dfrac{1}{1}\}$;

- $F_{4} = \{\dfrac{0}{1}, \dfrac{1}{4}, \dfrac{1}{3}, \dfrac{1}{2}, \dfrac{2}{3}, \dfrac{3}{4}, \dfrac{1}{1}\}$;

- $F_{5} = \{\dfrac{0}{1}, \dfrac{1}{5}, \dfrac{1}{4}, \dfrac{1}{3}, \dfrac{2}{5}, \dfrac{1}{2}, \dfrac{3}{5}, \dfrac{2}{3}, \dfrac{3}{4}, \dfrac{4}{5}, \dfrac{1}{1}\}$;
