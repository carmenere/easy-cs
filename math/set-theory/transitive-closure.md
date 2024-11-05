# Table of contents
- [Table of contents](#table-of-contents)
- [Transitive closure](#transitive-closure)
  - [Existence and description](#existence-and-description)
  - [Constructing the transitive closure](#constructing-the-transitive-closure)
    - [Pseudocode](#pseudocode)
  - [Examples](#examples)
    - [Example1](#example1)
  - [In graph theory](#in-graph-theory)
    - [Example: the transitive closure for DAG](#example-the-transitive-closure-for-dag)

<br>

A relation $R$ **may not** have a **desired property**, such as *reflexivity*, *symmetry*, or *transitivity*.<br>
The **smallest** relation $`R_p`$ such that $`R \subseteq R_p`$ and $`R_p`$ **satisfies** the **desired property** is the **closure of the relation** $R$ with respect to the **desired property**. Subscript $p$ means *desired property*.<br>

<br>

# Transitive closure
The **transitive closure** of the **homogeneous binary relation** $R$ on a set $X$ is the **smallest** relation $`R_t`$ (also $R^+$) on $X$ such that $`R_t`$ is **transitive** and $`R_t`$  **conatins** $R$.<br>

<br>

If $R$ itself is **transitive** then $`R_t = R`$.<br>

<br>


## Existence and description
1. The **intersection** of two **transitive relations** (not closures) is **transitive**.
2. The **union** of two **transitive relations** (not closures) need **not** be transitive.

<br>

For **any relation** $R$, the **transitive closure** of $R$ **always exists**. There exists **at least one transitive relation** containing $R$, the trivial one: $X × X$. The **transitive closure** of $R$ is then given by the **intersection** of **all transitive relations** (not closures) containing $R$.<br>

<br>

For any set $X$, we can prove that **transitive closure** is given by the following expression:<br>
$`{\displaystyle R_{t}=\bigcup _{i=1}^{\infty }R^{i}}`$

<br>

## Constructing the transitive closure
For **finite sets**, we can **construct** the *transitive closure* **step by step**, starting from $R$ and adding **transitive edges**:
1. Copy all elements from $R$ to $`R_t`$.
2. For any triple $x,y,z \in X$ such that $(x,y) \in R$ and $(y,z) \in R$ **add** $(x,z)$ **to** $`R_t`$.
3. **Repeat** *step 2* **until** $`R_t`$ **doesn't** satisfy **transitivity**.

<br>

### Pseudocode
```rust
v1, v2, ..., vn = an arbitrary numbering of the vertices     
G0 = G; // Initialize 

for (k = 1; k <= n; k++)
    Gk = Gk-1; // Build next graph
    for (i = 1; i <= n; i++)
        for (j = 1; j <= n; j++)
            if ( edges (i,k) and (k,j) exists in Gk )
                add edge (i,j) to Gk
```

<br>

## Examples
### Example1
Consider the set $S=\{a,b,c,d\}$ and **homogeneous binary relation** $R = \{(a,b),(b,c)(c,d),(b,a)\}$.<br>

Steps to construct the transitive closure:
1. Copy all elements from $R$ to $`R_t`$.
2. There is $(a,b) \in R$ and $(b,c) \in R$, so **add** $(a,c)$ **to** $`R_t`$.
3. There is $(a,b) \in R$ and $(b,a) \in R$, so **add** $(a,a)$ **to** $`R_t`$.
4. There is $(b,a) \in R$ and $(a,b) \in R$, so **add** $(b,b)$ **to** $`R_t`$.
5. There is $(b,c) \in R$ and $(c,d) \in R$, so **add** $(b,d)$ **to** $`R_t`$.
6. There is $`(a,c) \in R_t`$ and $`(c,d) \in R_t`$, so **add** $(a,d)$ **to** $`R_t`$.
7. and so on.

<br>

**Finally**:
$`R_t=\{(a,a),(a,d),(b,c),(a,b),(b,a),(b,d),(a,c),(b,b),(c,d)\}`$

<br>

## In graph theory
The **transitive closure** of a **graph** $G = (V,E)$ is a graph $G* = (V,E*)$ such that
- $G*$ has the **same vertices** as $G$;
- if $G$ has a **path from** $u$ **to** $v$ ($u \neq v$), $G*$ has an **edge** $(u,v)$;

<br>

The **transitive closure** of the **adjacency relation** of the **graph** is the **reachability relation** of the **graph**:
- a **binary relation** tells you **only** that **any** node $i$ is **directly connected or not** to **any** node $j$;
- a **transitive closure** tells you that **any** node $i$ is **reachable or not** from **any** node $j$;

<br>

### Example: the transitive closure for DAG
**DAG** is **directed acyclic graph**.<br>

![transitive-closure](/img/transitive-closure.png)