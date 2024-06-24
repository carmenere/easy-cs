# Table of contents
- [Table of contents](#table-of-contents)
- [Graphs](#graphs)
  - [Definitions](#definitions)
    - [Definitions for undirected graphs](#definitions-for-undirected-graphs)
    - [Definitions for directed graphs](#definitions-for-directed-graphs)
  - [Types of graphs](#types-of-graphs)
  - [Distance, diameter](#distance-diameter)
  - [Walk, trail, path, circuit, cycle](#walk-trail-path-circuit-cycle)
  - [Eulerian graph](#eulerian-graph)
  - [Hamiltonian graph](#hamiltonian-graph)
- [Subgraphs](#subgraphs)
- [Trees](#trees)
- [Connectivity](#connectivity)
  - [Undirected graphs](#undirected-graphs)
  - [Directed graphs](#directed-graphs)
- [Connected components](#connected-components)
    - [Undirected graphs](#undirected-graphs-1)
    - [Directed graphs](#directed-graphs-1)
- [Transpose graph](#transpose-graph)
- [Condensation graph](#condensation-graph)
- [Graph labeling](#graph-labeling)
  - [Graph coloring](#graph-coloring)
- [Flow network](#flow-network)
- [Representations of graphs](#representations-of-graphs)
  - [Adjacency list](#adjacency-list)
  - [Adjacency matrix](#adjacency-matrix)
  - [Incidence matrix](#incidence-matrix)

<br>

# Graphs
An **graph** $G$ is an **ordered pair** $(V, E)$, where $V$ is the **set of vertices** and $E$ is the **set of edges**.<br>

A **vertex** is an **element** of $V$. A **vertex** (aka **node**) is the **fundamental unit** of which graphs are formed.<br>
An **edge** is an **element** of $E$. An **edge** (aka **link**) is a **pair of vertices**.<br>

<br>

## Definitions
An *edge* **joins** *vertices*. The *vertices* of an *edge* are called the **endpoints** of the *edge*.<br>

A **loop** is an *edge* that *connects* a **vertex to itself**.<br>
The **multiple edges** (aka **parallel edges**) are *edges* *connecting* the **same two vertices**.<br>

The **order of a graph** is its **number of vertices**: $|V|$.<br>
The **size of a graph** is its **number of edges**: $|E|$.<br>

<br>

A *pair of vertices* can be **ordered** or **unordered**:
- **unordered** *pair of vertices*, denoted as $\{u,v\}$, is a **2-element subset** of $V$;
- **ordered** *pair of vertices*, denoted as $(u,v)$, is a **ordered pair** on $V$, i.e. $E \subseteq V \times V$;

<br>

Edges may **have orientation** (aka **direction**) or **not**:
- **unordered** *pairs of vertices* $\{u,v\}$ represent *edges* **without orientation**;
- **ordered** *pairs of vertices* $(u,v)$ represent *edges* **with orientation** (aka **directed edges**, **arcs**);

<br>

So,
- an **undirected graph** is a *graph* $G$ in which *edges* **have not** *orientation*.<br>
- a **directed graph** (aka **digraph**) is a *graph* $G$ in which *edges* **have** *orientation*.<br>

<br>

### Definitions for undirected graphs
The *edge* $e = \{v,u\}$ is **between** $v$ and $u$.<br>
The *edge* $e = \{v,u\}$ means $e$ is **incident on** $v$ and $u$.<br>

A *vertex* is **incident on** *edge* if the *vertex* is one of the **endpoints** of that *edge*.<br>
*Two edges* are **incident** if they **share** a **common vertex**.<br>

<br>

The **number of edges** *incident* **to** $v$ is the **degree** (aka **valency**) of $v$, denoted $𝛿(v)$. In other words, the **degree of vertex** is the **number of edges incident** on it.<br>

An **isolated vertex** is a vertex with $𝛿(v) = 0$.<br>
A **leaf vertex** (aka **pendant vertex**) is a vertex with $𝛿(v) = 1$.<br>

For graphs with loops, the **loop increases the degree of the vertex** by $2$.<br>

<br>

*Two vertices* $v$ and $v$ are **adjacent** if they belong to the **same edge**. $v$ and $v$ are **neighbors**.<br>
*Two edges* are **adjacent** if they **share a common vertex**, e.g. if they are **incident** with the **same vertex**.<br>

<br>

### Definitions for directed graphs
The *edge* $e = (v,u)$ is **from** $v$ **to** $u$.<br>
The *edge* $e = (v,u)$ means $e$ is
- **incident from** $v$;
- **incident to** $u$;
- **incident on** $v$ and $u$;

Two *vertices* $v$ and $u$ are **incident on** the *edge* $(v,u)$.<br>
Two *edges* $e1 = (v,u1)$ and $e2 = (v,u2)$ are **incident** as they **share** the **common vertex** $v$.<br>

<br>

The **number of edges** *incident* **to** $v$ is the **in-degree** of $v$, denoted $𝛿 -(v)$. In other words, the **in-degree of vertex** is the **number of edges** of **incoming** to it.<br>
The **number of edges** *incident* **from** $v$ is the **out-degree** of $v$, denoted $𝛿 +(v)$. In other words, the **out-degree of vertex** is the **number of edges** of **outgoing** from it.<br>

The **sink vertex** is a *vertex* with $𝛿 +(v) = 0$.<br>
The **source vertex** is a *vertex* with $𝛿 -(v) = 0$.<br>

<br>

The *vertex* $v$ is **adjacent** to $u$, if there is an **edge** $(v,u)$, i.e edge **leaving** $v$ and **coming** to $u$.<br>
The *vertex* $u$ is **adjacent** to $v$, if there is an **edge** $(u,v)$, i.e. edge **leaving** $u$ and **coming** to $v$.<br>

<br>

## Types of graphs
An **empty graph** is a graph that has an **empty set of vertices** and thus an **empty set of edges**.<br>
A **simple graph** is *graph* containing **no** *loops* and **no** *multiple* edges.<br>
A **multigraph** is a *graph* which is **permitted** to have **multiple edges**. In **multigraph** $E$ is a **multiset**.<br>
A **pseudograph** is a **multigraph** that is **permitted** to have **loops**.<br>

A **finite graph** is a graph in which the vertex set and the edge set are **finite** sets.<br>
A **regular graph** is a graph in which **every** vertex has **the same degree**. A **regular graph** with vertices of **degree** $k$ is called a **k‑regular graph** or **regular graph of degree** $k$.<br>
A **complete graph** is a graph in which **each pair** of vertices is joined by an edge. A complete graph contains **all possible edges**. The **complete graph** on $n$ vertices is denoted by $`K_n`$.

<br>

Properties of $`K_n`$:
- $`K_n`$ has $n(n − 1)/2$ edges. 
- $`K_n`$ is a **regular graph** of **degree** $n − 1$.

A **bipartite graph** is a **simple graph** in which the vertex set can be partitioned into two sets, $V1$ and $V2$, so that **every** edge **join** vertices from **different** sets.

A **complete bipartite graph** is a **bipartite graph** in which **every** vertex of the first set $V1$ is connected to **every** vertex of the second set $V2$.<br>
A **complete bipartite graph** with partitions of size $|V1| = m$ and $|V2| = n$, is denoted $`K_{m,n}`$.<br>

A **planar graph** is a graph whose vertices and edges can be drawn in a plane such that no two of the edges intersect.<br>

A **planar graph** is a graph that can be **embedded in the plane**, in other words, it can be drawn on the plane in such a way that **no edges cross each other**.<br>

If graph **cannot** be embedded in the plane it is called **non planar**.<br>

Minimal non **planar graphs** are $`K_5`$ and $`K_{3,3}`$. Any other graph that contains $`K_5`$ or/and $`K_{3,3}`$ is **non planar** too.<br>

A **plane graph** is a graph that **has already embedded in the plane**.

<br>

## Distance, diameter
The **distance** between **two vertices** in a graph is the **length of a shortest path between them**.<br>
The **diameter** of a **connected graph** is the **largest distance in this graph**.<br>

<br>

## Walk, trail, path, circuit, cycle
A **walk** (or **directed walk** in *directed graph*) is a **sequence of adjacent edges**.<br>
A **length** of *walk* is a **number of edges** in it. So, the *walk* of length $k$ contains $k$ **edges** and $k+1$ **vertices**.<br>

There are *2 types of walks*:
- **open walk** is a *walk* in which the **first** and **last** vertices **aren't equal** ($v_0 \neq v_k$).<br>
- **closed walk** is a *walk* in which the **first** and **last** vertices are **equal** ($v_0 = v_k$).<br>

<br>

A **trail** (or **directed trail** in *directed graph*) is an **open walk** in which all **edges** are **distinct**.<br>
A **path** (aka **simple trail**) (or **directed path** in *directed graph*) is a **trail** in which all **vertices** are **distinct**.<br>

<br>

A **circuit** (or **directed circuit** in *directed graph*) is a **closed trail**.<br>
A **cycle** (aka **simple cycle**) (or **directed cycle** in *directed graph*) is a **closed path**. So, in **simple cycle** only the **starting** and **ending** vertices are *repeated* and **no other** vertices are *repeated*.<br>

<br>

A *graph* **without** *cycles* is called an **acyclic graph**.<br>
A *connected graph* **without** *cycles* is called a **tree**.<br>
A *directed graph* **without** *directed cycles* is called a **directed acyclic graph** (**DAG**).<br>

<br>

The number $`N_k`$ of **all paths** of length $k<=n$ in complete graph $`K_n`$ is a **$k$-permutation of $n$ vertices without repetitions**, because **at every $i$ step** in *path* we have $n-i$ **choices**: $`N_k = \underbrace{n  \times n-1  \times n-2 \  ... \ n-k +1}_\text{k times} = P(n,k) = n!/(n-k)!`$.<br>

<br>

Note that in the **walk** the same vertex $`v_j`$ **cannot** appear next to each other, for example $`v_jv_j`$ is **not** valid, but $`v_jv_mv_j`$ is **valid**. So, the number $`W_k`$ of **all walks** of length $k>0$ in complete graph $`K_n`$ is a **$(k-1)$-permutation of $(n-1)$ vertices with repetitions multiplied by $n$**, because **at first step** we have all possible $n$ **choices**, but **at every subsequent step** in *walk* we have only $n-1$ **choices** and for $k-1$ remaining **vertices** in *walk*: $`W_k = n \times \underbrace{n-1  \times n-1  \times n-1 \  ...}_\text{k-1 times} = n \times {(n-1)}^{k-1}`$.<br>

<br>

## Eulerian graph
**Eulerian trail** is a **trail** that visits **every** graph’s **edge**.<br>
**Eulerian circuit** is a **closed Eulerian trail**.<br>
**Eulerian graph** is a graph that contains a **Eulerian circuit**.<br>

**Euler's Theorem:** a **connected** graph has an **Eulerian circuit** iif **every vertex** has **even degree**.<br>

<br>

## Hamiltonian graph
**Hamiltonian path** is a **path** that visits **every** graph’s **vertex**.<br>
**Hamiltonian cycle** is a **closed Hamiltonian path**.<br>
**Hamiltonian graph** is a graph that contains a **Hamiltonian cycle**.<br>

<br>

# Subgraphs
A **subgraph** of a graph $G = (V, E)$ is a graph $G’ = (V’, E’)$ whose **vertex set** $V’ ⊂ V$ and **edge set** $E’ ⊂ E$. In other words, **each** node in a **subgraph** is also a **node** in the **supergraph**. Further, **every** edge in the **subgraph** is an edge in the **supergraph**.<br>

If $G'$ is a **subgraph** of $G$, then $G$ is said to be a **supergraph** of $G'$.<br>

An **induced subgraph** $G’$ of a graph $G = (V, E)$ is a graph $G’ = (S, E’)$ whose vertex set $S ⊂ V$ and $E’$ contains **all** edges of $G$ that **have endpoints** in $S$. In other words, if **two vertices** ${u,v}: u,v ∈ S$ are **adjacent** in $G$ they **must be adjacent** in $G’$.<br>

So, an **induced subgraph** can be constructed by **deleting vertices with its incident edges**, but **no more edges**.<br>

Consider graph $G$:

<br>

![graph](/img/graph.png)

<br>

To construct **induced subgraph** $G’$ from $G$ with $S = {D, E, G, J, K}$ **delete vertices** $B$, $A$, $C$, $F$, $I$, $L$, $H$ and **its incident edges**.

**Induced subgraph** $G’$ from $G$ with $S = {D, E, G, J, K}$:

<br>

![induced-subgraph](/img/induced-subgraph.jpeg)

<br>

Consider following **subgraphs** of graph $G$ with $V’ = {D, E, G, J, K}$ (they are *all* **not** *induced subgraphs*, they are **all ordinary subgraphs**.):

<br>

![graph](/img/subgraphs.jpeg)

<br>

# Trees
A **tree** is **connected acyclic undirected graph**.<br>

There is **one and only one path** between **every pair** of vertices in a **tree**.<br>

**Spanning tree** of an **undirected graph** $G$ is a **subgraph** that is a **tree** which **includes all** of the **vertices** of $G$.<br>

**Forest** – set of **separated trees**.<br>

<br>

# Connectivity
*Vertices* $u$ and $v$ are called **connected**, if graph **contains** a **path** from $u$ to $v$.<br>
*Vertices* $u$ and $v$ are called **disconnected**, if graph **doesn’t contains** a **path** from $u$ to $v$.<br>

<br>

## Undirected graphs
A **connected graph** is *graph* in which **every pair** of vertices is **connected**. This means that there is a **path between every pair of vertices**.<br>
A **disconnected graph** is *graph* in which **at least one pair** of **disconnected** vertices.<br>

<br>

## Directed graphs
A **directed graph** is **weakly connected** if **replacing** all of its **directed edges** with **undirected edges** produces a **connected undirected graph**.<br>

A **directed graph** is **semi-connected** if it contains a **directed path** from $u$ to $v$ **OR** a **directed path** from $v$ to $u$ **for every pair** of vertices $u, v ∈ V$.<br>

A **directed graph** is **strongly connected** if there is a **path** in **each direction** between **each pair** of vertices of the graph. In other words, **directed graph** is **strongly connected** if **every vertex is reachable from every other vertex**.<br>

<br>

**Consequences**:
- a **graph with just one vertex** is **connected**.
- an **edgeless** graph with two or more vertices is **disconnected**.

<br>

# Connected components
### Undirected graphs
**Connected component** (or just **component**) of an **undirected graph** $G$ is a **maximal** (by **inclusion**) **connected subgraph** of graph $G$.<br>

A **connected graph** has exactly **1 connected component**, consisting of the **whole graph**.<br>

An *edge* in an *undirected graph* is called a **bridge** (or **cut-edge**) iff deleting that edge **disconnects** the **connected component** to which that edge belongs.<br>

<br>

### Directed graphs
**Strongly connected component** (aka **SCC**) of a **directed** graph $G$ is a **directed subgraph** $D*$ that is **strongly connected** and **maximal**: **no** additional edges or vertices from $G$ can be included in the subgraph $D*$ without breaking its property of being **strongly connecte**d.<br>

<br>

> **Note**: In other words, **SCC** is a **cycle** in **digraph**.<br>

<br>

**Example**:<br>
![strong-components](/img/strong-components.png)

<br>

**Weakly connected component** (aka **WCC**) of a **directed** graph $G$ is a **undirected subgraph** $U*$ that is **connected** and **maximal**: **no** additional edges or vertices from $G$ can be included in the subgraph $U*$ without breaking its property of being **connected**.<br>

**Example**:
![weak-components](/img/weak-components.png)

<br>

**Bridge** (aka **cut-edge**) is an **edge** of a graph whose deletion **increases** the graph's **number** of **connected components**.<br>

<br>

# Transpose graph
The **transpose** of a *directed graph* $G = (V, E)$ (aka **the converse**, **the reverse**) is another *directed graph* $`G^T`$ on the same set of vertices $V$ with all of the edges **reversed** compared to the orientation of the **corresponding edges** in $G$. If $G$ *contains* an **edge** $(u, v)$ then the **transpose** of $G$ *contains* an **edge** $(v, u)$ and **vice versa**.<br>

The **adjacency matrix** of the **transpose** is the **transpose** of the **adjacency matrix** of the **original** *directed graph*.<br>

Example:<br>

![transponse](/img/transponse.png)

<br>

# Condensation graph
The **strongly connected components** of *directed graph* **don't** *intersect* each other.<br>
Given directed graph $G = (V, E)$, 
The **condensation graph** of a given *directed graph* $G = (V, E)$ is a graph $CG$ containing **every strongly connected component** of graph $G$ as **one vertex**.<br>
In other words, **each vertex** of the **condensation graph** $CG$ corresponds to the **strongly connected component** of **original** graph $G$.<br>

Let $G$ contain two **strongly connected components**: $SCC1$ and $SCC2$. If $G$ contains edge $(v, u)$, such that $v \in SCC1$ and $u \in SCC2$, then $CG$ contains edge $(SCC1, SCC2)$.<br>

<br>

The **most important property** of the *condensation graph*:
- *condensation graph* is **DAG**;
- any $SCC$ that is **source** in *condensation* of $G$ *becomes* **sink** in *condensation* of $`G^T`$;
- any $SCC$ that is **sink** in *condensation* of $G$ *becomes* **source** in *condensation* of $`G^T`$;

<br>

Given graph:<br>
![condensation-1](/img/condensation-1.png)

It has **4 SCCs**:
![condensation-2](/img/condensation-2.png)

Its **condensation**:
![condensation-3](/img/condensation-3.png)

<br>

# Graph labeling
**Graph labelling** is the assignment of **labels** to **edges and/or vertices** of a graph. The *edges* or *vertices* are given **labels** that are **meaningful** in the **associated domain**.<br>

When the **labels** are **numbers**, the **labels** are called **weights**.<br>

<br>

Kinds of **labeled graphs**: 
- **vertex-labeled graph** is a graph where **each vertex** has a corresponding **label**;
- **edge-labeled graph** is a graph where **each edge** has a corresponding **label**;
- **edge-labeled and vertex-labeled graph**;

<br>

Kinds of **weighted graphs**: 
- **edge-weighted graph**;
- **vertex-weighted graph**;
- **edge-weighted and vertex-weighted graph**;

<br>

> **Note**:<br>
> The term **labeled graph** generally refers to a **vertex-labeled graph**.<br>
> The term **weighted graph** generally refers to a **edge-weighted graph**.<br>

<br>

## Graph coloring
**Graph coloring** is a **special case of graph labeling**, i.e. it is an assignment of **labels** (aka **colors**) to elements of a graph in such a way as to **satisfy** the **constraints** (**restrictions**).<br>

Kinds of **graph coloring**:
- **vertex coloring** is a way of coloring the vertices of a graph such that **no** two **adjacent vertices** are of the **same** color; 
- **edge coloring** is a way of coloring the vertices of a graph such that **no** two **adjacent edges** are of the **same** color;

<br>

# Flow network
In graph theory, a **flow network** (also known as a **transportation network**) is a **directed graph** where each edge has a **capacity** and each edge receives a **flow**. The **amount of flow** on an edge **cannot** exceed the **capacity** of the edge.<br>
Often in operations research, a *directed graph* is called a **network**, the *vertices* are called **nodes** and the *edges* are called **arcs**.<br>
A flow must satisfy the restriction that the *amount of flow* **into** a node **equals** the *amount of flow* **out** of it, unless it is a source, which has only outgoing flow, or sink, which has only incoming flow.<br>

<br>

# Representations of graphs
**Three ways** to represent a **finite graph**:
- **adjacency list**;
- **adjacency matrix;**
- **incidence matrix**;

<br>

## Adjacency list
An **adjacency list** is a **collection** of **unordered lists** used to represent a finite graph.<br>
**Each** *unordered list* describes the **set of neighbors** of a particular **vertex** in the graph.<br>

Example of **graph**:<br>
![graph-example-1](/img/graph-example-1.png)

And *corresponding* **adjacency list**:<br>
![adjacency-list-1](/img/adjacency-list-1.png)

<br>

## Adjacency matrix
The **adjacency matrix** of a finite graph is a **square** $`n×n`$ matrix $`A`$, where both **columns** and **rows** correspond to **vertices**, such that:

$$
A_{ij}=
\begin
{cases}
1&{\text{if vertex }}\nu_{i}{\text{ is \textbf{incident} with vertex }}\nu_{j}, \\
0&{\text{otherwise}}
\end
{cases}
$$

<br>

So, elements of the **adjacency matrix** indicate whether pairs of vertices are **adjacent** or **not**. For **weighted graphs** elements of the **adjacency matrix** store **weight** instead **1**.

<br>

**Properties**:
1. The **diagonal elements** of the *adjacency matrix* are **all zero**.
2. The *adjacency matrix* of a **undirected graph** is **symmetric**.
3. The *adjacency matrix* of a **directed graph can** be **asymmetric**.
4. The **total number** of values **greater than** 0 in **row** $`i`$ (or **column** $`i`$) represents the **degree** of vertex $`\nu_i`$.

<br>

## Incidence matrix
The **incidence matrix** of a finite **undirected** graph is a $`n×m`$ matrix $`A`$, where **columns** correspond to **edeges** and **rows** correspond to **vertices**, such that:

$$
A_{ij}=
\begin
{cases}
1&{\text{if vertex }}\nu_{i}{\text{ is \textbf{incident} with edge }}e_{j}, \\
0&{\text{otherwise.}}
\end
{cases}
$$

<br>

The **sum** of **each column** in the *incidence matrix* is equal to **2**.<br>

<br>

The **incidence matrix** of a finite **directed** graph is a $`n×m`$ matrix $`A`$, where **columns** correspond to **edeges** and **rows** correspond to **vertices**, such that:

$$
A_{ij}=
\begin
{cases}
1&{\text{if edge }} e_{j} {\text{ is \textbf{enters} vertex }} \nu_{i}, \\
-1&{\text{if edge }} e_{j} {\text{ is \textbf{leaves} vertex }} \nu_{i}, \\
0&{\text{otherwise.}}
\end
{cases}
$$

<br>
