# Binary relation
A **binary relation** `R` over sets `X` and `Y` is a **new set** of **ordered pairs** `(x, y)` consisting of elements `x` from `X` and `y` from `Y`.<br>
So, **binary relation** associates elements of one set with elements of another set.<br>

<br>

A **binary relation** `R` over sets `X` and `Y` is a subset of **cartesian product** `X×Y`:
- the set `X` is called the **domain** or **set of departure** of `R`;
- the set `Y` the **codomain** or **set of destination** of `R`;

<br>

In a binary relation, the **order of the elements** is **important**: IF `x≠y` then `yRx` can be `true` or `false` **independently** of `xRy`.<br>
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

# Equivalence relations
## Partial equivalence relation
**Partial equivalence relation** (**PER**) is a binary relation that satisfies the following properties:
1. **Symmetry**: IF `a=b` THEN `b=a`.
2. **Transitivity**: IF `a=b` AND `b=c`, THEN `a=c`.

> **Note**:<br>
> **Partial equivalence relation**  is **not reflexive**, i.e., `a != a`.<br>

<br>

## Equivalence relation
**Equivalence relation** is a binary relation that satisfies the following properties:
1. **Reflexivity**: Any number a is equal to itself.
2. **Symmetry**: IF `a=b` THEN `b=a`.
3. **Transitivity**: IF `a=b` AND `b=c`, THEN `a=c`.

<br>

# Order relations
There are 2 type of order:
1. **Partial order**. The word **partial** is used to indicate that **not every pair** of elements are **comparable**; that is, there may be pairs for which neither element precedes the other.
2. **Total order**. In **total order** every pair is **comparable**.

<br>

## Weak partial order or just Partial order (≤)
**Partial order** (aka **non-strict/weak partial order**) is a binary relation that satisfies the following properties:
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

## Weak total order or just Total order (≤)
**Total order** (aka **non-strict/weak total order**) is the **weak partial order** with the added property of **strong connectivity**:
1. **Antisymmetry**.
2. **Reflexivity**.
3. **Transitivity**.
4. **Strong connectivity**.

<br>

## Strict total order (<)
**Strict total order** is the **strict partial order** with the added property of **weak connectivity**:
1. **Asymmetry**.
2. **Irreflexivity**.
3. **Transitivity**.
4. **Weak connectivity**.