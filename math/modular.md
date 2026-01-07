# Arithmetic operations
- the **plus sign** (`+`) denotes the operation of **addition**:
  - $summand + summand = sum/total$
  - $addend + addend = sum/total$
  - where:
    - **minuend** is the number from which another number is subtracted;
    - **subtrahend** is the number being subtracted;
    - **difference** is the **result** of *subtraction*;
- the **minus sign** (`-`) denotes **subtraction**, its result is a **difference**:
  - $minuend - subtrahend = difference$
  - where:
    - **minuend** is the number from which another number is subtracted;
    - **subtrahend** is the number being subtracted;
    - **difference** is the **result** of *subtraction*;
- the **multiplication sign** (cross symbol $\times$, asterisk $\ast$ or mid-line dot operator $\cdot$) denotes the operation of **multiplication** and is read as **times**:
  - $multiplicand \cdot multiplier = product$
  - $factor \cdot factor = product$
  - where:
    - **multiplicand** is the number to be multiplied;
    - **multiplier** is the number by which it is multiplied;
      - both **multiplicand** and **multiplier** can be referred to as **factors**;
    - **product** is the **result** of *multiplication*;
    - when **one factor** is an **integer**, then *product* is called **multiple**, in other words a **multiple** of a number (integer or not) is the **product** of that number and an **integer**:
      - if $x = k \cdot y$ and $k$ in **integer** then $x$ is a **multiple** of $y$;
- the **division sign** (/) denotes the operation of **division**:
  - $dividend / divisor = quotient$
  - where:
    - **dividend** is the number to be divided;
    - **divisor** is the number by which it is divided;
    - **quotient** is the **result** of *division*;
- **division with remainder** is the process of dividing one *integer* $a$ (the **dividend**) **by** *another* $b$ (the **divisor**) in a way that produces an *integer* **quotient** $q$ and an *integer* **remainder** $r$ that is **strictly smaller** than the absolute value of the **divisor** ($|b|$ denotes the **absolute value** of $b$):
  - given two integers $a$ and $b$ with $b \ne 0$, there exist **unique integers** $q$ and $r$ such that
    - $a = b \cdot q + r$ **and** $0 \le r \lt |b|$;
  - an integer $a$ **evenly divisible** by another integer $b$ if **remainder** $r$ is **zero**;


<br>

# Modulo operation
Given two integers $a$ and $b$ with $b \ne 0$, there exist **unique integers** $q$ and $r$ such that $a = b \cdot q + r$ **and** $0 \le r \lt |b|$.<br>
The **modulo operation** (denoted as $a \space mod \space b$ or $a \space \% \space b$) returns the **remainder** $r$ of a **division** $a$ **by** $b$: $r = a \space mod \space b$. The **divisor** $b$ is called the **modulus** in the **modulo operation** $a \space mod \space b$.<br>

<br>

Example of **modulo operation** for **modulus** $3$:
- $\dfrac{0}{3} \Rightarrow q=0, r=0$

- $\dfrac{1}{3} \Rightarrow q=0, r=1$

- $\dfrac{2}{3} \Rightarrow q=0, r=2$

- $\dfrac{3}{3} \Rightarrow q=1, r=0$

- $\dfrac{4}{3} \Rightarrow q=1, r=1$

- $\dfrac{5}{3} \Rightarrow q=1, r=2$

- $\dfrac{6}{3} \Rightarrow q=2, r=2$

<br>

The remainders start at $0$ and increases by **one** each time, until the number reaches **one less** than the **modulus**. After that, the **sequence repeats**.<br>

<br>

Given **modulus** $m$, if we increase some integer number $a$ by a **multiple of** $m$ we will get the new integer that gives the **same remainder**: $a \space \text {mod} \space m = (a + k\cdot m) \space \text {mod} \space m = r, \space \forall k \in \mathbb{N}$.<br>

For example:
- $3 \mod 10 = 3$
- $13 \mod 10 = 3$
- $23 \mod 10 = 3$
- $33 \mod 10 = 3$

<br>

The **modulo operation** can be visualized by using circles. If the number is **positive** we step **clockwise**, if it's **negative** we step **counter-clockwise**.<br>

Negative **modulo operation**:<br>
$-a \mod m = m - (a \mod m)$

<br>

Example for $\text {mod} \space 4$:
![](/img/mod_4_example.png)

<br>

- $-1 \mod 4 = 4 - (1 \mod 4) = 3 \mod 4$
- $-2 \mod 4 = 4 - (2 \mod 4) = 2 \mod 4$
- $-3 \mod 4 = 4 - (3 \mod 4) = 1 \mod 4$

<br>

**Generalization** for $\text {mod} \space m$:
![](/img/mod_m_example.png)

<br>

# Divisor of. Prime and coprime integers
An integer $a$ is **divisible** or **evenly divisible** by another integer $b$ if the $a$ **can be divided by** $b$ with **no remainder**. Then $b$ is called **divisor of** $a$.<br>

The $1$ and $−1$ **divide** (are **divisors of**) **every integer**.<br>
**Every integer** and **its negation** is a **divisor of itself**.<br>
Integers **divisible** by $2$ are called **even**, and integers **not** *divisible* by $2$ are called **odd**.<br>

The $1$, $−1$, $n$ and $−n$ are known as the **trivial divisors** of $n$.<br>
A *divisor* of $n$ that is **not** a *trivial divisor* is known as a **non-trivial divisor**.<br>
A *positive divisor* of $n$ that is **different from** $n$ is called a **proper divisor**.<br>

Defenitons of **prime number**:
- a **prime number** is a positive integer $n>1$ whose **only** *proper divisor* is $1$;
- a **prime number** is a positive integer $n>1$ that has exactly **two** positive factors: $1$ and **itself**;
- **prime numbers** have no non-trivial divisors;

Two numbers $a$ and $b$ are **coprime**, **relatively prime** or **mutually prime** if the **only** positive integer that is a **divisor** of **both** of them is $1$. It is said $a$ is **coprime with** $b$.<br>
Two numbers $a$ and $b$ are **coprime** if their **GCD** is **equal** to **1**: $\gcd({a},{b})=1$.<br>

<br>

# Modular arithmetic
Two integers $a$ and $b$ are said to be **congruent modulo** $m$, if $m$ is a **divisor** of their **difference** $a-b$:
$a − b = k \cdot m \space \forall k \in \mathbb{N}$.<br>

**Congruence modulo** $m$ is denoted by $a \equiv b \pmod{m}$. The **parentheses** mean that $(\text {mod} \space m)$ applies to the **entire equation**, not just to the right-hand side (here, $b$).<br>

The congruence relation may be rewritten as $a = k \cdot m + b$. However, the $b$ here **need not** be the remainder in the division of $a$ by $m$.
Rather, $a \equiv b \pmod{m}$ asserts that $a$ and $b$ yield the **same remainder** when **divided** by $m$. That is,
- $a = p \cdot m + r$;
- $b = q \cdot m + r$;
  - where $0 \le r \lt m$;

So,
- $a - b = p \cdot m + r - q \cdot m - r = p \cdot m - q \cdot m = (p-q) \cdot m = k \cdot m$;
- $\Rightarrow \space k = p-q$;

Two integers, $a$ and $b$ are considered **congruent** under modulo $m$ if they yield the **same remainder** when **divided** by the positive integer $m$.<br>
In other words, expression $a \equiv b \pmod{m}$ means both $a$ and $b$ yield the **same remainder** when **divided** by the positive integer $m$.<br>

<br>

# Residue systems
The **congruence relation** is an **equivalence relation**. <br>
The **equivalence class modulo** $m$ of an integer $a$ is the **set of integers** $\{a + k \cdot m: \forall k \in \mathbb{Z}\}$.<br>
The **equivalence class modulo** $m$ of an integer $a$ is also called **congruence class** or **residue class** of $a$ modulo $m$.<br>
There are exactly $m-1$ **classes** for **modulo** $m$.<br>
Each **residue class modulo** $m$ may be represented by any one of its members, although we usually represent each residue class by the **smallest** **nonnegative** integer which belongs to that class.<br>
A **complete residue system modulo** $m$ is a **set** that contains **precisely one element from each** *residue class modulo* $m$.<br>
It is obvious that **no two** elements of a **complete residue system modulo** $m$ are **congruent** modulo $m$. They are all **incongruent** each other.<br>

The set of integers $\{0, 1, 2, ..., m − 1\}$ is called the **least residue system modulo** $m$.<br>
The **least residue system** is a *complete residue system*.<br>

<br>

# GCD. LCM
The **G**reatest **C**ommon **D**ivisor (**GCD**) of two integers $a$ and $b$, usually denoted by $\gcd(a,b)$, is the **largest** positive integer $d$ such that $d$ is a **divisor of** both $a$ and $b$.<br>

When one of $a$ and $b$ is **zero**, the **GCD** is the absolute value of the **nonzero integer**: $\gcd(a,0) = \gcd(0,a) = |a|$.<br>

**Example**: $\gcd(54,24)=6$.<br>

The **L**east **C**ommon **M**ultiple (**LCM**) of two integers $a$ and $b$, usually denoted by $\text{lcm}(a,b)$, is the **smallest** positive integer that is **divisible by** both $a$ and $b$.

**Example**: $\text{lcm}(4,6)=12$.<br>

The **LCM** can be computed from the **GCD**:
- $\text{lcm}(a,b) = {\dfrac {|ab|}{\gcd(a,b)}}$.<br>
