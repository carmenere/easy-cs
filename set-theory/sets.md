# Set terms
**Empty set** (aka **null set**) is a set that doesn't contain any values or elements. It is expressed as `{}` or `∅`.<br>
**Singleton** ia s set which has only one element, e.g. `{x}`.<br>

A set `B` is the **subset** of another set `A`, if every element of set `B` is also an element of set `A`, denoted by `B ⊆ A`. Formal defenition: `B ⊆ A, if ∀ b∈B ⇒ b∈A`.<br>
`A` is called a **superset** of set `B` and denoted `A ⊇ B`.<br>

A set `B` is the **proper subset** of another set `A`, if `A` **not** equal to `B`, denoted by `B ⊊ A`. Formal defenition: `B ⊊ A, if B ⊆ A and B ≠ A`.<br>
if `B ⊆ A and A ≠ B`.<br>
`A` is called a **proper superset** of set `B` and denoted `A ⊋ B`.<br>

**Trivial subsets** of set `A` are **empty set** `{}` and set `A` itself.<br>

Some authors use the symbols `⊂` and `⊃` to indicate **subset** and **superset** respectively; that is, with the same meaning as and instead of the symbols `⊆` and `⊇`.<br>
Some authors use the symbols `⊂` and `⊃` to indicate **proper subset** and **proper superset** respectively; that is, with the same meaning as and instead of the symbols `⊊` and `⊋`.<br>

Example, consider set `A = {1,3,5}`, then 
- set `B = {1,5}` is a **proper subset** of `A`: `B ⊊ A`, also `B` is a **subset** of `A`: `B ⊆ A`;
- set `C = {1,3,5}` is a **subset** of `A`: `C ⊆ A`, but it is **not** a **proper subset** of `A` since `C = A`;
- set `D = {1,4}` is not even a subset of `A`, since 4 is **not** an **element** of `A`.

<br>

## Properties
- Each set **contains itself**, i.e. `A ⊆ A`.
- **Null set** `∅` is a **subset** of any set.
- If set `B` is a **subset** of set `A`, then `B ∩ A = B`.
- If set `B` is a **subset** of set `A`, then `B ∪ A = A`.
