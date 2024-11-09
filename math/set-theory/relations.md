# Table of contents
- [Table of contents](#table-of-contents)
- [Cartesian product](#cartesian-product)
- [N-ary relations](#n-ary-relations)
- [Binary relations](#binary-relations)
- [Homogeneous binary relation](#homogeneous-binary-relation)
    - [Properties of homogeneous relations](#properties-of-homogeneous-relations)
  - [Equivalence relations](#equivalence-relations)
    - [Partial equivalence relation](#partial-equivalence-relation)
    - [Equivalence relation](#equivalence-relation)
  - [Order relations](#order-relations)
    - [Weak partial order or just Partial order (≤)](#weak-partial-order-or-just-partial-order-)
    - [Strict partial order (\<)](#strict-partial-order-)
    - [Weak total order or just Total order (≤)](#weak-total-order-or-just-total-order-)
    - [Strict total order (\<)](#strict-total-order-)
- [Converse relation](#converse-relation)
- [Composition relations](#composition-relations)
  - [Properties](#properties)
  - [Examples](#examples)
    - [Example1](#example1)
    - [Example2](#example2)
- [Power of relation](#power-of-relation)

<br>

# Cartesian product
Given sets $X$ and $Y$, the **Cartesian product** ${X\times Y}$ is a **new set** $`\{(x,y)\mid x\in X{\text{ and }}y\in Y\}`$ and its elements $(x,y)$ are called **ordered pairs**.<br>

<br>

# N-ary relations
An **n-ary relation** on $n$ sets, is **any subset** of **Cartesian product** of the $n$ sets.<br>

More formal: **n-ary relation** $R$ over sets (in *some order*) ${X_{1},\dots ,X_{n}}$ is **any subset** of **Cartesian product** (in the *same order*): $R \subseteq{X_{1}\times \cdots \times X_{n}}$.<br>

**Alternativly**: **n-ary relation** $R$ over sets (in *some order*) ${X_{1},\dots ,X_{n}}$ is an element of the **powerset** of ${X_{1}\times \cdots \times X_{n}}$ ($2^{X_{1}\times \cdots \times X_{n}}$).<br>

<br>

A *relation* is called a **homogeneous relation** when **all** sets are **equal to each other**: $X_i=Y_j \ \ ∀i \in [1,n],\  ∀j \in [1,n]$.<br>
A *relation* is called a **heterogeneous relation** when **all** sets are **distinct**: $X_i\neq Y_j \ \ ∀i\neq j, \  i\in [1,n],\  j \in [1,n]$.<br>

<br>

# Binary relations
A **binary relation** is the **2-ary relation** ($n=2$).<br>

A **binary relation** $R$ **from** the set $X$ **to** the set $Y$ (aka **binary relation** $R$ **over** sets $X$ and $Y$) is **any subset** of **Cartesian product**: $R \subseteq {X \times Y}$.<br>

Alternativly: a **binary relation** $R$ **from** the set $X$ **to** the set $Y$ is **any element** of the **power set** of $X\times Y$: $R \in 2^{X\times Y}$.<br>

In other words, **binary relation** associates elements of one set with elements of another set:
- the set $X$ is called the **domain** or **set of departure** of $R$;
- the set $Y$ the **codomain** or **set of destination** of $R$;

<br>

In a binary relation, the **order of the elements** is **important**: if $x \neq y$ then $yRx$ can be `true` or `false` **independently** of $xRy$. For example, `3 divides 9` $\Rightarrow$ `true`, but `9 divides 3` $\Rightarrow$ `false`.<br>

<br>

Examples of relations:
- **is greater than**;
- **is equal to**;
- **divides**;
- **was born before**;
- there is a **direct flight** from airport $x$ to airport $y$;

<br>

A *binary relation* is also called a **heterogeneous binary relation** when $X\neq Y$, i.e. if $X$ and $Y$ are **distinct sets**.<br>
A *binary relation* is called a **homogeneous binary relation** when $X=Y$: $R \subseteq {X \times X}$.<br>

<br>

# Homogeneous binary relation
Given a set $X$, a **homogeneous binary relation** is a **binary relation** $R$ on a set $X$ such that $`R ⊆ \{ (x,y)\ | \ \  x,y ∈ X \}`$.<br>
The statement $(x,y) ∈ R$ is read "**x is R-related to y**" and is written in **infix** notation as $xRy$.<br>

A **homogeneous relation** $R$ over a set $X$ may be represented as **directed simple graph permitting loops**, where $X$ is the **vertex set** and $R$ is the **edge set** (there is an *edge* $`e_{xy}`$ **from** a vertex $x$ **to** a vertex $y$ **iif** $xRy$).<br>

<br>

### Properties of homogeneous relations
1. **Reflexivity**: for all `x ∈ X`: `xRx` returns `true`, i.e. every element is related to itself.
   - For example, `≥` is a **reflexive** relation but `>` is **not**.
2. **Irreflexivity** (aka **Anti-reflexivity**): for all `x ∈ X`: `xRx` returns `false`,  i.e. no element is related to itself .
   - For example, `>` is an **irreflexive** relation, but `≥` is **not**.
3. **Symmetry**: for all `a,b ∈ X`: IF `aRb` THEN `bRa`.
   - For example, **blood relation** is a **symmetric** relation.
4. **Antisymmetry**: 
   1. For all `a,b ∈ X`: IF `aRb` AND `bRa` THEN `a=b`.
   2. Alternative definition, for all `a,b ∈ X`: IF `aRb` AND `a≠b` THEN `bRa` returns `false`.
   3. In other words, no two distinct elements precede each other.
   4. Example, `≥` is an **antisymmetric** relation.
5. **Asymmetry**: it is **antisymmetry** + **irreflexivity**:
   1. For all `a,b ∈ X`: IF `aRb` THEN `bRa` returns `false`.
   2. In other words, IF `a` is related to `b` THEN `b` is **not** related to `a`.
   3. For example, `>` is an **asymmetric** relation, but `≥` is **not**.
6. **Transitivity**: for all `a,b ∈ X`: IF `aRb` AND `bRc` THEN `aRc`.
7. **Weak connectivity** (aka **Weak totality**): for all `a,b ∈ X`: IF `a≠b` THEN `aRb` or `bRa`.
8. **Strong connectivity** (aka **Strong totality**): for all `a,b ∈ X`: `aRb` or `bRa`.

<br>

## Equivalence relations
### Partial equivalence relation
**Partial equivalence relation** (**PER**) is a binary relation that satisfies the following properties:
1. **Symmetry**: IF `a=b` THEN `b=a`.
2. **Transitivity**: IF `a=b` AND `b=c`, THEN `a=c`.

> **Note**:<br>
> **Partial equivalence relation**  is **not reflexive**, i.e., `a != a`.<br>

<br>

### Equivalence relation
**Equivalence relation** is a binary relation that satisfies the following properties:
1. **Reflexivity**: Any number a is equal to itself.
2. **Symmetry**: IF `a=b` THEN `b=a`.
3. **Transitivity**: IF `a=b` AND `b=c`, THEN `a=c`.

<br>

## Order relations
There are 2 type of order:
1. **Partial order**. The word **partial** is used to indicate that **not every pair** of elements are **comparable**; that is, there may be pairs for which neither element precedes the other.
2. **Total order**. In **total order** every pair is **comparable**.

<br>

### Weak partial order or just Partial order (≤)
**Partial order** (aka **non-strict/weak partial order**) is a binary relation that satisfies the following properties:
1. **Antisymmetry**.
2. **Reflexivity**.
3. **Transitivity**.

<br>

### Strict partial order (<)
**Strict partial order** is a binary relation that satisfies the following properties:
1. **Asymmetry**.
2. **Irreflexivity**.
3. **Transitivity**.

<br>

### Weak total order or just Total order (≤)
**Total order** (aka **non-strict/weak total order**) is the **weak partial order** with the added property of **strong connectivity**:
1. **Antisymmetry**.
2. **Reflexivity**.
3. **Transitivity**.
4. **Strong connectivity**.

<br>

### Strict total order (<)
**Strict total order** is the **strict partial order** with the added property of **weak connectivity**:
1. **Asymmetry**.
2. **Irreflexivity**.
3. **Transitivity**.
4. **Weak connectivity**.

<br>

# Converse relation
The **converse of a binary relation** is the relation that occurs when the **order** of the elements is **switched** in the relation.<br>
For example, the **converse** of the relation **child of** is the relation **parent of**.<br>

if $X$ and $Y$ are sets and ${R\subseteq X\times Y}$ is a relation from $X$ to $Y$ then $`{R^T}`$ is the relation defined so that $`y{R^T}x`$ **iif** $xRy$.<br>

In *set-builder notation*: $`{R^T = \{(y,x)\in Y\times X:(x,y)\in R\}}`$

<br>

# Composition relations
Let $R$ is a **binary relation** from the set $X$ to the set $Y$, and $S$ is a binary relation from the set $Y$ to the set $Z$. The **composition of two binary relations** $R$ and $S$ is a **new binary relation** $R \circ S$ consisting of **ordered pairs** $(x,z)$ such that $(x,y) \in R$ and $(y,z) \in S$.<br>

**More formal**: if ${R\subseteq X\times Y}$ and ${ S\subseteq Y\times Z}$ are two binary relations, then their composition is the new relation $R \circ S$:<br>
$`R \circ S = \{(x,z)\in X\times Z:{\text{ there exists }}y\in Y{\text{ such that }}(x,y)\in R{\text{ and }}(y,z)\in S\}`$.<br>

<br>

## Properties
1. **Composition of relations** is **associative**: $R \circ (S \circ W) = (R \circ S) \circ W$.
2. The **converse relation** of $R \circ S$ is $`(R \circ S)^T = S^{\textsf T}\, \circ R^{\textsf T}`$.<br>

<br>

## Examples
### Example1
Let $`X = \{1,2,3\}`$, $`Y = \{1,2,3,4\}`$ and $`Z = \{0,1,2\}`$.<br>
Let $`R = \{(1,1), (1,4), (2,3), (3,1), (3,4)\}`$.<br>
Let $`S = \{(1,0), (2,0), (3,1), (3,2), (4,1)\}`$.<br>
Then, $`R \circ S = \{(1,0), (1,1), (2,1), (2,2), (3,0), (3,1)\}`$.<br>

<br>

### Example2
The relation **uncle of** (${xUz}$) is the **composition** of relations **is a brother of** (${xBy}$) and **is a parent of** (${yPz}$).<br>

**More formal**: $U = B \circ P$ is **equivalent** to $xByPz{\text{ if and only if }}xUz$.<br>

<br>

# Power of relation
${R^{i}}$ is the **i-th power** of $R$. The ${R^{i}}$ is defined inductively by 
- ${R^{1}=R}$;
- and for ${i>0}$, ${R^{i+1}=R\circ R^{i}}$;
