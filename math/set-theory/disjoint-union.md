# Table of contents
- [Table of contents](#table-of-contents)
- [Family](#family)
  - [Rus](#rus)
- [Family of sets](#family-of-sets)
  - [Rus](#rus-1)
  - [Class](#class)
- [Disjoint union](#disjoint-union)
    - [Example](#example)

<br>

# Family
A **family** (aka **indexed family**) is a **collection** of **elements** from some **set** $`X`$, each **associated** with an **index** from some **index set** $`I`$.<br>

Let $`I`$ and $`X`$ be **sets** and $`f`$ a **function** such that $`f: I \longrightarrow X`$ where $`i \in I`$ and the **image** $`f(i) \in X`$ of $`i`$ is **denoted** by $`x_{i}`$. For example, $`f(3)`$ is **denoted** by $`x_{3}`$.<br>

The **symbol** $`x_{i}`$ is used to indicate that $`x_{i}`$ is the **element** of $`X`$ **indexed** by $`i\in I`$.<br>
The **function** $`f`$ thus establishes a **family** of elements in $`X`$ **indexed** by $`I`$ which is **denoted** by $`\{x_{i}\}_{i\in I}`$.<br>

Then $`I`$ is called the **index set** of the **family**, and $`X`$ is called the **indexed set**.<br>

<br>

## Rus
**Семейство** или **индексированное семейство** — некоторая **совокупность объектов**, каждый из которых ассоциирован с **индексом** из некоторого **индексного множества**.<br>
**Cемейство** элементов из множества $`X`$ индексированное множеством $`I`$ обозначается так: $`\{x_{i}\}_{i\in I}`$.<br>

<br>

# Family of sets
A **collection** $`F`$ of **subsets** of a given **set** $`S`$ is called a **family of subsets of** $`S`$ or a **family of sets over** $`S`$.<br>

A **family of sets** may be defined as a **function** from a set $`I`$, known as the **index set**, to $`F`$, in which case the **elements** of the **family** are **indexed** by members of $`I`$.<br>

The set of **all** subsets of a given set $`S`$ is called the **power set of** $`S`$ and is denoted by $`\wp (S)`$.<br>
The **power set** $`\wp (S)`$ of a given set $`S`$ is a **family of sets over** $`S`$.<br>

<br>

## Rus
**Система множеств** - множество, все **элементы** которого также являются **множествами**, например, булеан.<br>
**Семейство множеств** - **индексированный** аналог *системы множеств*.<br>

<br>

## Class
A **class** is a **collection of sets** (or sometimes other **mathematical objects**) that can be **unambiguously** defined by a **property** that **all** its members **share**.<br>
**Classes** act as a way to have **set-like collections** while **differing** from **sets** so as to **avoid paradoxes**, especially *Russell's paradox*.<br>

A **class** that is **not** a **set** is called a **proper class**.<br>

**Zermelo–Fraenkel set theory** is an axiomatic system that was proposed in order to formulate a theory of sets **free of paradoxes** such as *Russell's paradox*. The **axiom of regularity** together with the **axiom of pairing** implies that **no set** is an **element** of **itself**.<br>

<br>

# Disjoint union
**Disjoint union** of the sets $`A`$ and $`B`$ is the **new set** formed from the elements of $`A`$ and $`B`$ **labelled** (**indexed**) with the name of the set from which they come.<br>
Denoted as $`A ⊔ B`$.<br>
In other words, an **element** belonging to both $`A`$ and $`B`$ appears **twice** in the **disjoint union**, with **two different labels**.<br>

### Example
Consider the sets:
- $`A_{0} = \{5,6,7\}`$;
- $`A_{1} = \{5,6\}`$;

<br>

It is possible to **index** sets $`A_{0}`$ and $`A_{1}`$ by forming the following sets:
- $`A'_{0} = A_{0} \times \{0\} = \{(5,0), (6,0), (7,0)\}`$;
- $`A'_{1} = A_{1} \times \{1\} = \{(5,1), (6,1)\}`$;

**Second** element in each **pair** matches the *subscript of the original set*, for example, the $`0`$ in $`(5,0)`$ matches the *subscript* $`_{0}`$ in $`A_{0}`$.<br>

The **disjoint union** $`A_{0} ⊔ A_{1}`$ can then be calculated as **union** $`A'_{0} ∪ A'_{1}`$:<br>
$`A_{0} ⊔ A_{1} = A'_{0} ∪ A'_{1} = \{(5,0), (6,0), (7,0), (5,1), (6,1)\}`$.<br>

So, for $`i≠j`$ the sets $`A'_{i}`$ and $`A'_{j}`$ are **disjoint** even if the sets $`A_{i}`$ and $`A_{j}`$ are **not**.<br>

<br>

$`\{A_{i}\}_{i\in I}`$

Let $`\{A_{i}\}_{i\in I}`$ is an **indexed family of sets** indexed by $`I`$. The **disjoint union** of this **family** is the **set**: $`⨆A_{𝑖} = ⋃\{(x,𝑖): x ∈ A_{𝑖}\}`$.<br>

The elements of the **disjoint union** are **ordered pairs** $`(x,𝑖)`$. Here $`𝑖`$ serves as an auxiliary **index** that indicates from which $`A_{𝑖}`$ the element $`x`$ came from.<br>

In the extreme case where each of the $`A_{𝑖}`$ is equal to some fixed set $`A`$ for each $`𝑖 ∈ I`$, the **disjoint union** is the **cartesian product** of $`A`$ and $`I`$: $`⨆A_{𝑖} = A \times I`$.<br>
