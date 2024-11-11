# Table of contents
- [Table of contents](#table-of-contents)
- [Cosets](#cosets)
  - [Left coset](#left-coset)
  - [Right coset](#right-coset)
  - [Examples](#examples)
    - [Example1](#example1)
- [Equivalence classes](#equivalence-classes)
- [Normal subgroups](#normal-subgroups)
- [Index of a subgroup](#index-of-a-subgroup)
- [Lagrange's theorem](#lagranges-theorem)
- [Rus](#rus)

<br>

# Cosets
A **subgroup** $`H`$ of a **group** $`G`$ may be used to **decompose** the **underlying set** of $`G`$ into **disjoint**, **equal-size** *subsets* called **cosets**.<br>

There are **left cosets** and **righ cosetst**.<br>

The $`H`$ **itself** is **both** a **left coset** and a **right coset**.<br>

Every **left** or **right** coset of $`H`$ has the **same number** of elements as $`H`$ **has**:
- $`|gH| = |H|`$;
- $`|Hg| =|H|`$;

<br>

## Left coset
Let $`H`$ be a **subgroup** of the **group** $`G`$ whose operation is written **multiplicatively** ($`\cdot`$):
- **group** $`G = (S; \cdot)`$
- **subgroup** $`H = (T; \cdot)`$, where $`T⊆S`$;

<br>

Given an element $`g`$ of $`G`$, the **left coset** of $`H`$ in $`G`$ is the **set** $`gH`$ obtained by multiplying **each** element of $`H`$ by a **fixed element** $`g`$ of $`G`$ (where $`g`$ is the **left factor**):
- $`gH = \{g \cdot h \in S : h \in H\}`$ for **fixed** $`g`$ in $`G`$.<br>

<br>

In other words, **if** $`H = \{h_{1} = e,h_{2},...,h_{m}\}`$, **then**:
- **left coset**: $`gH = \{g \cdot h_{1},\space g \cdot h_{2},\space \dots,\space g \cdot h_{m}\}`$ for **fixed** $`g`$ in $`G`$.

<br>

## Right coset
Given an element $`g`$ of $`G`$, the **right coset** of $`H`$ in $`G`$ is the **set** $`Hg`$ obtained by multiplying **each** element of $`H`$ by a **fixed element** $`g`$ of $`G`$ (where $`g`$ is the **right factor**):
- $`Hg = \{h \cdot g \in S : h \in H\}`$ for **fixed** $`g`$ in $`G`$.<br>

<br>

In other words, **if** $`H = \{h_{1} = e,h_{2},...,h_{m}\}`$, **then**:
- **right coset**: $`Hg = \{h_{1} \cdot g,\space h_{2} \cdot g,\space \dots,\space h_{m} \cdot g\}`$ for **fixed** $`g`$ in $`G`$.

<br>

## Examples
### Example1
Find **left coset** of $`S_{3}`$.
$`S_{3}`$ has **subgroup** $`H= {e = (1)(2)(3),(123),(132)}`$.
Then:
- element $`e`$ generates **left coset** $`eH = H = {(1)(2)(3),(123),(132)}`$;
- element $`(1)(23)`$ generates **left coset** $`(1)(23)H= {(1)(23),(12)(3),(13)(2)}`$;

<br>

# Equivalence classes
**Defeinition 1**: two elements $`a \in G`$ and $`b \in G`$ are called **equivalent** ($`a \sim b`$) if there exists $`h \in H`$ such that $`a \cdot h = b`$.<br>

**Defeinition 2**: two elements $`a \in G`$ and $`b \in G`$ are called **equivalent** ($`a \sim b`$) **if and only if** ⁠$`a^{-1} \cdot b = h \And h \in H`$.<br>

We can get **defeinition 2** from **defeinition 1** and vice versa by multiplying by $`a^{-1}`$ or $`a`$, because $`G`$ is **group** and the **element** $`a^{-1}`$ **exists**:
- from **defeinition 1** we can get **defeinition 2**:
  - **multiply** $`a \cdot h = b`$ **by** $`a^{-1}`$:
    - then $`a^{-1}a \cdot h = a^{-1} \cdot b`$
    - then ⁠$`e \cdot h = a^{-1} \cdot b`$;
    - then $`h = a^{-1} \cdot b \implies a^{-1} \cdot b = h \And h \in H`$;
- from **defeinition 2** we can get **defeinition 1**:
  - **multiply** $`a^{-1} \cdot b = h`$ **by** $`a`$:
    - then ⁠$`a \cdot a^{-1} \cdot b = a \cdot h`$;
    - then ⁠$`e \cdot b = a \cdot h`$;
    - then ⁠$`b = a \cdot h \implies a \cdot h = b`$;

<br>

**Proof** that **equivalence** ($`a \sim b`$) defined in **defeinition 2** satisfies properties of **equivalence relation**.<br>

1. **Symmetry**.<br>

**Multiply** $`a^{-1} \cdot b = h`$ **by** $`a`$:
  - then ⁠$`a \cdot a^{-1} \cdot b = a \cdot h`$;
  - then ⁠$`e \cdot b = a \cdot h`$;
  - then ⁠$`b = a \cdot h`$;
  - then ⁠$`b^{-1} \cdot b = b^{-1} \cdot a \cdot h`$;
  - then ⁠$`e = b^{-1} \cdot a \cdot h`$;
  - then ⁠$`e \cdot h^{-1} = b^{-1} \cdot a \cdot (h \cdot h^{-1})`$;
  - then ⁠$`h^{-1} = b^{-1} \cdot a`$;
  - then ⁠$`b^{-1} \cdot a = h^{-1}`$;

We got ⁠$`b^{-1} \cdot a = h^{-1}`$ from $`a^{-1} \cdot b = h`$ where $`h^{-1} \in H \And h \in H`$.<br>
This means **symmetry**.<br>

2. **Transitivity**.<br>
Consider $`a^{-1} \cdot b = h_{1}`$ and $`b^{-1} \cdot c = h_{2}`$ where $`h_{1} \in H`$, and $`h_{2} \in H`$, and $`h_{1} \cdot h_{2} \in H`$.<br>
Then
- $`(a^{-1} \cdot b) \cdot (b^{-1} \cdot c) = h_{1} \cdot h_{2}`$;
- $`a^{-1} \cdot (b \cdot b^{-1}) \cdot c = h_{1} \cdot h_{2}`$;
- $`a^{-1} \cdot e \cdot c = h_{1} \cdot h_{2}`$;
- $`a^{-1} \cdot c = h_{1} \cdot h_{2}`$;

This means **transitivity**.

3. **Reflection**.<br>
⁠$`a^{-1} \cdot a = e`$. Because $`H`$ is a **group** ⁠$`e \in H`$.<br>

<br>

**Proof** that **equivalence** ($`a \sim b`$) defined in **defeinition 1** satisfies properties of **equivalence relation**.<br>

Consider group $`G = (S; \space \cdot)`$ and its subgroup $`H = (T \subset S;\space \cdot)`$.<br>
Consider the **binary relation** $`R`$ on the set $`S`$: $`aR_{H}b \space \space \text{where}\space a,b \in S \iff \exists h \in H : a \cdot h = b`$.

**Proof** that relation $`R`$ is **equivalence relation** on the set $`S`$ because it satisfies properties of **equivalence relation**:
1. **Reflection**: $`aR_{H}a`$, **indeed**, $`\forall a \in S \space \space \exists e \in H: a \cdot e = a`$, where $`e`$ is a **neutral element** from H.
2. **Symmetry**: $`aR_{H}b \implies bR_{H}a`$, **indeed**
   - $`\forall a,b \in S \space \space \exists h \in H: a \cdot h = b`$;
   - because $`H`$ is a **group**, it has $`h^{-1}`$, so $`(a \cdot h) \cdot h^{-1} = b \cdot h^{-1}`$;
   - then $`a = b \cdot h^{-1}`$ **or** $`b \cdot h^{-1} = a \implies bR_{H}a`$;
3. **Transitivity**: $`aR_{H}b \space \And \space bR_{H}c \implies aR_{H}c`$, **indeed**
   - $`\forall a,b \in S \space \space \exists h_{1} \in H: a \cdot h_{1} = b`$;
   - $`\forall b,c \in S \space \space \exists h_{2} \in H: b \cdot h_{2} = c`$;
   - then $`a \cdot h_{1} \cdot h_{2} = a \cdot (h_{1} \cdot h_{2}) = (a \cdot h_{1}) \cdot h_{2} = b \cdot h_{2} = c`$;
   - so, $`a \cdot (h_{1} \cdot h_{2}) = c \implies aR_{H}c`$

<br>

**All** equivalent elements ($`a \sim b`$) **form** some **equivalence relation** $`R`$ and $`R`$ **decomposes** the **underlying set** of $`G`$ into **disjoint equivalence classes**.

The **left cosets** are the **equivalence classes** corresponding to some **equivalence relation** $`R`$  ($`a \sim b`$).<br>

Therefore, the **left cosets** $`gH`$ **form** a **partition** of $`G`$.<br>

<br>

# Normal subgroups
If $`gH = Hg`$ for **every** $`g`$ in $`G`$, then $`H`$ is said to be a **normal subgroup**& in toher words, for **every** $`g`$ of $`G`$ the corresponding **left** and **right** *cosets* are **equal**.<br>

<br>

# Index of a subgroup
As $`g`$ **varies** through the group $`G`$, it would appear that **many** cosets (**right** or **left**) would be generated. Nevertheless, it turns out that **any** two **left** cosets (respectively **right** cosets) are either **disjoint** or are **identical** as sets.<br>

The **number** of **all** *left cosets* of $`H`$ in $`G`$ is **equal** to the number of **all** *right cosets* of $`H`$ in $`G`$ and this **number** is called the **index** of $`H`$ in $`G`$ and is usually denoted by $`[G : H]`$.<br>

The symbol $`G/H`$ denotes the **set** of **all** *left cosets* $`\{gH : g \in G\}`$ of $`H`$ in $`G`$.<br>

The symbol $`H \backslash G`$ denotes the **set** of **all** *right cosets* $`\{Hg : g \in G\}`$ of $`H`$ in $`G`$.<br>

So,
- $`|G/H| = |H \backslash G| = [G : H]`$

<br>

# Lagrange's theorem 
**Lagrange's theorem**: $`|G|=[G:H]|H|`$.<br>

**Lagrange's theorem** states that if $`H`$ is a **subgroup** of any **finite group** $`G`$, then $`|H|`$ is a **divisor** of $`|G|`$, i.e. the **order** (number of elements) of every **subgroup** $`H`$ **divides** the **order** of **group** $`G`$.<br>

<br>

# Rus
**coset** = **класс смежности**.<br>

Для элемента $`g \in G`$:
- **левый** смежный класс (или просто **класс смежности**) по подгруппе $`H`$ — множество $`gH = \{gh \mid h \in H\}`$;
- **правый** смежный класс по подгруппе $`H`$ — множество $`Hg=\{hg \mid h\in H\}`$;
