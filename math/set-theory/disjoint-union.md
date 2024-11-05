# Table of contents
- [Table of contents](#table-of-contents)
- [Disjoint union](#disjoint-union)
    - [Example](#example)

<br>

# Disjoint union
**Disjoint union** of the sets $`A`$ and $`B`$ is the **new set** formed from the elements of $`A`$ and $`B`$ **labelled** (**indexed**) with the name of the set from which they come.<br>
Denoted as $`A ⊔ B`$.<br>
In other words, an **element** belonging to both $`A`$ and $`B`$ appears **twice** in the **disjoint union**, with **two different labels**.<br>

### Example
Consider the sets:
- $`A_{0} = {5,6,7}`$;
- $`A_{1} = {5,6}`$;

<br>

It is possible to index the set elements according to set origin by forming the following sets:
- $`{\displaystyle A'_{0} = A_{0} \times \{0\} = \{(5,0), (6,0), (7,0)\}}`$;
- $`A'_{1} = A_{1} \times \{1\} = \{(5,1), (6,1)\}`$;

*Second element* in each pair matches the *subscript of the origin set* (for example, the $`0`$ in $`(5,0)`$) matches the subscript in $`A_{0}`$.<br>

The **disjoint union** $`A_{0} ⊔ A_{1}`$ can then be calculated as **union** $`A'_{0} ∪ A'_{1}`$:<br>
$`A_{0} ⊔ A_{1} = A'_{0} ∪ A'_{1} = {(5,0), (6,0), (7,0), (5,1), (6,1)}`$.<br>

So, for $`i≠j`$ the sets $`A'_{i}`$ and $`A'_{j}`$ are **disjoint** even if the sets $`A_{i}`$ and $`A_{j}`$ are **not**.<br>

**Indexed family**, is informally a collection of objects, each associated with an **index** from some **index set**.<br>

Let $`I`$ and $`X`$ be sets and $`𝑓`$ a function such that $`𝑓: I \longrightarrow X`$ where $`𝑖`$ is an element of $`I`$ and the **image** $`𝑓(𝑖)`$ is the element of $`X`$ indexed by $`𝑖 ∈ I`$.<br>
Then $`I`$ is called the **index set** of the family, and $`X`$ is called the **indexed set**.<br>

Let $`(A_{𝑖}: 𝑖 ∈ I)`$ is an **indexed family of sets** indexed by $`I`$. The **disjoint union** of this **family** is the set: $`⨆A_{𝑖} = ⋃\{(x,𝑖): x ∈ A_{𝑖}\}`$.<br>
The elements of the disjoint union are ordered pairs $`(x,𝑖)`$. Here $`𝑖`$ serves as an auxiliary index that indicates which $`A_{𝑖}`$ the element $`x`$ came from.<br>

In the extreme case where each of the $`A_{𝑖}`$ is equal to some fixed set $`A`$ for each $`𝑖 ∈ I`$, the **disjoint union** is the **cartesian product** of $`A`$ and $`I`$: $`⨆A_{𝑖} = A \times I`$.<br>
