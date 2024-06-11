# Table of contents
- [Table of contents](#table-of-contents)
- [Undirected graphs](#undirected-graphs)
  - [Undirected graph](#undirected-graph)
  - [Incidence](#incidence)
  - [Adjacency](#adjacency)
  - [Degree](#degree)
  - [Types of graphs](#types-of-graphs)
  - [Subgraph](#subgraph)
  - [Walk, trail, path](#walk-trail-path)
  - [Circuit and cycle](#circuit-and-cycle)
  - [Distance, diameter](#distance-diameter)
  - [Eulerian graph](#eulerian-graph)
  - [Hamiltonian graph](#hamiltonian-graph)
- [Trees](#trees)
- [Directed graphs](#directed-graphs)
  - [Directed graph](#directed-graph)
- [Connected graph](#connected-graph)
  - [Connected component](#connected-component)
- [Graph labeling](#graph-labeling)
- [Flow network](#flow-network)
- [Representations of graphs](#representations-of-graphs)
  - [Adjacency list](#adjacency-list)
  - [Adjacency matrix](#adjacency-matrix)
  - [Incidence matrix](#incidence-matrix)
- [Algorithms](#algorithms)
  - [Spanning trees](#spanning-trees)
    - [Prim's algorithm](#prims-algorithm)
    - [Kruskal's algorithm](#kruskals-algorithm)
  - [Shortest paths](#shortest-paths)
    - [Dijkstra's algorithm](#dijkstras-algorithm)
      - [Algorithm](#algorithm)
    - [Bellman–Ford algorithm](#bellmanford-algorithm)
    - [Floyd–Warshall algorithm](#floydwarshall-algorithm)
      - [Pseudocode](#pseudocode)
  - [Flows](#flows)
    - [Ford–Fulkerson algorithm](#fordfulkerson-algorithm)

<br>

# Undirected graphs
## Undirected graph
**Undirected graph** is an ordered pair `G = (V, E)`, where `V` is the **vertex set** (or the **set of vertices**), and `E` is the **edge set**.<br>

**Vertex** (aka **node**) is the fundamental unit of which graphs are formed.<br>

**Edge** (aka **link**) is an **unordered pair of vertices**; more formal: `E ⊂ {{x,y}: x,y ∈ V}`.
The edge **joins** `x` and `y`.<br>
The **vertices** `x` and `y` of an *edge* `{x,y}` are called the **endpoints** of the edge.<br>

The **order of a graph** is its **number of vertices**: `|V|`.<br>
The **size of a graph** is its **number of edges**: `|E|`.<br>

An **empty graph** is a graph that has an **empty set of vertices** and thus an **empty set of edges**.<br>

<br>

## Incidence
The **edge** `e = {x,y}` is **incident** to **its vertices** `x` and `y`.<br>
The **edge** (`x → y`) in **directed** graph is **incident on** the vertex `y` **from** the vertex `x`.<br>

A vertex is **incident** with an edge if the vertex is one of the **endpoints** of that edge.<br>

<br>

## Adjacency
Two vertices `x` and `y` are **adjacent** if they belong to the **same edge**.<br>
Two edges are **adjacent** if they **share a common vertex**, e.g. if they are **incident** with the **same vertex**.

<br>

## Degree
The **degree** (aka **valency**) of a vertex, denoted `𝛿(v)` is the *number of edges* **incident** to it. 

- an **isolated vertex** is a vertex with `𝛿(v) = 0`;
- a **leaf vertex** (aka **pendant vertex**) is a vertex with `𝛿(v) = 1`;

For graphs with loops, the **loop increases the degree of the vertex** by `2`.

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
- **K<sub>n</sub>** has `n(n − 1)/2` edges. 
- **K<sub>n</sub>** is a **regular graph** of **degree** `n − 1`.

A **bipartite graph** is a **simple graph** in which the vertex set can be partitioned into two sets, `V1` and `V2`, so that **every** edge **join** vertices from **different** sets.

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

<br>

![graph](/img/graph.png)

<br>

To construct **induced subgraph** `G’` from `G` with `S = {D, E, G, J, K}` **delete vertices** `B`, `A`, `C`, `F`, `I`, `L`, `H` and **its incident edges**.

**Induced subgraph** `G’` from `G` with `S = {D, E, G, J, K}`:

<br>

![induced-subgraph](/img/induced-subgraph.jpeg)

<br>

Consider following **subgraphs** of graph `G` with `V’ = {D, E, G, J, K}` (they are *all* **not** *induced subgraphs*, they are **all ordinary subgraphs**.):

<br>

![graph](/img/subgraphs.jpeg)

<br>

## Walk, trail, path
A **walk** is a **sequence of adjacent edges**.<br>
A **trail** is a **walk** in which all **edges** are **distinct**.<br>
A **path** (aka **simple trail**) is a **trail** in which all **vertices** are **distinct**.<br>

**Length** of *walk*/*trail*/*path* is a **number of edges** in it.<br>

<br>

## Circuit and cycle
A **closed** *walk*/*trail*/*path* is a *walk*/*trail*/*path* in which the **first** and **last** vertices are **equal** (**v<sub>0</sub> = v<sub>n</sub>**).<br>
A **closed walk** is a **closed walk**.<br>
A **circuit** is a **closed trail**.<br>
A **cycle** is a **closed path**.<br>

<br>

## Distance, diameter
The **distance** between **two vertices** in a graph is the **length of a shortest path between them**.<br>
The **diameter** of a **connected graph** is the **largest distance in this graph**.<br>

<br>

## Eulerian graph
**Eulerian trail** is a **trail** that visits **every** graph’s **edge**.<br>
**Eulerian circuit** is a **closed Eulerian trail**.<br>

**Euler's Theorem:** a **connected** graph has an **Eulerian circuit** iif **every vertex** has **even degree**.<br>

**Eulerian graph** is a graph that contains a **Eulerian circuit**.<br>

<br>

## Hamiltonian graph
**Hamiltonian path** is a **path** that visits **every** graph’s **vertex**.<br>
**Hamiltonian cycle** is a **closed Hamiltonian path**.<br>
**Hamiltonian graph** is a graph that contains a **Hamiltonian cycle**.<br>

<br>

# Trees
A **tree** is **connected acyclic undirected graph**.<br>
There is **exactly one path between any two vertices in tree**.<br>

**Spanning tree** of an **undirected graph** `G` is a **subgraph** that is a **tree** which **includes all** of the **vertices** of `G`.<br>

**Forest** – set of **separated trees**.<br>

<br>

# Directed graphs
## Directed graph
A **directed graph** (**digraph**) is a graph in which edges have **orientations** and called **directed edges** (aka **arrows**).<br>

**Directed edge** is an **ordered pair** of **vertices**; more formal: `E ⊂ {(x,y): x,y ∈ V}`.<br>

The **outdegree** of vertex is the number of **outgoing** edges, denoted `𝛿 +(v)`.<br>
The **indegree** of vertex is the number of **incoming** edges, denoted `𝛿 -(v)`.<br>

The **sink vertex** is a vertex with `𝛿 +(v) = 0`.<br>
The **source vertex** is a vertex with `𝛿 -(v) = 0`.<br>

Vertices `u` and `v` are called **connected**, if graph **contains** a **path** from `u` to `v`.<br>
Vertices `u` and `v` are called **disconnected**, if graph **doesn’t contains** a **path** from `u` to `v`.<br>

<br>

# Connected graph
A **connected graph** is graph in which **every pair** of vertices is **connected**. This means that there is a **path between every pair of vertices**.<br>
A **disconnected** graph is graph in which **at least one pair** of **disconnected** vertices.<br>

<br>

In digraphs:
- a **directed graph** is **weakly connected** if replacing all of its directed edges with undirected edges produces a **connected undirected graph**. 
- a **directed graph** is **semi-connected** if it contains a **directed path** from `u` to `v` **OR** a **directed path** from `v` to `u` **for every pair** of vertices `u, v ∈ V`.
- a **directed graph** is **strongly connected**, if it contains a **directed path** from `u` to `v` **AND** a **directed path** from `v` to `u` **for every pair** of vertices `u, v ∈ V`. In other words, **directed graph** is **strongly connected** if **every vertex is reachable from every other vertex**.

<br>

**Consequences**:
- a **graph with just one vertex** is **connected**.
- an **edgeless** graph with two or more vertices is **disconnected**.

<br>

## Connected component
**Connected component** (or just **component**) of an **undirected graph** `G` is a **maximal** (by **inclusion**) **connected subgraph** of graph `G`.
A **connected graph** has exactly **1 connected component**, consisting of the **whole graph**.<br>

<br>

In digraphs:
- **strong component** of a **directed** graph `G` is a **directed subgraph** that is **strongly connected** and **maximal**: **no** additional edges or vertices from `G` can be included in the subgraph without breaking its property of being **strongly connected**;
- **weak component** of a **directed** graph `G` is a un**directed subgraph** that is **strongly connected** and **maximal**: **no** additional edges or vertices from `G` can be included in the subgraph without breaking its property of being **strongly connected**;

**Bridge** (aka **cut-edge**) is an edge of a graph whose deletion **increases** the graph's **number** of connected components.<br>

<br>

# Graph labeling
**Graph labelling** is the assignment of **labels**, traditionally represented by integers, to **edges and**/**or vertices** of a graph. The *edges* or *vertices* are given **labels** that are **meaningful** in the **associated domain**.<br>

<br>

Kinds of **labeled graphs**: 
- **vertex-labeled** graph;
- **edge-labeled** graph;
- **edge-labeled** and **vertex-labeled** graph;

<br>

When used **without** qualification, the term **labeled graph** generally refers to a **vertex-labeled graph** with all labels **distinct**.<br>

A **weighted graph** is a **graph** or **digraph** in which the **number** (aka **weight**) is assigned to **each edge**. Such **weights** might **represent different properties** of real world: *distance*, *weight*, *flow* etc.<br>

<br>

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

$$`
A_{ij}=
\begin
{cases}
1&{\text{if vertex }}\nu_{i}{\text{ is \textbf{incident} with vertex }}\nu_{j}, \\
0&{\text{otherwise}}
\end
{cases}
`$$

<br>

So, elements of the **adjacency matrix** indicate whether pairs of vertices are **adjacent** or **not**. For **weighted graphs** elements of the **adjacency matrix** store **weight** instead **1**.

<br>

**Properties**:
1. The **diagonal elements** of the *adjacency matrix* are **all zero**.
2. The *adjacency matrix* of a **undirected graph** is **symmetric**.
3. The *adjacency matrix* of a **directed graph can** be **asymmetric**.
4. The **total number** of values **greater than** 0 in **row** $`i`$ (or **column** $`i$) represents the **degree** of vertex $`\nu_i`$.

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

<br>

# Algorithms
## Spanning trees
### Prim's algorithm
The **Prim's algorithm** is a **greedy algorithm** that finds a **minimum spanning tree** for a **edge-weighted** graph.

<br>

### Kruskal's algorithm
**Kruskal's algorithm** is a **greedy algorithm** that finds a **minimum spanning forest** of an undirected **edge-weighted** graph. If the graph is **connected**, it finds a **minimum spanning tree**.
Worst-case performance **O(|E|log|V|)**.

<br>

## Shortest paths
### Dijkstra's algorithm
**Dijkstra's algorithm** (/ˈdaɪkstrəz/) finds the **shortest path** from a given **source node** to **every other node** in a **edge-weighted** *graph*.<br>
It can also be used to find the shortest path to a specific destination node, by terminating the algorithm once the shortest path to the destination node is know.<br>
With using a **Fibonacci heap priority queue** the **worst-case performance**: **O(|E| + |V|log|V|)**.<br>

*Dijkstra's algorithm* is just **BSF** that uses a **priority queue** instead **regular queue**. It **greedily** selects the **nearest vertex** that has **not** visited yet. So, Dijkstra's algorithm is a **greedy algorithm** because at each step, it selects the vertex with the smallest distance from the source vertex and adds it to the set of vertices that have been visited. This choice is made without considering the overall path or the global optimal solution.

The *Dijkstra's algorithm* is **greedy** because it makes **locally optimal choices at each step**.<br>

For example, if the nodes of the graph represent **cities**, and the costs of edges represent the average **distances** between pairs of cities connected by a direct road, then *Dijkstra's algorithm* can be used to find the **shortest route** between one city and all other cities.<br>

<br>

#### Algorithm
1. Mark **all** nodes as **unvisited**.
2. Assign to **every** node an **initial distance**: **zero** for the **starting node** and $`+\infty`$ for **all other nodes**.
3. Add **all** nodes to **priority queue**.
4. Remove from the **queue** next **unvisited** with the **smallest distance** node. It is **current node**.
5. Calculate `new_distance` for **every** child node of current node: `new_distance` = `min(distance_of_current_node + weight_of_edge_to_child, distance_of_child_node)` and assign it to child.
6. **Repeat** steps `4` and `5` while queue is **not** empty.

<br>

### Bellman–Ford algorithm
The **Bellman–Ford algorithm** finds the **shortest path** from a given **source** vertex to **all** other vertices in a **edge-weighted** *digraph*.<br>
**Worst-case performance O(|V||E|)**.<br>

It is **slower** than *Dijkstra's algorithm*, but it can handle graphs in which some of the edge **weights** are **negative** numbers.<br>

The *Bellman-Ford algorithm* is **dynamic** because it uses a **dynamic programming approach** to compute the shortest paths by **solving subproblems** and iteratively updating the distances.<br>

<br>

### Floyd–Warshall algorithm
The **Floyd–Warshall algorithm** (aka the **WFI algorithm**) finds **shortest paths** between **all** *pairs of vertices* in a **edge-weighted** *digraph* with **positive** or **negative** edge weights (but with **no negative cycles**).<br>
**Worst-case** and **best-case performance**: **O(|V|<sup>3</sup>)**.<br>
**Worst-case space** complexity: **O(|V|<sup>2</sup>)**.<br>

The *Floyd–Warshall algorithm* is an example of **dynamic programming**.<br>

Consider a **weighted graph** $`{\displaystyle G=\{V, E\}}`$ with vertices $`{\displaystyle V}`$ numbered $`{\displaystyle 1}`$ through through $`{\displaystyle n}`$.<br>

Let $`\omega_{\upsilon\nu}`$ denote **weight** of **edge** $`{\varepsilon_{\upsilon\nu}=\upsilon\nu}`$, then:
```math
\boldsymbol{\omega_{\upsilon\nu}}=
\begin
{cases}
\varepsilon_{\upsilon\nu}, & \text{if} & \varepsilon_{\upsilon\nu}∈E \\
+\infty, & \text{if} & \varepsilon_{\upsilon\nu}∉E
\end
{cases}
```

In other words $`{\omega_{\upsilon\nu}=+\infty}`$ **if** $`\nu_{i}`$ and $`\nu_{j}`$ are **not adjacent** (or **not directly connected**).

<br>

Let $`d^{(k)}_{ij}`$ denotes the **shortest path** from $`\nu_{i}`$ to $`\nu_{j}`$ that can contain **intermediate** vertices **only** from the set $`{\{\nu_{1},\nu_{2}, ..., \nu_{k}\}}`$.<br>

Then:
- $`{d^{(0)}_{ij}}`$ is just a **weight** ($\omega_{ij}$) of **edge** ($\varepsilon_{ij}$) - the **base case**;
- $`{d^{(1)}_{ij}}`$ is a **shortests distance** of **path** that pass through **vertex** $`\nu_{1}`$;
- $`{d^{(2)}_{ij}}`$ is a **shortests distance** of **path** that pass through **vertices** $`{\{\nu_{1},\nu_{2}\}}`$;
- ...
- $`{d^{(k-1)}_{ij}}`$ is a **shortests distance** of **path** that pass through **vertices** $`{\{\nu_{1},\nu_{2}, ..., \nu_{k-1}\}}`$;
- $`{d^{(k)}_{ij}}`$ is a **shortests distance** of **path** that pass through **vertices** $`{\{\nu_{1},\nu_{2}, ..., \nu_{k}\}}`$;

<br>

So, the goal is to find the **length** of the **shortest path** from **each** $`\nu_{i}`$ to **each** $`\nu_{j}`$ using **any** vertex in set $`\{1,2,…,n\}`$.<br>

Let $`p_{1k}=(\nu_{1}, ...,  \nu_{k})`$ is a path from $`\nu_{1}`$ to $`\nu_{k}`$.<br>
Let $`p_{ij}=(\nu_{i}, ...,  \nu_{j})`$ is a **part** of $`p_{1k}`$ and $`1 ≤ i ≤ j ≤ k`$.<br>

**Theorem**. If $`p_{1k}`$ is the **shortest path** from $`\nu_{1}`$ to $`\nu_{k}`$ then $`p_{ij}`$ is the **shortest path** from $`\nu_{i}`$ to $`\nu_{j}`$ too.<br>
**Proof**. If there is path that is shorter than $`p_{ij}`$ it means we can also reduce length of $`p_{1k}`$, but it leads to a **contradiction** with the statement that $`p_{1k}`$ is the **shortest path**.<br>

**Consider** *shortest path* $`d^{(k)}_{ij}`$. Add a vertex $`\nu_{k}`$ to the set of allowed intermediate vertices then resulting set is $`{\{\nu_{1},\nu_{2}, ..., \nu_{k}\}}`$.<br>

There are **2 cases**:
- $`case_1$: Vertex $`\nu_{k}`$ **doesn't** change **shortest path** $`d^{(k)}_{ij}`$ and it **doesn't** belong to $`d^{(k)}_{ij}`$, so $`d^{(k)}_{ij} = d^{(k-1)}_{ij}`$;
- $`case_2$: Vertex $`\nu_{k}`$ **changes shortest path** $`d^{(k)}_{ij}`$ and $`d^{(k)}_{ij}`$ includes $`\nu_{k}`$. what is new distance of new shortest path? $`\nu_{k}`$ splits new path into 2 subpaths: $`d_{ik}`$ and $`d_{kj}`$, and they are both **shortest paths**, so $`d^{(k)}_{ij} = d^{(k)}_{ik} + d^{(k)}_{kj}`$ and both $`d_{ik}`$ and $`d_{kj}`$ contain $`k`$ as **intermediate** vertex. It means, we can exclude $`k`$ from the **set of intemediate verteces** and then $`d^{(k)}_{ij} = d^{(k-1)}_{ik} + d^{(k-1)}_{kj}`$;


So, **shortest path** $`d^{(k)}_{ij}`$ must be **less than or equal to** $`d^{(k-1)}_{ij}`$.<br>

**Finally**: $`d^{(k)}_{ij} = min(case_1, case_2) = min(d^{(k-1)}_{ij}, d^{(k-1)}_{ik} + d^{(k-1)}_{kj})`$.<br>

<br>

The algorithm works by first computing $`d^{(k)}_{ij}`$ for all $`(i,j)`$ pairs for $`k=0`$, then $`k=1`$, then $`k=2`$ and so on up to $`k=n`$.<br>

<br>

#### Pseudocode
```rust
let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
for each edge (u, v) do
    dist[u][v] ← w(u, v)  // The weight of the edge (u, v)
for each vertex v do
    dist[v][v] ← 0
for k from 1 to |V|
    for i from 1 to |V|
        for j from 1 to |V|
            if dist[i][j] > dist[i][k] + dist[k][j] 
                dist[i][j] ← dist[i][k] + dist[k][j]
            end if
```

<br>

## Flows
### Ford–Fulkerson algorithm
The **Ford–Fulkerson algorithm** is a **greedy algorithm** that computes the maximum flow in a flow network.<br>
