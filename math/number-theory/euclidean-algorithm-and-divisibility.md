# Table of contents
- [Table of contents](#table-of-contents)
- [Euclidean division](#euclidean-division)
- [Greatest common divisor](#greatest-common-divisor)
- [The lowest common multiple](#the-lowest-common-multiple)
- [Prime and coprime numbers](#prime-and-coprime-numbers)
- [Congruence](#congruence)

<br>

# Euclidean division
The **computation** of the *quotient* and the *remainder* **from** the *dividend* and the *divisor* is called **Euclidean division** or **division with remainder**.<br>

**Notation**:
- $`\mathbb{Z}`$ is the **set of integers** $`\{... ,−2,−1,0,1,2, ...\}`$;
- $`a`$ is a **dividend**;
- $`b`$ is a **divisor**;
- $`q`$ is a **quotient**;
- $`r`$ is a **remainder**;

<br>

**Division theorem** (aka **Euclid's division lemma** or **division algorithm**): given **two integers** $`a`$ and $`b \ne 0`$, there **exist unique integers** $`q`$ and $`r`$ such that $`a = b \cdot q + r`$, where $0 \le r \lt |b|$, in other words, the **remainder is non-negative**.<br>
To compute $`q`$ the **floored division** used, in oter words, $`q`$ is always rounded **towards negative infinity**:
- $`+3.2 \rightarrow +3`$
- $`+3.8 \rightarrow +3`$
- $`-3.2 \rightarrow -4`$ 
- $`-3.8 \rightarrow -4`$ 

<br>

**Examples**:
- if $`a = 7`$ and $`b = 3`$, then $`q = +2.3 \rightarrow +2`$ and $`r = +1`$, since $`7 = 3 \cdot 2 + 1`$;
- if $`a = 7`$ and $`b = −3`$, then $`q = −2.3 \rightarrow -3`$ and $`r = +2`$, since $`7 = 3 \cdot (−3) + 2`$;

<br>

The number $`b`$ **divides** $`a`$ **without** *remainder* (written $`b \vert a`$ or $`a \vdots b`$) if there exists some $`k \in \mathbb{Z}`$ such that $`a = k\cdot b`$.<br>

When $`b \vert a`$ we also say $`a`$ is a **multiple** of $`b`$ or $`a`$ **perfectly divisible** by $`b`$ or $`a`$ **evenly divisible** by $`b`$.<br>

**Properties**:
- if $`a \vdots b`$, $`a_{1} \vdots b`$ and $`a_{2} \vdots b`$ then:
  -  $`(a_{1} + a_{2}) \vdots b`$;
  -  $`(a_{1} - a_{2}) \vdots b`$;
  -  $`(a \cdot k) \vdots b`$, where $`k \in \mathbb{N}`$;

<br>

# Greatest common divisor
The **greatest common divisor** (or **gcd**) of $`a`$ and $`b`$, written $`gcd(a,b)`$ or $`(a,b)`$, is the **largest** positive integer that **divides** both $`a`$ and $`b`$.

<br>

**Properties**:
- $`gcd(a,b) = gcd(b,r)`$
- $`gcd(a,b) = gcd(a-b,b)`$
- $`gcd(a,0) = gcd(0,a) = |a|`$, in other words, when one of $a$ and $b$ is **zero**, the **GCD** is the **absolute value** of the **nonzero integer**;
- $`gcd(a,b) = gcd(b,a)`$;
- if $`a \gt 0`$ and $`b \vert a`$ then $`gcd(a,b) = b`$;
- if $`a \equiv c \pmod{b}`$, then $`gcd(a,b) = gcd(c,b)`$;


<br>

**Theorem**. $`gcd(a,b)`$ **divides** $`a-b`$.<br>
Proof:
- by definition $`gcd(a,b)`$ divides $`a`$ and $`b`$;
- so exists such $`a'`$  that $`a=a' \cdot gcd(a,b)`$ and exists such $`b'`$  that $`b=b' \cdot gcd(a,b)`$;
- we see that $`a-b = a' \cdot gcd(a,b) - b' \cdot gcd(a,b) = (a' - b') \cdot gcd(a,b)`$;

So, $`gcd(a,b)`$ **divides** $`a-b`$.<br>

<br>

**Theorem**. Consider $`a`$, $`b`$, $`q`$ and $`r`$ such that: $`a = b \cdot q + r`$, where $0 \le r \lt |b|$, then $`gcd(a,b) = gcd(b,r)`$.<br>

**Intuition**: consider $a = b \cdot q + r$, where $0 \le r \lt |b|$. If both $`a`$ and $`b`$ have **common divisors** and $`d`$ is the **largest**, then $`d`$ **evenly divides** $`a`$ and $`d`$ **evenly divides** $`b`$ and $`d`$ **evenly divides** $`r`$:<br>
![gcd](/img/gcd.png)

<br>

To prove that $`gcd(a,b) = gcd(b,r)`$, we establish that the *set of all common divisors of* $`a`$ and $`b`$ is **identical** to the *set of common divisors of* $`b`$ and $`r`$:
- **step 1**: every *common divisor* of $`a`$ and $`b`$ is a divisor of $`b`$ and $`r`$:
  - assume $`d`$ is a *common divisor* of $`a`$ and $`b`$, this means $`d \vert a`$ and $`d \vert b`$;
  - since $`d \vert a`$ and $`d \vert bq`$, $`d`$ must divide their difference ($`r = a - bq`$), so $`d \vert r`$;
  - therefore, $`d`$ **divides both** $`b`$ and $`r`$;
- **step 2**: every *common divisor* of $`b`$ and $`r`$ is a divisor of $`a`$ and $`b`$:
  - assume $`d`$ is a *common divisor* of $`b`$ and $`r`$, this means $`d \vert b`$ and $`d \vert r`$;
  - so, $`d`$ **must divide** their **sum**;
    - thus, $`d \mid (bq + r)`$, meaning $`d \mid a`$;
  - therefore, $`d`$ **divides both** $`a`$ and $`b`$;
- **conclusion**:
  - because the set of **all** *common divisors of* $`(a, b)`$ is completely identical to the set of *common divisors of* $`(b, r)`$, their **greatest common divisors** must be the **same number**: $`gcd(a,b) = gcd(b,r)`$

<br>

The **Euclidean algorithm** is based on the **proved property** $`gcd(a,b) = gcd(b,r)`$.<br>

<br>

The **Euclidean algorithm**:
- to compute $`gcd(a,b)`$ where $a \ge b$, divide the **larger** number by the **smaller** number, so $`a=b \cdot q_{1} + r_{1}`$ and $`r_{1}<b`$;
- then by $`(a,b)=(b,r_{1})`$ we get $`b=r_{1} \cdot q_{2} + r_{2}`$ and $`r_{2}<r_{1}`$;
- then by $`(b,r_{1})=(r_{1}, r_{2})`$ we get $`r_{1}=r_{2} \cdot q_{3} + r_{3}`$ and $`r_{3}<r_{2}`$ , and so on;
- since $`b > r_{1} > r_{2} > r_{3} > ...`$, eventually some $`r_{k+1}=0`$ and $`(a,b) = (b,r_{1}) = (r_{1}, r_{2}) = ... = (r_{i-1}, r_{i}) = ... = (r_{k}, r_{k+1}) = (r_{k}, 0) = r_{k}`$, in other words, $`(a,b)`$ is the **last non-zero remainder** $`r_{k}`$ we compute;

<br>

Example:
- $`(198,168)=(168,30)`$
- $`=(30,18)`$
- $`=(18,12)`$
- $`=(12,6)`$
- $`=(6,0)`$
- $`=6`$

<br>

# The lowest common multiple
The **lowest common multiple** (aka **lcm**) of two numbers $`a`$ and $`b`$ is the **smallest positive integer** that is **perfectly divisible** by $`a`$ and $`b`$.<br>
Consider 2 numbers: $`a`$ and $`b`$ with $`gcd(a,b)=d`$. Then $`a`$ can be represented as $`a = a' \cdot d`$ and $`b`$ can be represented as $`b = b' \cdot d`$.<br>
It is obvious that $`a'`$ and $`b'`$ are **coprime** and their $`gcd(a',b')=1`$.<br>

So, $`a \cdot b = a' \cdot d \cdot b' \cdot d = a' \cdot b' \cdot d \cdot d`$ and this $`a' \cdot b' \cdot d`$ is an $`lcm(a,b)`$.<br>

So,
- $`lcm(a,b) \cdot gcd(a,b) = a \cdot b`$;
- $`lcm(a,b) = \dfrac{a \cdot b}{gcd(a,b)}`$;


<br>

# Prime and coprime numbers
An integer $`p>0`$ is called **prime** if it has **exactly** *two positive divisors*, namely, $`1`$ and $`p`$.<br>
If $`a>0`$ has **more** *than two positive divisors*, we say it is **composite**.
Tne number $`1`$ is **not prime neither composite**.<br>

A **coprime** (or **relatively prime**) relationship applies to **pairs of numbers**, meaning they **share no common divisors** other than $`1`$.<br>
A **gcd** of 2 **coprime numbers** $`a`$ and $`b`$ is **equal to** $`1`$: $`gcd(a,b) = 1`$.<br>

<br>

# Congruence
Given an integer $`m \ge 1`$, called a **modulus**, two integers $`a`$ and $`b`$ are said to be **congruent modulo** $`m`$, if their difference $`m \vert a - b`$; in other words, if there is an integer $`k`$ such that $`a − b = km`$.<br>

<br>

**Congruent modulo** $`m`$ is denoted by: $`a \equiv b \pmod{m}`$.<br>

<br>

**Note**, the $`a \equiv 0 \pmod{m}`$ means $`m \vert a`$.<br>

<br>

The **congruence relation** satisfies all the conditions of an **equivalence relation**:
- **reflexivity**: $`a \equiv a \pmod{m}`$;
- **symmetry**: $`a \equiv b \pmod{m}`$ **if and only if** $`b \equiv a \pmod{m}`$;
- **transitivity**: if $`a \equiv b \pmod{m}`$ and $`b \equiv c \pmod{m}`$, then $`a \equiv c \pmod{m}`$;

<br>

Properties:
- if $`a \equiv b \pmod{m}`$ and $`c \equiv d \pmod{m}`$, then:
  - $`a + k \equiv b + k \pmod{m}`$
  - $`a \cdot k \equiv b \cdot k \pmod{m}`$
  - $`a + c \equiv b + d \pmod{m}`$
  - $`a - c \equiv b - d \pmod{m}`$
  - $`a \cdot c \equiv b \cdot d \pmod{m}`$
  - $`a^{k} \equiv b^{k} \pmod{m}`$

<br>

Proof that $`a + c \equiv b + d \pmod{m}`$:
- $`a + c - (b + d) = a + c - b - d = (a - b) + (c - d)`$
- from $`c \equiv d \pmod{m}`$ we know that $`m \vert (c - d)`$
- from $`a \equiv b \pmod{m}`$ we know that $`m \vert (a - b)`$
- so, $`m`$ divides $`(a - b) + (c - d)`$ **without** *remainder*;
- so, both $`a+c`$ and $`b+d`$ **have the same remainder** when divided by $`m`$;

<br>

Proof that $`a - c \equiv b - d \pmod{m}`$:
- $`a - c - (b - d) = a - c - b + d = (a - b) - (c - d)`$
- from $`c \equiv d \pmod{m}`$ we know that $`m \vert (c - d)`$
- from $`a \equiv b \pmod{m}`$ we know that $`m \vert (a - b)`$
- so, $`m`$ divides $`(a - b) - (c - d)`$ **without** *remainder*;
- so, both $`a-c`$ and $`b-d`$ **have the same remainder** when divided by $`m`$;

<br>

**Fermat's little theorem**: If $`p`$ is **prime** and **does not divide** $`a`$, then $`a^{p-1} \equiv 1 \pmod{p}`$.<br>

<br>

