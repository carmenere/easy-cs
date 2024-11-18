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
  - [Semilattice](#semilattice)
- [Algebraic structures with two binary operations](#algebraic-structures-with-two-binary-operations)
  - [Semiring](#semiring)
  - [Ring](#ring)
  - [Commutative ring](#commutative-ring)
  - [Field](#field)
  - [Lattice](#lattice)
  - [Distributive lattice](#distributive-lattice)
  - [Boolean lattice](#boolean-lattice)
  - [Ring of sets](#ring-of-sets)
  - [Field of sets](#field-of-sets)
- [The algebra of sets](#the-algebra-of-sets)
- [Zero divisor](#zero-divisor)

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

## Semilattice
A **semilattice** $`(S, \cdot)`$ is a **commutative**, **idempotent semigroup**.<br>
A **bounded semilattice** $`(S, \cdot)`$ is a **commutative**, **idempotent monoid**.<br>

<br>

The **binary operation** $`\cdot`$ is **idempotent** if:
- ($`x \cdot x = x \space \space \forall x \in S`$)

<br>

There are **2** types of *semilattice*:
- a **join-semilattice** (or **upper semilattice**) $`(S, \lor)`$ is a **partially ordered set** $`S`$ with a binary operation $`\lor`$, called **join**, (aka **union**);
  - a **join-semilattice** $`(S, \lor)`$ is **bounded** if S includes an **identity element** $`0`$ (the lattice's **bottom**, **least element**) for the **join** operation such that $`a \lor 0 = a \space \space \forall a \in S`$;
- a **meet-semilattice** (or **lower semilattice**) $`(S, \land)`$ is a **partially ordered set** $`S`$ with a binary operation $`\land`$, called **meet**, (aka **intersection**);
  - a **meet-semilattice** $`(S, \land)`$ is **bounded** if S includes an **identity element** $`1`$ (aka **lattice's top** , **greatest element**) for the **meet** operation such that $`a \land 1 = a \space \space \forall a \in S`$;

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

<br>

## Lattice
A **lattice** is **both** a **join-semilattice** and a **meet-semilattice**.<br>
If $`(S, \lor)`$ and $`(S, \land)`$ are both **semilattices** over set $`S`$ and their binary operations are **connected** through **absorption laws**:
- $`a \lor (a \land b) = a \space \space \forall a \in S`$;
- $`a \land (a \lor b) = a \space \space \forall a \in S`$;

then an algebraic structure $`(L, \lor ,\land )`$ over set $`S`$ is called **lattice**.<br>

<br>

## Distributive lattice
A **distributive lattice** is a *lattice* $`(L, \lor ,\land )`$ in which **both operations** are **distribute** over each other:
- $`a \lor (b \land c) = (a \lor b) \land (a \lor c) \space \space \forall a,b,c \in L`$;
- $`a \land (b \lor c) = (a \land b) \lor (a \land c) \space \space \forall a,b,c \in L`$;

<br>

A **totally ordered set** is a **distributive lattice**.<br>

<br>

## Boolean lattice
A **Boolean lattice** (aka **Boolean algebra**) is is a *distributive lattice* $`(L, \lor ,\land )`$ in which exists **inverse element** ($`a^{-1}`$ or $`\neg a`$) for **each** element for **each** operation in $`L`$:
- $`a \land a^{-1} = 1 \space \space \forall a \in L`$;
- $`a \lor a^{-1} = 0 \space \space \forall a \in L`$;

<br>

The **inverse element** ($`a^{-1}`$) is the same as the **not operation** ($`\neg a`$).<br>

<br>

In the **Boolean lattice** the **not operation** ($`\neg`$) is an **involution**:
- $`\neg(\neg a) = a \space \space \forall a \in L`$

<br>

**De Morgan's laws** are also satisfied in the **Boolean lattice**:
- $`\neg (a \lor b) = (\neg a) \land (\neg b)`$;
- $`\neg (a \land b) = (\neg a) \lor (\neg b)`$;

<br>

## Ring of sets
A **ring of sets** is a **distributive lattice** in which the set's **intersection** and **union** operations correspond to the lattice's **meet** and **join** operations, respectively.<br>

<br>

## Field of sets
A **field of sets** is a **Boolean lattice** in which the set's **intersection** and **union** operations correspond to the lattice's **meet** and **join** operations, respectively.<br>

<br>

# The algebra of sets
Any set of sets closed under the set-theoretic operations forms a **Boolean algebra** with
- the **join** operator being **union**;
- the **meet** operator being **intersection**;
- the **complement** operator being **set complement** (aka **not**);
- the **bottom** being ⁠$`\varnothing`$⁠;
- the **top** being the **universe set** under consideration;

<br>

# Zero divisor
An element $`a`$ of a **ring** $`R`$ is called a **left zero divisor** if there exists a **nonzero** $`x \in R`$ such that $`a \cdot x = 0`$.<br>
An element $`a`$ of a **ring** $`R`$ is called a **right zero divisor** if there exists a **nonzero** $`y \in R`$ such that $`y \cdot a = 0`$.<br>

An element $`a`$ that is **both** a **left** and a **right** zero divisor is called a **zero divisor**.<br>
If the **ring** is **commutative**, then the **left** and **right** zero divisors are the **same**.<br>

A **zero divisor** that is **nonzero** is called a **nonzero zero divisor** or a **nontrivial zero divisor**.<br>

An element of a **ring** that is **not** a *left zero divisor* is called **left regular** or **left cancellable**.<br>
An element of a **ring** that is **not** a *right zero divisor* is called **right regular** or **right cancellable**.<br>
An element of a **ring** that is **left** and **right** cancellable, and is hence **not** a zero divisor, is called **regular** or **cancellable**, or a **non-zero-divisor**.<br>
