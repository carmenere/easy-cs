# Table of contents
- [Table of contents](#table-of-contents)
- [Sets](#sets)
  - [Subset](#subset)
    - [Properties](#properties)
    - [Trivial subsets](#trivial-subsets)
  - [Disjoint sets](#disjoint-sets)
- [Multisets](#multisets)
- [Powerset](#powerset)
- [Set operations](#set-operations)
  - [Cartesian product](#cartesian-product)

<br>

# Sets
A **set**, *informally*, is a **collection** of **elements** (**objects**) that is **unordered** and has **no duplicate** elements.<br>
**Unorderedness**: `{1, 2, 3}` is the **same** as `{3, 2, 1}`, i.e. `{1, 2, 3} = {3, 2, 1}`.<br>
**Notation**:
- `∈` (**element of** symbol): if `a` is a **member** (or **element**) of `A` (`a` is **contained** in `A`, `a` **belongs** to `A`), it is written as `a ∈ A`;
- `{ }` (**curly braces**) to describe sets, there are 2 approaches to describe sets:
  - by **listing all elements** surrounded with `{ }`;
  - using **set-builder notation** (aka **set comprehension**) `{ x: P(x) }`, which means set of all `x` belonging to some set `X` such that predicate `P(x)` is `true`;

<br>

**Empty set** (aka **null set**) is the set with **no** elements. It is expressed as `{}` or `∅`.<br>
**Singleton** ia s set which has **only one** element, e.g. `{x}`.<br>

If `A` has exactly `n` distinct elements we write `|A| = n` and call `n` the **size** or **cardinality** of `A`.<br>

For example, `|{1,2}| = 2`.<br>

<br>

Let `A` and `B` be sets:<br>
- `x∈A` means `x` is an *element of* `A`;
- `x∉A` means `x` is **not** an *element of* `A`;

<br>

## Subset
- `A ⊆ B` means `A` is a **subset** of `B` or `A` is **contained in** `B`, in other words **every** element of `A` is **also** an element of `B`;
  - **formal defenition**: `A ⊆ B, if ∀ a∈A ⇒ b∈B`;
  - also `B` is called a **superset** of set `A` and denoted `B ⊇ A`;
- If `A ⊆ B and A ⊆ B` => `A = B`;
- `A ⊈ B` means `A` is **not** a **subset** of `B`.
- `A ⊊ B` means `A` is a **proper subset** of `B`, in other words `A` is a **subset** of `B` **but** `A ≠ B`;
  - **formal defenition**: `A ⊊ B, if A ⊆ B and A ≠ B`;
  - `B` is called a **proper superset** of set `A` and denoted `B ⊋ A`;

<br>

Some authors use the symbols `⊂` and `⊃` to indicate **subset** and **superset** respectively; that is, with the same meaning as and instead of the symbols `⊆` and `⊇`.<br>
Some authors use the symbols `⊂` and `⊃` to indicate **proper subset** and **proper superset** respectively; that is, with the same meaning as and instead of the symbols `⊊` and `⊋`.<br>

Example, consider set `A = {1,3,5}`, then 
- set `B = {1,5}` is a **proper subset** of `A`: `B ⊊ A`, also `B` is a **subset** of `A`: `B ⊆ A`;
- set `C = {1,3,5}` is a **subset** of `A`: `C ⊆ A`, but it is **not** a **proper subset** of `A` since `C = A`;
- set `D = {1,4}` is not even a subset of `A`, since 4 is **not** an **element** of `A`.

<br>

### Properties
- Each set **contains itself**, i.e. `A ⊆ A`.
- **Null set** `∅` is a **subset** of any set.
- If set `B` is a **subset** of set `A`, then `B ∩ A = B`.
- If set `B` is a **subset** of set `A`, then `B ∪ A = A`.

<br>

### Trivial subsets
**Trivial subsets** of set `A` are **empty set** `{}` and **set itself** `A`.<br>

<br>

## Disjoint sets
Two sets are said to be **disjoint sets** if they **have no element in common**.<br>
Equivalently, **two disjoint sets are sets whose intersection is the empty set**. Two sets `A` and `B` are called **disjoint** if `A ∩ B = ∅`.<br>
For example, `{1, 2, 3}` and `{4, 5, 6}` are **disjoint sets**, while `{1, 2, 3}` and `{3, 4, 5}` are **not disjoint**.<br>

<br>

# Multisets
A **multiset**, *informally*, is a **collection** of elements that is **unordered** and may have **duplicate** elements.<br>
The **number of occurrences** of a specific element is called its **multiplicity**.<br>
**Notation**: same as for sets.<br>
**Example**: `{3,5,5,7}`.<br>
**Unorderedness**: `{3,5,5,7}` is the same as `{3,5,7,5}`.<br>

<br>

# Powerset
The **power set** (or **powerset**) of a set `S` is the **set** of **all subsets** of `S`, including the **empty set** and `S` itself.<br>

The **power set** is denoted as `P(S)`.<br>

If `S` is the set `{x, y, z}`, then all the subsets of `S` are
- `{}`
- `{x}`
- `{y}`
- `{z}`
- `{x, y}`
- `{x, z}`
- `{y, z}`
- `{x, y, z}`

and hence the **power set** of `S` is `{{}, {x}, {y}, {z}, {x, y}, {x, z}, {y, z}, {x, y, z}}`.

<br>

**Cardinality** of a set is a measure of the number of elements of the set.<br>
If `S` is a **finite set** with the **cardinality** `|S| = n`, then the **cardinality** of its **powerset** is `|P(S)| = 2ⁿ`.

<br>

# Set operations
Suppose that a **universal set** `U` has been fixed, and that `A` and `B` are a subset of `U`.

<br>

|Operation|Symbol|Description|
|:--------|:-----|:----------|
|**Complement**|`∁` or `'`|The **complement** of `A` is the set of all elements of `U` that do **not** belong to `A`.|
|**Union**|`∪`|`A ∪ B = { x \| x∈A or x∈B }`<br>**Cardinality of union**: `\|A ∪ B\| = \|A\| + \|B\|`.|
|**Intersection**|`∩`|`A ∩ B = { x \| x∈A and x∈B }`|
|**Set difference**|`\`|`A \ B = { x \| x∈A and x∉B }`|
|**Symmetric difference**|`∆`|`A ∆ B = { x \| (x∈A and x∉B) or (x∈B and x∉A) }`<br>`A ∆ B = (A ∪ B) \ (A ∩ B)`|
|**Cartesian product**|`×`|`A × B = { ⟨a,b⟩ \| a∈A and b∈B }`|

<br>

## Cartesian product
**Cartesian product** of two sets `A` and `B`, denoted `A × B`, is the set of all **ordered pairs** `(a, b)` (**2-tuples**) where `a` is in `A` and `b` is in `B`.<br>
An **ordered pair** `⟨a,b⟩` has the property that `⟨a,b⟩ = ⟨c,d⟩` **if and only if** `a = c` and `b = d`.<br>

The **cardinality** of the **cartesian product** is equal to the product of the cardinalities of all the input sets: `|A × B| = |A| · |B|`.<br>
