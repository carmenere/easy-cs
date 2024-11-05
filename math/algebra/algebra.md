# Abstract algebra
**Abstract algebra** studies **algebraic structures**.<br>

An **algebraic structure** consists of
- a **nonempty set** `S` (aka **underlying set**, **carrier set** or **domain**);
- a **collection of operations** on `S`, every operation must be **closed**;
- a **set of axioms**, that these operations must satisfy.

Operation `•` is **closed** in `S` means that for all `x`, `y` in `S`, the **result** of the **operation** `x • y` is also in `S`.

<br>

## Common axioms
1. **Commutativity**: an operation `*` is **commutative** if `x*y = y*x` for every `x` and `y` in the algebraic structure.
2. **Associativity**: an operation is **associative** if `(x*y)*z = x*(y*z)` for every `x`, `y` and `z` in the algebraic structure.
3. **Left distributivity**: an operation `*` is **left distributive** with respect to another operation `+` if `x*(y+z) = x*y + x*z` for every `x`, `y` and `z` in the algebraic structure.
4. **Right distributivity**: an operation `*` is **right distributive** with respect to another operation `+` if `(y+z)*x = y*x + z*x` for every `x`, `y` and `z` in the algebraic structure..
5. **Distributivity**: an operation `*` is **distributive** with respect to another operation `+` if it is **both** *left distributive* and *right distributive*.

<br>

## Identity element
**Identity element** (aka **neutral element**) of a binary operation is an element that leaves unchanged every element when the operation is applied. For example, `0` is an **identity element** of the **addition** of *real numbers*.

Let `(S, *)` be a set `S` with a binary operation `*`.<br>
An element `e` of `S` is called a **left identity** if `e * s = s` for all `s` in `S`.<br>
An element `e` of `S` is called a **right identity** if `s * e = s` for all `s` in `S`.<br>

If `e` is **both** a *left identity* and a *right identity*, then it is called a **two-sided identity**, or simply an **identity**.<br>

The **identity elements** for **addition** and **multiplication** are denoted `0` and `1`, respectively.

<br>

## Inverse element
Let `(S, *)` be a set `S` with a binary operation `*` and **indentity** `e`.<br>
An element `l` of `S` is called a **left inverse** of `y` if `l * y = e`.<br>
An element `r` of `S` is called a **right inverse** of `y` if `y * r = e`.<br>

If **left inverse** of `y` and **right inverse** of `y` are equal and unique then they are called the **inverse element** or simply the **inverse**.<br>
An **invertible element** is an element that has an **inverse**.<br>

<br>

# Algebraic structures with one binary operation
## Magma
**Magma** (rarely, **groupoid**) is a basic algebraic structure. Magma **doesn't has axioms**, but it **closed** by defenition:
- for all `x`, `y` in `M`, the **result** of the **operation** `x • y` is also in `M`.

<br>

## Semigroup
**Semigroup** is a *magma* with **associative** operation.

<br>

## Monoid
**Monoid** is a *semigroup* with **identity**.

<br>

## Group
**Group** is a *monoid* in which **every element** has an **inverse** element.

<br>

## Abelian group
**Abelian group** is a *group* with **commutative** operation.

<br>

# Algebraic structures with two binary operations
## Semiring
A **semiring** is an algebraic structure with **two** operations: **addition** `+` and **multiplication** `*`.<br>

**Semiring axioms**:
- *ring* is an **commutative monoid** under `+`;
- *ring* is a **monoid** under `*`;
- `*` is **distributive** with respect to `+`;

<br>

## Ring
A **ring** is an algebraic structure with **two** operations: **addition** `+` and **multiplication** `*`.<br>

**Ring axioms**:
- *ring* is an **abelian group** under `+`;
- *ring* is a **monoid** under `*`;
- `*` is **distributive** with respect to `+`;

<br>

## Commutative ring
**Commutative ring** is a **ring** in which the **multiplication** operation is **commutative**.

<br>

## Field
A **field** is an algebraic structure with **two** operations: **addition** `+` and **multiplication** `*`.<br>

**Field axioms**:
- *field* is an **abelian group** under `+`;
- *field* is a **abelian group** under `*`;
- `*` is **distributive** with respect to `+`;
