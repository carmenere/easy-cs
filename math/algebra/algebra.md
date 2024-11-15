# Table of contents
- [Table of contents](#table-of-contents)
- [Abstract algebra](#abstract-algebra)
  - [Common axioms](#common-axioms)
  - [Identity element](#identity-element)
  - [Inverse element](#inverse-element)
- [Algebraic structures with one binary operation](#algebraic-structures-with-one-binary-operation)
  - [Magma](#magma)
  - [Semigroup](#semigroup)
  - [Monoid](#monoid)
  - [Group](#group)
  - [Subgroup](#subgroup)
  - [Abelian group](#abelian-group)
- [Algebraic structures with two binary operations](#algebraic-structures-with-two-binary-operations)
  - [Semiring](#semiring)
  - [Ring](#ring)
  - [Commutative ring](#commutative-ring)
  - [Field](#field)

<br>

# Abstract algebra
**Abstract algebra** studies **algebraic structures**.<br>

An **algebraic structure** $`A`$ consists of
- a **nonempty set** $`S = \{a,\space b,\dots,\}`$ (aka **underlying set**, **carrier set** or **domain**);
- a **collection of operations** on $`S`$, **every** operation must be **closed**, i.e. for all $`x`$, $`y`$ in $`S`$, the **result** of the **operation** $`(x \cdot y) \in S`$;
- a **set of axioms**, that these operations must satisfy;

<br>

For example, an **algebraic structure** $`A`$ with **2** operations can be denoted as $`(A; \cdot; +)`$

<br>


## Common axioms
1. **Commutativity**: an operation $`\cdot`$ is **commutative** if $`x\cdot y = y\cdot x`$ for every $`x`$ and $`y`$ in the algebraic structure.
2. **Associativity**: an operation is **associative** if $`(x\cdot y)\cdot z = x\cdot (y\cdot z)`$ for every $`x`$, $`y`$ and $`z`$ in the algebraic structure.
3. **Left distributivity**: an operation $`\cdot`$ is **left distributive** with respect to another operation $`+`$ if $`x\cdot (y+z) = x\cdot y + x\cdot z`$ for every $`x`$, $`y`$ and $`z`$ in the algebraic structure.
4. **Right distributivity**: an operation $`\cdot`$ is **right distributive** with respect to another operation $`+`$ if $`(y+z)\cdot x = y\cdot x + z\cdot x`$ for every $`x`$, $`y`$ and $`z`$ in the algebraic structure..
5. **Distributivity**: an operation $`\cdot`$ is **distributive** with respect to another operation $`+`$ if it is **both** *left distributive* and *right distributive*.

<br>

## Identity element
**Identity element** (aka **neutral element**) of a binary operation is an element that leaves unchanged every element when the operation is applied. For example, $`0`$ is an **identity element** of the **addition** of *real numbers*.

Let $`(S, \cdot)`$ be a set $`S`$ with a binary operation $`\cdot`$.<br>
An element $`e`$ of $`S`$ is called a **left identity** if $`e \cdot s = s`$ for all $`s`$ in $`S`$.<br>
An element $`e`$ of $`S`$ is called a **right identity** if $`s \cdot e = s`$ for all $`s`$ in $`S`$.<br>

If $`e`$ is **both** a *left identity* and a *right identity*, then it is called a **two-sided identity**, or simply an **identity**.<br>

The **identity elements** for **addition** and **multiplication** are denoted $`0`$ and $`1`$, respectively.

<br>

## Inverse element
Let $`(S; \cdot )`$ be a set $`S`$ with a binary operation $`\cdot`$ and **indentity** $`e`$.<br>
An element $`l`$ of $`S`$ is called a **left inverse** of $`y`$ if $`l \cdot y = e`$.<br>
An element $`r`$ of $`S`$ is called a **right inverse** of $`y`$ if $`y \cdot r = e`$.<br>

If **left inverse** of $`y`$ and **right inverse** of $`y`$ are equal and unique then they are called the **inverse element** or simply the **inverse**.<br>
An **invertible element** is an element that has an **inverse**.<br>

<br>

# Algebraic structures with one binary operation
## Magma
**Magma** (rarely, **groupoid**) is a basic algebraic structure. Magma **doesn't has axioms**, but it **closed** by defenition:
- for all $`x`$, $`y`$ in $`M`$, the **result** of the **operation** $`x \cdot y`$ is also in $`M`$.

<br>

## Semigroup
**Semigroup** is a *magma* with **associative** operation.

<br>

## Monoid
**Monoid** is a *semigroup* with **identity**.

<br>

## Group
**Group** is a *monoid* in which **every element** has an **inverse** element.

There are **2** main notations for operations for **groups**:
- **additive notation**: $`+`$ sign;
- **multiplicative notation**: $`\cdot`$ sign;

<br>

It is a **common convention** that for an **abelian** group either **additive** or **multiplicative** notation may be used, but for a **nonabelian** group **only** **multiplicative** notation is used.

<br>

## Subgroup
Formally, given a group $`G`$ under a binary operation $`\cdot`$, a **subset** $`H`$ of $`G`$ is called a **subgroup** of $`G`$ **if** $`H`$ also forms a **group** under the operation $`\cdot`$.<br>

A **proper subgroup** of a group $`G`$ is a **subgroup** $`H`$ which is a **proper subset** of $`G`$ (that is, $`H ≠ G`$). This is often represented as $`H < G`$, read as "$`H`$ is a proper subgroup of $`G`$".<br>

<br>

## Abelian group
**Abelian group** is a *group* with **commutative** operation.

<br>

# Algebraic structures with two binary operations
## Semiring
A **semiring** is an algebraic structure with **two** operations: **addition** $`+`$ and **multiplication** $`\cdot`$.<br>

**Semiring axioms**:
- *ring* is an **commutative monoid** under $`+`$;
- *ring* is a **monoid** under $`\cdot`$;
- $`\cdot`$ is **distributive** with respect to $`+`$;

<br>

## Ring
A **ring** is an algebraic structure with **two** operations: **addition** $`+`$ and **multiplication** $`\cdot`$.<br>

**Ring axioms**:
- *ring* is an **abelian group** under $`+`$;
- *ring* is a **monoid** under $`\cdot`$;
- $`\cdot`$ is **distributive** with respect to $`+`$;

<br>

## Commutative ring
**Commutative ring** is a **ring** in which the **multiplication** operation is **commutative**.

<br>

## Field
A **field** is an algebraic structure with **two** operations: **addition** $`+`$ and **multiplication** $`\cdot`$.<br>

**Field axioms**:
- *field* is an **abelian group** under $`+`$;
- *field* is a **abelian group** under $`\cdot`$;
- $`\cdot`$ is **distributive** with respect to $`+`$;
