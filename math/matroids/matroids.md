# Table of contents
- [Table of contents](#table-of-contents)
- [Linear combinations](#linear-combinations)
  - [Linearly dependent vectors](#linearly-dependent-vectors)
  - [Linearly independent vectors](#linearly-independent-vectors)
  - [Basis](#basis)
- [Matroids](#matroids)
  - [Definition in terms of independent sets](#definition-in-terms-of-independent-sets)
  - [Definition in terms of circuits](#definition-in-terms-of-circuits)
  - [Definition in terms of bases](#definition-in-terms-of-bases)
- [Matroid types](#matroid-types)
  - [Free matroid](#free-matroid)
  - [Uniform matroid](#uniform-matroid)
  - [Graphic matroid](#graphic-matroid)
- [Rank](#rank)
- [Non-isomorphic matroids](#non-isomorphic-matroids)

<br>

# Linear combinations
If $`{\displaystyle \mathbf {v} _{1},\mathbf {v} _{2},\dots ,\mathbf {v} _{k}}`$ are **vectors** and $`a_1`$ ,..., $`a_n`$ are **scalars**, then the **linear combination** of those **vectors** is $`{\displaystyle a_{1}\mathbf {v} _{1}+a_{2}\mathbf {v} _{2}+a_{3}\mathbf {v} _{3}+\cdots +a_{n}\mathbf {v} _{n}}`$.<br>

<br>

## Linearly dependent vectors
A sequence of vectors $`{\displaystyle \mathbf {v} _{1},\mathbf {v} _{2},\dots ,\mathbf {v} _{k}}`$ is said to be **linearly dependent**, if there exist scalars $`{\displaystyle a_{1},a_{2},\dots ,a_{k}}`$, not all zero, such that $`{\displaystyle a_{1}\mathbf {v} _{1}+a_{2}\mathbf {v} _{2}+\cdots +a_{k}\mathbf {v} _{k}=\mathbf {0} }`$, where $`{\displaystyle \mathbf {0}}`$ denotes the **zero vector**.<br>

This implies that at least one of the scalars is nonzero, say $`{\displaystyle a_{1}\neq 0}`$, and the above equation is able to be written as 
$`{\displaystyle \mathbf {v} _{1}={\frac {-a_{2}}{a_{1}}}\mathbf {v} _{2}+\cdots +{\frac {-a_{k}}{a_{1}}}\mathbf {v} _{k}}`$.<br>

In other words, **any vector** in the sequence of vectors **can be represented** as a **linear combination** of the **remaining vectors** in the sequence.<br>

<br>

## Linearly independent vectors
A sequence of vectors $`{\displaystyle \mathbf {v} _{1},\mathbf {v} _{2},\dots ,\mathbf {v} _{n}}`$ is said to be **linearly independent** if it is **not** *linearly dependent*, that is, if the equation $`{\displaystyle a_{1}\mathbf {v} _{1}+a_{2}\mathbf {v} _{2}+\cdots +a_{n}\mathbf {v} _{n}=\mathbf {0}}`$
can only be satisfied by $`{\displaystyle a_{i}=0}$ for ${\displaystyle i=1,\dots ,n}`$.<br>

In other words, **no vector** in the sequence of vectors **can be represented** as a **linear combination** of the **remaining vectors** in the sequence.<br>

<br>

## Basis
A *set of vectors* which is **linearly independent** forms a **basis** for that **vector space**.<br>

A set $B$ is a **basis** if its elements are **linearly independent** and **every other** vector in vector space is a **linear combination** of elements of $B$.<br>

A **vector space** can have **several** *bases*. However **all** the *bases* have the **same number of elements**, called the **dimension** of the *vector space*.<br>

<br>

# Matroids
A **matroid** /ˈmeɪtrɔɪd/ is a structure that **abstracts** and **generalizes** the notion of **linear independence** in **vector spaces** and **graph theory**.<br>
There are several equivalent definitions of matroids:
- in terms of **independent sets**;
- in terms of **circuits**;
- in terms of **bases**;

The **independent sets**, the **bases**, or the **circuits** of a matroid characterize the matroid completely:
- a *set is independent* **iff** it is **not dependent**;
- a *set is independent* **iff** it is a **subset** of a **basis**;
- a *set is independent* **iff** it does **not contain** a **circuit**;

<br>

The **collections** of independent sets, of bases, and of circuits **each** have **some properties** that may be taken as **axioms** for a matroid.<br>

<br>

## Definition in terms of independent sets
A matroid $M$ is a pair $(E,I)$, where:
- $E$ is a **finite set**, $E$ is called **ground set** of the **matroid**;
- $I$ is a **collection of subsets** of $E$, **elements** of $I$ are called **independent sets**;
- **subsets** of $E$ that are **not** in $I$ (i.e. **not** *independent*) are called **dependent**;

<br>

Set $I$ **must** satisfy the following properties:
1. The **empty set** $∅$ is **independent**, so $I$ is **always non-empty**:
   - $∅ ∈ I$;
2. **Any subset** of an **independent set** is **independent**, in other words, **every subset** of **every element** of $I$ is also in $I$:
   - if $A ∈ I$ and $B ⊂ A$, then $B ∈ I$;
3. If you have **two independent sets** $A$ and $B$ with **different** sizes ($|A| > |B|$), you can take an element from the **bigger** one $A$ and add it to the **smaller** one $B$ to get a **larger independent set**:
   - if $A,B ∈ I$ and if $|A| = |B| + 1$, then $∃ x∈A \setminus B$ such that $B ∪ {x} ∈ I$;

The **1st** and **2nd** properties define a structure known as an **independence system**.<br> 
The **3rd** property is sometimes called the **augmentation property** or the **exchange property**.<br>

<br>

If we want to **prove** that *something is a matroid* we have to **check all** these properties.

<br>

## Definition in terms of circuits
A **circuit** of some matroid $(E,I)$ is any **minimal dependent subset** of $E$. In other words, a **circuit** is a **dependent set** whose **proper subsets** are **all independent**.<br>

A **matroid** $M$ is a pair $(E,\mathcal {C})$ where:
- $E$ is a **finite set**, $E$ is called **ground set** of the **matroid**;
- $\mathcal {C}$ is a **collection of all circuits** of $E$ called **circuits**, each **element** of $\mathcal {C}$ is called **circuit**;

<br>

Set $\mathcal {C}$ **must** satisfy the following properties:
1. If $`C_1`$ and $`C_2`$ are **distinct** members of ${\mathcal {C}}$ and $`e ∈ C_1 ∩ C_2$, then $∃ C'∈\mathcal {C}`$ such that $`C'=(C_1 ∪ C_2 ) \setminus {e}`$;

<br>

## Definition in terms of bases
A **basis of a matroid** is a **maximal independent set** of the matroid. In other words, the **basis** of a matroid is an *independent set* that is **not** contained in **any other** *independent set*. A **basis** becomes **dependent** set by adding to it **any** element of $E$.<br>

A **matroid** $M$ is a pair $(E,{\mathcal {B}})$ where:
- $E$ is a **finite set**, $E$ is called **ground set** of the **matroid**;
- ${\mathcal {B}}$ is a **collection of all bases** of $E$ called **bases**, each **element** of ${\mathcal {B}}$ is called **basis**;

<br>

Set ${\mathcal {B}}$ **must** satisfy the following properties:
1. ${\mathcal {B}}$ is **always non-empty**.
2. If $A$ and $B$ are **distinct** members of ${\mathcal {B}}$ and ${\displaystyle a\in A \setminus B}$, then there exists an element ${\displaystyle b\in B\setminus A}$ such that  ${\displaystyle (A\setminus \{a\})\cup \{b\}\in {\mathcal {B}}}$.

<br>

Notes:
- **no element** of ${\mathcal {B}}$ can be a **proper subset of another**;
- **any** two bases of a matroid $M$ have the **same cardinality**, i.e. **all** elements of ${\mathcal {B}}$ of a given matroid have the **same cardinality**;

<br>

The basis has a specialized name in several specialized kinds of matroids:
- in a **graphic matroid** the **basis** is called the **spanning forest of the graph** $G$, so, the **bases** of **graphic matroid** is the set ${\mathcal {B}}$ of **all spanning forests of the graph** $G$:
  - **spanning trees** of the graph $G=(V,E)$ are a **subset** of $E$, such that **all** the **vertices** are **covered**, but there are **no cycles**;
  - it is obvious that the **spanning trees** are **maximally independent**, as if you add another edge, there will be a cycle;
- in a **linear matroid** the **basis** is called **basis of the vector space**, the **cardinality** of **all bases** is called the **dimension** of the **vector space**.
- in a **uniform matroid** the set ${\mathcal {B}}$ is the **set of all subsets of $E$ with cardinality exactly k**;
- in a **free matroid** there is the **only one basis** and it is **ground set itself**, $E$;

<br>

# Matroid types
- **free matroid**;
- **uniform matroid**;
- **graphic matroid**;
- **linear matroid**;

<br>

## Free matroid
The **free matroid** $M = (E,I)$ is the **matroid** in which **all subsets** of the *ground set* $E$ are **independent**, in other words its set $I$ is the **powerset** of $E$: $`I = 2^E`$.<br>

For example, if $`E = \{1, 2, 3\}`$, then $`I = \{∅, \{1\}, \{2\}, \{3\}, \{1, 2\}, \{1, 3\}, \{2, 3\}, \{1, 2, 3\}\}`$.<br>

<br>

## Uniform matroid
The **uniform matroid** $`U_{k,n} = (E,I)`$ is the **matroid** in which *independent sets* are **subsets** of the *ground set* $E$ with *cardinality* **no more than** $k$, i.e. $cardinality\in[0,k]$. The $n$ refers to the **cardinality of** $E$.<br>

For example, let $`E = \{1, 2, 3, 4\}`$ and let $`I = \{∅, \{1\}, \{2\}, \{3\}, \{4\}, \{12\}, \{13\}, \{14\}, \{23\}, \{24\}, \{34\}\}`$. It is the **uniform matroid** $`U_{2,4}`$. Let's check that it is a matroid:
1. The empty set $∅$ is in $I$, so property 1 is satisfied.
2. The only subset of the empty set $∅$ is itself and it is in $I$.
3. The only subsets of $`\{1\}`$ are $∅$ and $`\{1\}`$. Both of these are in $I$. Check the rest in the same way.
4. The only subsets of $`\{1, 2\}`$ are $∅$, $`\{1\}`$, $`\{2\}`$, and $`\{1, 2\}`$. Check the rest in the same way.
5. Take $A = ∅$ and $`B = \{1\}`$. We see that $1 = |B| > |A| = 0$, and we can add $1$ to $∅$ to get a larger independent set. 
6. Take $A = ∅$ and $`B = \{1, 2\}`$. Then we can add either $1$ or $2$ to $∅$. Check the rest the same way.
7. Take $`A = \{1\}`$ and $`B = \{1, 4\}`$. We can add $4$ to $A$. Take $`A = \{1\}`$ and $`B = \{2, 3\}`$. We can add either $2$ or $3$ to $A$. Check the rest the same way.

<br>

## Graphic matroid
The **graphic matroid** $`M(G) = (E,\mathcal {F})`$ is the **matroid** of given undirected graph $G=(V,E)$.<br>
The **ground set** $E$ of **graphic matroid** $M(G)$ is **equal to** the set $E$ of graph $G=(V,E)$.<br>
The **independent sets** $\mathcal {F}$ of **graphic matroid** is the **collection** of **all forests** in $G$, i.e. **all acyclic subgraphs** of $G$.<br>
The **forest** $F$ of given graph $G=(V,E)$ is **any subset** of $E$ **without** cycles. So, $F∈\mathcal {F}$.<br>
The **set of all circuits** $\mathcal {C}$ of **graphic matroid** is the **collection** of **all cycles** in $G$.<br>

<br>

Consider the graph G:<br>
![matroid-1](/img/matroid-1.png)

The **ground set** of **graphic matroid** corresponding to given graph $G$ is $`E = \{e1, e2, e3, e4, e5\}`$.<br>

There are **3 cycles** in $G$, so the are 3 **circuits** in matroid: $`\mathcal {C} = \{\{e1,e2,e3\}, \{e3,e5,e4\}, \{e1,e2,e5,e4\}\}`$. Note that this follows the **axioms of cycles**. For example, $e3$ is common to both $`\{e1,e2,e3\}`$ and $`\{e3,e5,e4\}`$, it means that $`\{e1,e2,e3\} ∪ \{e3,e5,e4\} \setminus {e3}`$ gives **another circuit** $`\{e1, e5, e4, e2\}`$.<br>

The **independent sets** $\mathcal {F}$ of **graphic matroid** corresponding to given graph $G$ is the collection of **all forests** in $G$: $`\mathcal {F} = \{∅, \{e1\}, \{e2\}, ..., \{e1, e2, e5\}...\}`$.

<br>

# Rank
The **rank of a matroid** $M$ is the **size** of **any** its **basis**.<br>
Consider **uniform matroid** $`U_{r,n}`$. This matroid has **rank** $r$ and is called the **uniform matroid of rank** $r$ on an $n$**-element set**.<br>

<br>

# Non-isomorphic matroids
If we look at the set $E = ∅$, we find there is exactly **one** matroid on $E$:
   - $`I = \{∅\}`$

If $`E = \{1\}`$, we find the **two** matroids on $E$:
   - $`I = \{∅\}`$
   - $`I = \{∅, \{1\}\}`$

If $`E = \{1, 2\}`$ there are **five** matroids on E:
   - $`I = \{∅\}`$
   - $`I = \{∅, \{1\}\}`$
   - $`I = \{∅, \{2\}\}`$
   - $`I = \{∅, \{1\}, \{2\}\}`$
   - $`I = \{∅, \{1\}, \{2\}, \{1, 2\}\}`$

We have the same structure in the matroids $`I = \{∅, \{1\}\}`$ and $`I = \{∅, \{2\}\}`$. Even though there are **five** matroids, we have **four non-isomorphic matroids** on this set since $`\{∅, \{1\}\}`$ and $`\{∅,\{2\}\}`$ are **isomorphic**.

<br>

If $`E = \{1, 2, 3\}`$ we have **eight non-isomorphic matroids**. They are:
- $`I = \{∅\}`$
- $`I = \{∅, \{1\}\}`$
- $`I = \{∅, \{1\}, \{2\}\}`$
- $`I = \{∅, \{1\}, \{2\}, \{1, 2\}\}`$
- $`I = \{∅, \{1\}, \{2\}, \{3\}\}`$
- $`I = \{∅, \{1\}, \{2\}, \{3\}, \{1, 2\}, \{2, 3\}\}`$
- $`I = \{∅, \{1\}, \{2\}, \{3\}, \{1, 2\}, \{2, 3\}, \{1, 3\}\}`$
- $`I = \{∅, \{1\}, \{2\}, \{3\}, \{1, 2\}, \{1, 3\}, \{2, 3\}, \{1, 2, 3\}\}`$
