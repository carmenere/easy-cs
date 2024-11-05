# Table of contents
- [Table of contents](#table-of-contents)
- [Disjoint union](#disjoint-union)
    - [Example](#example)

<br>

# Disjoint union
**Disjoint union** of the sets `A` and `B` is the **new set** formed from the elements of `A` and `B` **labelled** (**indexed**) with the name of the set from which they come.<br>
Denoted as `A ⊔ B`.<br>
In other words, an **element** belonging to both `A` and `B` appears **twice** in the **disjoint union**, with **two different labels**.<br>

### Example
Consider the sets:
- `A₀ = {5,6,7}`;
- `A₁ = {5,6}`;

<br>

It is possible to index the set elements according to set origin by forming the following sets:
- `A'₀ = A₀ x {0} = {(5,0), (6,0), (7,0)}`;
- `A'₁ = A₁ x {1} = {(5,1), (6,1)}`;

*Second element* in each pair matches the *subscript of the origin set* (for example, the `0` in `(5,0)`) matches the subscript in `A₀`.<br>

The **disjoint union** `A₀ ⊔ A₁` can then be calculated as **union** `A'₀ ∪ A'₁`:<br>
`A₀ ⊔ A₁ = A'₀ ∪ A'₁ = {(5,0), (6,0), (7,0), (5,1), (6,1)}`.<br>

So, for `i≠j` the sets `A'i` and `A'j` are **disjoint** even if the sets `Ai` and `Aj` are **not**.<br>

**Indexed family**, is informally a collection of objects, each associated with an **index** from some **index set**.<br>

Let `I` and `X` be sets and `𝑓` a function such that `𝑓: I -> X` where `𝑖` is an element of `I` and the **image** `𝑓(𝑖)` is the element of `X` indexed by `𝑖 ∈ I`.<br>
Then `I` is called the **index set** of the family, and `X` is called the **indexed set**.<br>

Let `(A𝑖: 𝑖 ∈ I)` is an **indexed family of sets** indexed by `I`. The **disjoint union** of this **family** is the set: `⨆A𝑖 = ⋃{(x,𝑖): x ∈ A𝑖}`.<br>
The elements of the disjoint union are ordered pairs `(x,𝑖)`. Here `𝑖` serves as an auxiliary index that indicates which `A𝑖` the element `x` came from.<br>

In the extreme case where each of the `A𝑖` is equal to some fixed set `A` for each `𝑖 ∈ I`,  the **disjoint union** is the **cartesian product** of `A` and `I`: `⨆A𝑖 = A x I`.<br>
