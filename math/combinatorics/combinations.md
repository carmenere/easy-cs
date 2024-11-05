# Table of contents
- [Table of contents](#table-of-contents)
- [Combinations](#combinations)
  - [Combinations without repetitions](#combinations-without-repetitions)
    - [Binomial coefficients](#binomial-coefficients)
  - [Combinations with repetitions](#combinations-with-repetitions)
    - [Proof](#proof)

<br>

# Combinations
A **combination** is an **arrangement** of items in which **order doesn't matter**.<br>

<br>

## Combinations without repetitions
A **$k$-combination of $n$** *without repetitions* is a **subset** of set $A$ with *cardinality* $k$.<br>

The **number** of all such **$k$-combination of $n$** is denoted as $`{\displaystyle C(n,k)}`$ or $`{\displaystyle C^{n}_{k}}`$ or $`{\displaystyle \binom{n}{k}}`$.<br>

**Formula**:
- $`{\displaystyle C^{n}_{k} = \frac{A^{n}_{k}}{k!} = \frac{n!}{k!*(n-k)!}}`$

<br>

**Proof**:<br>
- Each **$k$-combination of $n$** *without repetitions* has $k!$ **permutations**:
  - $`A^{n}_{k} = C^{n}_{k} \times k!`$
- So, it is obvious that:
  - $`{\displaystyle C^{n}_{k} = \frac{A^{n}_{k}}{k!} = \frac{n!}{k!*(n-k)!}}`$

<br>

When $k = n$, **$k$-combination of $n$** is simply called **combination of $n$**.<br>
The **number** of all such **combinations of $n$** is denoted as $C(n)$.<br>

Since the *order doesn't matter* in combinations, it is obvious that $`C^{n}_{k} < A^{n}_{k}`$, i.e. the all *combinations* are **subset** of all *permutations*.<br>

<br>

### Binomial coefficients
The $`{\displaystyle C^{n}_{k}}`$ is also called **binomial coefficient**.<br>

<br>

**Properties** of *binomial coefficients*:
- $`{\displaystyle C^{n}_{0} = C^{n}_{n}=1}`$
- $`{\displaystyle C^{n}_{k}=C^{n-1}_{k-1}+C^{n-1}_{k}}`$ for all integers $`{\displaystyle n,k}`$ such that $`{\displaystyle 1 \leq k \lt n}`$
- $`{\displaystyle \sum_{i=0}^{i=n} C^{n}_{k} = C^{n}_{0} + C^{n}_{1} + C^{n}_{2} + ... + C^{n}_{i} + ... + C^{n}_{n-1} + C^{n}_{n} = 2^n}`$

So, the number of **$k$-combination of $n$** for **all** $k∈[0;n]$ is the number of **all subsets** of a set $A$ of $n$ elements, i.e. it is **powerset**.<br>

<br>

## Combinations with repetitions
A **$k$-combination of $n$** *with repetitions* is a **multiset** with *cardinality* $k$ (aka **$k$-multisubset**) that consists of elements from $A$.<br>
The **number** of all such **$k$-multisubsets** is denoted as $`{\displaystyle \overline {C(n,k)}}`$ or $`\overline {C^{n}_{k}}`$ or $`{\displaystyle \left({\binom {n}{k}}\right)}`$.<br>

**Formula**:
- $`{\displaystyle \overline{C^{n}_{k}} = C^{n+k-1}_{k}}`$

<br>

### Proof
Consider set $X$ of $4$ elements: $X = \{a,b,c,d\}$. The $n=4$ is the **cardinality** of $X$.<br>
In every *k-multiset* the **order doesn't matter**, but two *k-multiset* **differs in the number of particular elements**, for example: two **4-multisets** $\{a,a,a,b\}$ and $\{a,a,b,b\}$ are **different**.<br>

So, we can represent **every** *k-multiset* as **ordered tuple** in which every **position** (**index**) corresponds to **particular element** from $X$ and contains the **number of occurrences** of this element in *k-multiset*. The **sum** of **all** such numbers in the tuple is **always equals** to the $k$ (the **cardinality** of such *k-multiset*).

**Example**. Let $a$ has **index** $0$, $b$ has **index** $1$, $c$ has **index** $2$, $d$ has **index** $3$, then:
- **4-multiset** $\{a,a,b,c\}$ will be encoded as follows: $(2,1,1,0)$;
- **4-multiset** $\{a,b,b,d\}$ will be encoded as follows: $(1,2,0,1)$;
- and so on;

<br>

Then we can use $0$ as **separator** between positions in tuples and instead the **number of occurrences** write corresponding number of $1$. Note that
- the number of zeros ($0$) in every tuple is $n-1$;
- the number of ones ($1$) in every tuple is $k$;

<br>

**Examples**:
- $`{\displaystyle (2,1,1,0) \implies \underbrace{11}_\text{a} \underbrace{0}_\text{separator} \underbrace{1}_\text{b} \underbrace{0}_\text{separator} \underbrace{1}_\text{c} \underbrace{0}_\text{separator} \underbrace{}_\text{d} \implies 1101010}`$;
- $`{\displaystyle (1,2,0,1) \implies \underbrace{1}_\text{a} \underbrace{0}_\text{separator} \underbrace{11}_\text{b} \underbrace{0}_\text{separator} \underbrace{}_\text{c} \underbrace{0}_\text{separator} \underbrace{1}_\text{d} \implies 1011001}`$;

<br>

So, **every** such *k-multiset* has **appropriate binary code** of **length** $k+n-1$ in which **zero** is repeated $(n-1)$ times and **ones** is repeated $k$ times and the number of **all** such **$k$-multisubsets** is equal to the number of **all** such **binary codes**.<br>

In turn, the number of **all** such **binary codes** is equal to number of  **permutations of multiset** of **length** $k+n-1$ in which **zero** is repeated $(n-1)$ times and **ones** is repeated $k$ times.<br>

<br>

**Finally**:
- $`{\displaystyle \overline{C^{n}_{k}} = P^{k+n-1}_{(n-1),k} = \frac{(k+n-1)!}{(n-1)!*k!} = C^{n+k-1}_{k}}`$