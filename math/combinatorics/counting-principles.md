# Table of contents
- [Table of contents](#table-of-contents)
- [Counting principles](#counting-principles)
- [In the set theory](#in-the-set-theory)
  - [AP](#ap)
  - [MP](#mp)

<br>

# Counting principles
- **The addition principle** (aka **AP**, **the rule of sum**):
  - If some event $A$ has $m$ possible outcomes and **another independent** event $B$ has $k$ possible outcomes, then there are $m + n$ possible outcomes of exactly **one** event $A$ or event $B$.
- **The multiplication principle** (aka **the fundamental counting principle**, **MP**, **the rule of product**):
  - If there are $m$ **choices** to select an element of type $A$ and there are $n$ **choices** to select **another independent** element of type $B$, then there are $m \times n$ **possible choices** to select **two** elements $A$ and $B$ **together**.
  - If some event $A$ has $m$ possible outcomes and **another independent** event $B$ has $n$ possible outcomes, then there are $m \times n$ possible outcomes of **two** events $A$ and $B$ **together**.
    - **MP** only **works** when the **choice** of element $A$ is **independent** of **choice** of another element $B$. If one element **depends** on another, then **MP doesn't** work.

<br>

**AP** and **MP** can be used **separately** to solve **simple** counting problems, but solving **more complicated problem** may require **both AP** and **MP**.

<br>

# In the set theory
## AP
- Consider **one** finite set $X$: $`X = \{a, b, c\}`$.
  - Then there are $|X|$ **ways** to select **exactly one** element from $X$.
- Consider **two** finite **disjoint** sets $A$ and $B$, $A ∩ B = ∅$.
  - Then there are $|A| + |B|$ **ways** to select **exactly one** element from the $A ∪ B$ (**union** of $A$ and $B$).
- Consider $k$ finite sets $A1$, $A2$, ..., $An$, where $k >= 1$ and **all** given sets are **pairwise disjoint**, i.e. $`A_i ∩ A_j = ∅`$ where $1 <= i,j <= k$ and $i ≠ j$.
  - Then there are $`|A_1| + |A_2| + ... + |A_n|`$ **ways** to select **exactly one** element from the $`A_1 ∪ A_2 ∪ ... ∪ A_n`$ (**union** of $`A_1`$, $`A_2`$, ..., $`A_n`$).

<br>

## MP
- Consider **two** finite **disjoint** sets $A$ and $B$, $A ∩ B = ∅$.
  - Then there are $|A| * |B|$ **ways** to select **exactly one ordered pair** from the $A \times B$ (**cartesian product** of $A$ and $B$).
- Consider $k$ finite sets $`A_1`$, $`A_2`$, ..., $`A_k`$, where $k >= 1$ and **all** given sets are **pairwise disjoint**, i.e. $`A_i ∩ A_j = ∅`$ where $1 <= i,j <= k$ and $i ≠ j$.
  - Then there are $`|A_1| * |A_2| * ... * |A_k|`$ **ways** to select **exactly one k-tuple** $`(a_1, a_2, ..., a_k)`$ from the set $`A_1 \times A_2 \times ... \times A_k`$, where
    - set $`A_1 \times A_2 \times ... \times A_k`$ is a **cartesian product** of $`A_1`$, $`A_2`$, ..., $`A_k`$;
    - elements in **k-tuple** $`(a_1, a_2, ..., a_k)`$ such that $`a_1∈A_1`$, $`a_2∈A_2`$, ..., $`a_k∈A_k`$;

<br>

The **Cartesian product** is the basis for the **MP**.<br>
To choose one element of $`A = \{a,b,c\}`$ **and** after such choose to choose one element of $`B = \{1,2\}`$ is the **same as** to choose one of $`R = \{(a,1), (a,2), (b,1), (b,2), (c,1), (c,2)\}`$. The sets $A$ and $B$ in this example are **disjoint** sets, but that is not necessary.<br>

The number of ways to choose **ordered pair with repetitions** from $`\{a,b,c\}`$ is the same as number of ways to choose element from $`\{a,b,c\} \times \{a,b,c\} = \{(a,a), (a,b), (a,c), (b,a), (b,b),(b,c),(c,a),(c,b),(c,c)\}`$.
