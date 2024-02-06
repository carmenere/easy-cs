# Binary relation
A **binary relation** `R` over sets `X` and `Y` is a **new set** of **ordered pairs** `(x, y)` consisting of elements `x` from `X` and `y` from `Y`.<br>
So, **binary relation** associates elements of one set with elements of another set.<br>

<br>

A **binary relation** `R` over sets `X` and `Y` is a subset of **cartesian product** `X×Y`:
- the set `X` is called the **domain** or **set of departure** of `R`;
- the set `Y` the **codomain** or **set of destination** of `R`;

<br>

In a binary relation, the **order of the elements** is **important**: if `x≠y` then `yRx` can be `true` or `false` **independently** of `xRy`.<br>
For example, `3 divides 9` => `true`, but `9 divides 3` => `false`.<br>

<br>

Examples of relations:
- **is greater than**;
- **is equal to**;
- **divides**;

<br>

# Homogeneous relation
A **homogeneous relation** over a set `X` is a *binary relation* over `X` and itself (`X`), i.e. it is a subset of the `X×X`. It is also simply called a (**binary**) **relation over** `X`.

<br>

## Properties of homogeneous relations
1. **Reflexivity**: for all `x ∈ X` => `xRx`.
   - For example, `≥` is a **reflexive** relation but `>` is **not**.
2. **Irreflexivity** (aka **Anti-reflexivity**): for all `x ∈ X` => **not** `xRx`.
   - For example, `>` is an **irreflexive** relation, but `≥` is **not**.
3. **Symmetry**: for all `a,b ∈ X` => if `aRb` then `bRa`.
   - For example, **blood relation** is a **symmetric** relation.
4. **Antisymmetry**: for all `a,b ∈ X` => if `aRb` and `bRa` then `a=b`.
   - For example, `≥` is an **antisymmetric** relation.
5. **Asymmetry**: for all `a,b ∈ X` => if `aRb` then **not** `bRa`.
   - A relation is **asymmetric** if and only if it is both **antisymmetric** and **irreflexive**.
     - For example, `>` is an **asymmetric** relation, but `≥` is **not**.
6. **Transitivity**: for all `a,b ∈ X` => if `aRb` and `bRc` then `aRc`.
    - A **transitive** relation is **irreflexive** if and only if it is **asymmetric**.
      - For example, **is ancestor of** is a **transitive** relation, while **is parent of** is **not**.
7. **Connectivity** (aka **Totality**): for all `a,b ∈ X` => if `a≠b` then `aRb` or `bRa`.
8. **Strong connectivity**: for all `a,b ∈ X` => `aRb` or `bRa`.

<br>

# Equivalence relations
## Partial equivalence relation
**Partial equivalence relation** (**PER**) is a binary relation that satisfies the following properties:
1. **Symmetry**: if `a=b` then `b=a`.
2. **Transitivity**: if `a=b` and `b=c`, then `a=c`.

> **Note**:<br>
> **Partial equivalence relation**  is **not reflexive**, i.e., `a != a`.<br>

<br>

## Equivalence relation
**Equivalence relation** is a binary relation that satisfies the following properties:
1. **Reflexivity**: Any number a is equal to itself.
2. **Symmetry**: If `a=b` then `b=a`.
3. **Transitivity**: If `a=b` and `b=c`, then `a=c`.

<br>

# Order relations
There are 2 type of order:
1. **Partial order**. The word **partial** is used to indicate that **not every pair** of elements are **comparable**; that is, there may be pairs for which neither element precedes the other.
2. **Total order**. In **total order** every pair is **comparable**.

<br>

## Weak partial order (≤)
**Weak partial order** (aka **non-strict partial order**) is a binary relation that satisfies the following properties:
1. **Antisymmetry**.
2. **Reflexivity**.
3. **Transitivity**.

<br>

## Strict partial order (<)
**Strict partial order** is a binary relation that satisfies the following properties:
1. **Asymmetry**.
2. **Irreflexivity**.
3. **Transitivity**.

<br>

## Weak total order (≤)
**Weak total order** (aka **non-strict total order**) is a binary relation that satisfies the following properties:
1. **Totality**.
2. **Reflexivity**
3. **Transitivity**.

<br>

## Strict total order (<)
**Strict total order** is a binary relation that satisfies the following properties:
1. **Totality**.
2. **Irreflexivity**.
3. **Transitivity**.
