# Table of contents
- [Table of contents](#table-of-contents)
- [Undirected graph](#undirected-graph)
  - [Degree](#degree)
  - [Incidence](#incidence)
  - [Adjacency](#adjacency)
  - [Types of graphs](#types-of-graphs)
  - [Subgraph](#subgraph)
  - [Walk, trail, path](#walk-trail-path)
  - [Circuit and cycle](#circuit-and-cycle)
  - [Distance, diameter](#distance-diameter)
- [Trees](#trees)
- [Directed graph](#directed-graph)
- [Connected graph](#connected-graph)
- [Connected component](#connected-component)
- [Independent sets](#independent-sets)
  - [Independent set](#independent-set)
  - [Maximum independent set](#maximum-independent-set)
  - [Dominating set](#dominating-set)
  - [Maximal independent set](#maximal-independent-set)
  - [Examples](#examples)
    - [Example 1](#example-1)
    - [Example 2](#example-2)
    - [Example 3](#example-3)
- [Weighted graph](#weighted-graph)

<br>

# Undirected graph 
**Undirected graph** is an ordered pair `G = (V, E)`, where `V` is the **vertex set** (or the **set of vertices**), and `E` is the **edge set**.<br>

**Vertex** /ˈvɜːtɛks/ (plural vertices /ˈvɜːtɪˌsiːz/) (aka **node**) is the fundamental unit of which graphs are formed.<br>

**Edge** (aka **link**) is an **unordered pair of vertices**; more formal: `E ⊂ {{x,y}: x,y ∈ V}`.
The edge **joins** `x` and `y`.<br>
The **vertices** `x` and `y` of an *edge* `{x,y}` are called the **endpoints** of the edge.<br>

The **order of a graph** is its **number of vertices**: `|V|`.<br>
The **size of a graph** is its **number of edges**: `|E|`.<br>

An **empty graph** is a graph that has an **empty set of vertices** and thus an **empty set of edges**.<br>

<br>

## Degree
The **degree** /dɪ'griː/ (aka **valency**) of a vertex, denoted `𝛿(v)` is the *number of edges* **incident** to it. 

- an **isolated vertex** is a vertex with `𝛿(v) = 0`;
- a **leaf vertex** (aka **pendant vertex** /'pendənt/) is a vertex with `𝛿(v) = 1`;

For graphs with loops, the **loop increases the degree of the vertex** by `2`.

<br>

## Incidence
The **edge** `e = {x,y}` is **incident** to **its vertices** `x` and `y`.<br>
Edge (`x → y`) in **directed** graph is **incident** **on** the vertex `y` **from** the vertex `x`.<br>

A vertex is **incident** with an edge if the vertex is one of the **endpoints** of that edge.<br>

<br>

## Adjacency
Two vertices `x` and `y` are **adjacent** if they belong to the **same edge**.<br>
Two edges are **adjacent** if they **share a common vertex**, e.g. if they are **incident** with the **same vertex**.

<br>

## Types of graphs
**Loop** is an edge that **connects** a **vertex to itself**.<br>
**Multiple edges** (aka **parallel** edges) are edges **connecting** the **same two vertices**.<br>

**Simple graph** is an **unweighted**, **undirected graph** containing **no** *loops* and **no** *multiple* edges.<br>
**Multigraph** is a graph which is permitted to have **multiple edges**.<br>
**Pseudograph** is a *multigraph* that is permitted to have **loops**.<br>

A **finite graph** is a graph in which the vertex set and the edge set are **finite** sets.<br>
A **regular graph** is a graph in which **every** vertex has **the same degree**. A **regular graph** with vertices of **degree** `k` is called a **k‑regular graph** or **regular graph of degree** `k`.<br>
A **complete graph** is a graph in which **each pair** of vertices is joined by an edge. A complete graph contains **all possible edges**. The **complete graph** on `n` vertices is denoted by **K<sub>n</sub>**.

<br>

Properties of **K<sub>n</sub>**:
- **K<sub>n</sub>** has `n(n − 1)/2` edges (a triangular /traɪ'æŋgjələ/ number). 
- **K<sub>n</sub>** is a **regular graph** of **degree** `n − 1`.

A **bipartite graph** /baɪˈpɑːtaɪt/ is a **simple graph** in which the vertex set can be partitioned into two sets, `V1` and `V2`, so that **every** edge **join** vertices from **different** sets.

A **complete bipartite graph** is a **bipartite graph** in which **every** vertex of the first set `V1` is connected to **every** vertex of the second set `V2`.<br>
A **complete bipartite graph** with partitions of size `|V1| = m` and `|V2| = n`, is denoted **K<sub>m,n</sub>**.<br>

A **planar graph** is a graph whose vertices and edges can be drawn in a plane such that no two of the edges intersect.<br>

A **planar graph** is a graph that can be **embedded in the plane**, in other words, it can be drawn on the plane in such a way that **no edges cross each other**.<br>

If graph **cannot** be embedded in the plane it is called **non planar**.<br>

Minimal non **planar graphs** are **K<sub>5</sub>** and **K<sub>3,3</sub>**. Any other graph that contains **K<sub>5</sub>** or/and **K<sub>3,3</sub>** is **non planar** too.<br>

A **plane graph** is a graph that **has already embedded in the plane**.

<br>

## Subgraph
A **subgraph** of a graph `G = (V, E)` is a graph `G’ = (V’, E’)` whose **vertex set** `V’ ⊂ V` and **edge set** `E’ ⊂ E`. In other words, **each** node in a **subgraph** is also a **node** in the **supergraph**. Further, **every** edge in the **subgraph** is an edge in the **supergraph**.<br>

If `G'` is a **subgraph** of `G`, then `G` is said to be a **supergraph** of `G'`.<br>

An **induced subgraph** `G’` of a graph `G = (V, E)` is a graph `G’ = (S, E’)` whose vertex set `S ⊂ V` and `E’` contains **all** edges of `G` that **have endpoints** in `S`. In other words, if **two vertices** `{u,v}: u,v ∈ S` are **adjacent** in `G` they **must be adjacent** in `G’`.<br>

So, an **induced subgraph** can be constructed by **deleting vertices with its incident edges**, but **no more edges**.<br>

Consider graph `G`:
![graph](/img/graph.png)

<br>

To construct **induced subgraph** `G’` from `G` with `S = {D, E, G, J, K}` **delete vertices** `B`, `A`, `C`, `F`, `I`, `L`, `H` and **its incident edges**.

**Induced subgraph** `G’` from `G` with `S = {D, E, G, J, K}`:
![induced-subgraph](/img/induced-subgraph.png)

<br>

Consider following **subgraphs** of graph `G` with `V’ = {D, E, G, J, K}`:
![graph](/img/subgraphs.png)

<br>

They are **all not induced subgraphs**, they are **all ordinary subgraphs**.

<br>

## Walk, trail, path
A **walk** is a **sequence of adjacent edges**.<br>
A **trail** is a **walk** in which all **edges** are **distinct**.<br>
A **path** (aka **simple trail**) is a **trail** in which all **vertices** are **distinct**.<br>

**Length** of *walk*/*trail*/*path* is a **number of edges** in it.<br>

<br>

## Circuit and cycle
A **closed walk** is a **walk** in which the **first** and **last** vertices are **equal** (`v0=vn`).<br>
A **circuit** is a **trail** in which the **first** and **last** vertices are **equal**, i.e. **circuit** is **closed trail**.<br>
A **cycle** (aka **simple circuit**) is a **path** in which the **first** and **last** vertices are **equal**, i.e. **cycle** is **closed path**.<br>

<br>

## Distance, diameter
The **distance** between **two vertices** in a graph is the **length of a shortest path between them**.<br>
The **diameter** of a **connected graph** is the **largest distance in this graph**.<br>


**Eulerian trail** is a **trail** that visits **every** graph’s **edge**.<br>
**Eulerian circuit** is a **closed Eulerian trail**.<br>

**Euler's Theorem:** a **connected graph** has an Eulerian circuit iif **every vertex** has **even degree**.<br>

**Hamiltonian path** is a **path** that visits **every** graph’s vertex.<br>
**Hamiltonian cycle** is a **closed Hamiltonian path**.<br>

**Eulerian graph** is a graph that contains a **Eulerian circuit**.<br>
**Hamiltonian graph** is a graph that contains a **Hamiltonian cycle**.<br>

<br>

# Trees
A **tree** (дерево) is **connected acyclic undirected graph**.<br>
There is **exactly one path between any two vertices in tree**.<br>

**Spanning tree** of an **undirected graph** `G` is a **subgraph** that is a **tree** which **includes all** of the **vertices** of `G`.<br>

**Forest** – set of **separated trees**.<br>

<br>

# Directed graph
A **directed graph** (**digraph**) is a graph in which edges have **orientations** and called **directed edges** (aka **arrows**).<br>

**Directed edge** is an **ordered pair** of **vertices**; more formal: `E ⊂ {(x,y): x,y ∈ V}`.<br>

The **outdegree** of vertex is the number of **outgoing** edges, denoted `𝛿 +(v)`.<br>
The **indegree** of vertex is the number of **incoming** edges, denoted `𝛿 -(v)`.<br>

The **sink** vertex is a vertex with `𝛿 +(v) = 0`.<br>
The **source** vertex is a vertex with `𝛿 -(v) = 0`.<br>

Vertices `u` and `v` are called **connected**, if graph **contains** a **path** from `u` to `v`.<br>
Vertices `u` and `v` are called **disconnected**, if graph **doesn’t contains** a **path** from `u` to `v`.<br>

<br>

# Connected graph
A **connected graph** is graph in which **every pair** of vertices is **connected**. This means that there is a **path between every pair of vertices**.<br>
A **disconnected** graph is graph in which **at least one pair** of **disconnected** vertices.<br>

<br>

In digraphs:
- a **directed graph** is **weakly connected** if replacing all of its directed edges with undirected edges produces a **connected undirected graph**. 
- a **directed graph** is **semi-connected** if it contains a **directed path** from `u` to `v` OR a **directed path** from `v` to `u` for every pair of vertices `u, v ∈ V`.
- a **directed graph** is **strongly connected**, if it contains a **directed path** from `u` to `v` AND a **directed path** from `v` to `u` for every pair of vertices `u, v ∈ V`. In other words, **directed graph** is **strongly connected** if **every vertex is reachable from every other vertex**.

<br>

**Consequences** / 'kɔn(t)sɪkwən(t)siz/:
- a **graph with just one vertex** is **connected**.
- an **edgeless** graph with two or more vertices is **disconnected**.

<br>

# Connected component
**Connected component** (or just **component**) of an **undirected graph** `G` is a **maximal** (by **inclusion**) **connected subgraph** of graph `G`.
A **connected graph** has exactly **1 connected component**, consisting of the **whole graph**.<br>

<br>

In digraphs:
- **strong component** of a **directed** graph `G` is a **directed subgraph** that is **strongly connected** and is **maximal**: **no** additional edges or vertices from `G` can be included in the subgraph without breaking its property of being **strongly connected**;
- **weak component** of a **directed** graph `G` is a un**directed subgraph** that is **strongly connected** and is **maximal**: **no** additional edges or vertices from `G` can be included in the subgraph without breaking its property of being **strongly connected**;

**Bridge** (aka **cut-edge**) is an edge of a graph whose deletion **increases** the graph's number of connected components.<br>

<br>

# Independent sets
## Independent set
**Independent set** is a set of vertices `V` such that **any two vertices** in `V` are **not adjacent**. In other words, the induced subgraph by `V` consists of isolated vertices.<br>
There can be **more than one** independent sets for a given graph.<br>

<br>

## Maximum independent set
**Maximum independent set** is an **independent set** of **largest cardinality**.<br>

The **independence number** of graph `G` is the **cardinality** of its **maximum independent set**.<br>
The **independence number** of graph `G` is denoted by `α(G)`.<br>

<br>

## Dominating set
**Dominating set** for a graph `G = (V, E)` is a **subset** `D` of `V` such that every vertex **not** in `D` is adjacent to **at least one** member of `D`.<br>

There can be **more than one** dominating sets for a given graph.

The **domination number** of graph `G` is the **number of vertices** in its **smallest dominating set**.
The **domination number** of graph `G` is denoted by `γ(G)`.


## Maximal independent set
**Maximal independent set** (**MIS**) is an **independent set** that is **not** a subset of any other independent set.<br>

There can be **more than one** MIS for a given graph.<br>

Every maximum independent set is MIS but the converse /kənˈvɜːrs/ is not always true.<br>

The given graph has **6 different MIS** shown as the red vertices, **2** of them are **maximum**:
![max-independant-set-1](/img/max-independant-set-1.png)

Any MIS is also a **dominating set** in the graph, and every dominating set that is independent must be maximal, so MISs are also called independent dominating sets.

<br>

## Examples
### Example 1
![max-independant-set-2](/img/max-independant-set-2.png)

<br>

### Example 2
![graph-1](/img/graph-1.png)

<br>

All the possible **independent sets** for the given graph: `{ }; { 1 }; { 1 2 }; { 1 2 3 }; { 1 3 }; { 2 }; { 2 3 }; { 3 }`.<br>
All the possible **maximum independent sets** for the given graph: `{ 1 2 3 }`.<br>

<br>

### Example 3
![graph-2](/img/graph-2.png)

<br>

All the possible **independent sets** for the given graph: `{ }; { 1 }; { 1 3 }; { 2 }; { 2 4 }; { 3 }; { 4 }`.<br>
All the possible **maximum independent sets** for the given graph: `{ 1 3 }; { 2 4 }`.<br>

<br>

# Weighted graph 
A **weighted graph** is a **graph** or **digraph** in which the **number** (aka **weight**) is assigned to **each edge**.<br>
Such weights might represent different properties of real world: *distance*, *weight*, *flow* etc.<br>

**Graph labelling** is the assignment of **labels**, traditionally represented by integers, to **edges** and/or **vertices** of a graph.<br>

Kinds of labeled graphs: 
- **vertex-labeled** graph;
- **edge-labeled** graph;
- **edge-labeled** and **vertex-labeled** graph;

<br>

**Graph coloring** is a **special case of graph labeling**; it is an assignment of labels traditionally called **colors** to elements of a graph subject to certain **constraints**.<br>

**Examples** of graph coloring:
- **vertex coloring** is a way of coloring the vertices of a graph such that no two adjacent vertices are of the same color; 
- **edge coloring** assigns a color to each edge so that no two adjacent edges are of the same color;
- **face coloring** of a planar graph assigns a color to each face or region so that no two faces that share a boundary have the same color.
