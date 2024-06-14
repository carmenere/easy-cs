# Table of contents
- [Table of contents](#table-of-contents)
- [Spanning trees](#spanning-trees)
  - [Prim's algorithm](#prims-algorithm)
  - [Kruskal's algorithm](#kruskals-algorithm)
- [Shortest paths](#shortest-paths)
  - [Dijkstra's algorithm](#dijkstras-algorithm)
    - [Algorithm](#algorithm)
  - [Bellman–Ford algorithm](#bellmanford-algorithm)
  - [Floyd–Warshall algorithm](#floydwarshall-algorithm)
    - [Algorithm](#algorithm-1)
    - [Pseudocode](#pseudocode)
- [Flows](#flows)
  - [Ford–Fulkerson algorithm](#fordfulkerson-algorithm)
- [Finding all strongly connected components](#finding-all-strongly-connected-components)
  - [Kosaraju’s algorithm](#kosarajus-algorithm)
    - [Idea of Kosaraju’s algorithm](#idea-of-kosarajus-algorithm)
    - [Why Kosaraju’s algorithm works?](#why-kosarajus-algorithm-works)
  - [Tarjan’s algorithm](#tarjans-algorithm)
- [Topological sorting](#topological-sorting)
  - [Kahn's algorithm (BFS)](#kahns-algorithm-bfs)
    - [Idea of Kahn's algorithm](#idea-of-kahns-algorithm)
    - [Pseudocode](#pseudocode-1)
  - [DFS based](#dfs-based)
    - [DFS based: color nodes](#dfs-based-color-nodes)
      - [Pseudocode](#pseudocode-2)
    - [DFS based: use stack](#dfs-based-use-stack)

<br>

# Spanning trees
## Prim's algorithm
The **Prim's algorithm** is a **greedy algorithm** that finds a **minimum spanning tree** for a **edge-weighted** graph.

<br>

## Kruskal's algorithm
**Kruskal's algorithm** is a **greedy algorithm** that finds a **minimum spanning forest** of an undirected **edge-weighted** graph. If the graph is **connected**, it finds a **minimum spanning tree**.
Worst-case performance **O(|E|log|V|)**.

<br>

# Shortest paths
## Dijkstra's algorithm
**Dijkstra's algorithm** (/ˈdaɪkstrəz/) finds the **shortest path** from a given **source node** to **every other node** in a **edge-weighted** *graph*.<br>
It can also be used to find the shortest path to a specific destination node, by terminating the algorithm once the shortest path to the destination node is know.<br>
With using a **Fibonacci heap priority queue** the **worst-case performance**: **O(|E| + |V|log|V|)**.<br>

*Dijkstra's algorithm* is just **BSF** that uses a **priority queue** instead **regular queue**. It **greedily** selects the **nearest vertex** that has **not** visited yet. So, Dijkstra's algorithm is a **greedy algorithm** because at each step, it selects the vertex with the smallest distance from the source vertex and adds it to the set of vertices that have been visited. This choice is made without considering the overall path or the global optimal solution.

The *Dijkstra's algorithm* is **greedy** because it makes **locally optimal choices at each step**.<br>

For example, if the nodes of the graph represent **cities**, and the costs of edges represent the average **distances** between pairs of cities connected by a direct road, then *Dijkstra's algorithm* can be used to find the **shortest route** between one city and all other cities.<br>

<br>

### Algorithm
1. Mark **all** nodes as **unvisited**.
2. Assign to **every** node an **initial distance**: **zero** for the **starting node** and $`+\infty`$ for **all other nodes**.
3. Add **all** nodes to **priority queue**.
4. Remove from the **queue** next **unvisited** with the **smallest distance** node. It is **current node**.
5. Calculate `new_distance` for **every** child node of current node: `new_distance` = `min(distance_of_current_node + weight_of_edge_to_child, distance_of_child_node)` and assign it to child.
6. **Repeat** steps `4` and `5` while queue is **not** empty.

<br>

## Bellman–Ford algorithm
The **Bellman–Ford algorithm** finds the **shortest path** from a given **source** vertex to **all** other vertices in a **edge-weighted** *digraph*.<br>
**Worst-case performance O(|V||E|)**.<br>

It is **slower** than *Dijkstra's algorithm*, but it can handle graphs in which some of the edge **weights** are **negative** numbers.<br>

The *Bellman-Ford algorithm* is **dynamic** because it uses a **dynamic programming approach** to compute the shortest paths by **solving subproblems** and iteratively updating the distances.<br>

<br>

## Floyd–Warshall algorithm
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
- $`{d^{(0)}_{ij}}`$ is just a **weight** ($`\omega_{ij}`$) of **edge** ($`\varepsilon_{ij}`$) - the **base case**;
- $`{d^{(1)}_{ij}}`$ is a **shortests distance** of **path** that pass through **vertex** $`\nu_{1}`$;
- $`{d^{(2)}_{ij}}`$ is a **shortests distance** of **path** that pass through **vertices** $`{\{\nu_{1},\nu_{2}\}}`$;
- ...
- $`{d^{(k-1)}_{ij}}`$ is a **shortests distance** of **path** that pass through **vertices** $`{\{\nu_{1},\nu_{2}, ..., \nu_{k-1}\}}`$;
- $`{d^{(k)}_{ij}}`$ is a **shortests distance** of **path** that pass through **vertices** $`{\{\nu_{1},\nu_{2}, ..., \nu_{k}\}}`$;

<br>

Let $`p_{1k}=(\nu_{1}, ...,  \nu_{k})`$ is a path from $`\nu_{1}`$ to $`\nu_{k}`$.<br>
Let $`p_{ij}=(\nu_{i}, ...,  \nu_{j})`$ is a **part** of $`p_{1k}`$ and $`1 ≤ i ≤ j ≤ k`$.<br>

**Theorem**. If $`p_{1k}`$ is the **shortest path** from $`\nu_{1}`$ to $`\nu_{k}`$ then $`p_{ij}`$ is the **shortest path** from $`\nu_{i}`$ to $`\nu_{j}`$ too.<br>
**Proof**. If there is path that is shorter than $`p_{ij}`$ it means we can also reduce length of $`p_{1k}`$, but it leads to a **contradiction** with the statement that $`p_{1k}`$ is the **shortest path**.<br>

**Consider** *shortest path* $`d^{(k)}_{ij}`$. Add a vertex $`\nu_{k}`$ to the set of allowed intermediate vertices then resulting set is $`{\{\nu_{1},\nu_{2}, ..., \nu_{k}\}}`$.<br>

There are **2 cases**:
- $`case_1`$: Vertex $`\nu_{k}`$ **doesn't** change **shortest path** $`d^{(k)}_{ij}`$ and it **doesn't** belong to $`d^{(k)}_{ij}`$, so $`d^{(k)}_{ij} = d^{(k-1)}_{ij}`$;
- $`case_2`$: Vertex $`\nu_{k}`$ **changes shortest path** $`d^{(k)}_{ij}`$ and $`d^{(k)}_{ij}`$ includes $`\nu_{k}`$. what is new distance of new shortest path? $`\nu_{k}`$ splits new path into 2 subpaths: $`d_{ik}`$ and $`d_{kj}`$, and they are both **shortest paths**, so $`d^{(k)}_{ij} = d^{(k)}_{ik} + d^{(k)}_{kj}`$ and both $`d_{ik}`$ and $`d_{kj}`$ contain $`k`$ as **intermediate** vertex. It means, we can exclude $`k`$ from the **set of intemediate verteces** and then $`d^{(k)}_{ij} = d^{(k-1)}_{ik} + d^{(k-1)}_{kj}`$;


So, **shortest path** $`d^{(k)}_{ij}`$ must be **less than or equal to** $`d^{(k-1)}_{ij}`$.<br>

**Finally**: $`d^{(k)}_{ij} = min(case_1, case_2) = min(d^{(k-1)}_{ij}, d^{(k-1)}_{ik} + d^{(k-1)}_{kj})`$.<br>

<br>

### Algorithm
The algorithm works by first computing $`d^{(k)}_{ij}`$ for all $`(i,j)`$ pairs for $`k=0`$, then $`k=1`$, then $`k=2`$ and so on up to $`k=n`$.<br>

If there is **no** path from $`\nu_{i}`$ to $`\nu_{j}`$ for some $k$ then $`{d^{(k)}_{ij}}=+\infty`$.<br>

Consider graph:<br>
![floyd-warshall](/img/floyd-warshall.png)

**Steps**:
- $k=0$:
  - $`{d^{(0)}_{AC}}=1`$
  - $`{d^{(0)}_{BC}}=2`$
  - ...
  - $`{d^{(0)}_{AB}}=+\infty`$
  - $`{d^{(0)}_{AD}}=+\infty`$
- $k=1$:
  - $`{d^{(1)}_{AC}}=1`$
  - $`{d^{(1)}_{BC}}=2`$
  - ...
  - $`{d^{(1)}_{AB}}={d^{(0)}_{AC}} + {d^{(0)}_{BC}}=1+2=3`$
  - $`{d^{(1)}_{AD}}={d^{(0)}_{AC}} + {d^{(0)}_{DC}}=1+3=4`$
- ...

<br>

### Pseudocode
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

# Flows
## Ford–Fulkerson algorithm
The **Ford–Fulkerson algorithm** is a **greedy algorithm** that computes the maximum flow in a flow network.<br>

<br>

# Finding all strongly connected components
**Kosaraju's algorithm** for **strongly connected components** (**SCC**) applies *DFS* **twice**, *first* time to the **given graph** and a *second* time to its **transpose graph**. **Time Complexity**: $O(V+E)$ + $O(V+E)$ + $O(V+E)$ ~ $O(V+E)$.<br>
**Tarjan’s algorithm** for **strongly connected components** (**SCC**) applies *DFS* **once**. **Time Complexity**: $O(V+E)$.<br>

<br>

## Kosaraju’s algorithm
### Idea of Kosaraju’s algorithm
1. Perform a **DFS traversal** to define the **priorities** of the vertices (**finish time** of of the vertices). Push the vertices onto a **stack** during *DFS* according to the **priorities**. 
2. Build the **transpose** $`G^T`$ of G.
3. Perform **DFS traversal** on $`G^T`$. **Each invocation** of *DFS* finds **one** *SCC*. So, the number of times we invoke *DFS* on $`G^T`$ is **equal to** the number of *SCCs* in $G$. In **each** invokation the *DFS* is strated on **non-visited node** with the **highest priority**.

<br>

For **each vertex** $v$ algorithm **keeps track** of **exit time** $v.out$. These **exit times** have a **key role** in an algorithm.<br>
Let define **exit time** $C.out$ from the **SCC** $C$ as **maximum** of **all** values $v.out$ such that $v \in C$.<br>
Let define **entry times** $C.in$ into the **SCC** $C$ as **minimum** of **all** values $v.in$ such that $v \in C$.<br>

**Theorem**. Let $C$ and $C'$  are **two different** *SCC* and there is an **edge** $(C, C')$ in a **condensation graph**. Then $C.out > C'.out$.<br>

<br>

Given graph:
![condensation-1](/img/condensation-1.png)

<br>

For example, we start the process from `vertex 1` (but we could start from any node).<br>

**After step 1**:
![condensation-4](/img/condensation-4.png)

The **priorities** are **red** numbers.<br>
The *sequence* of **vertices** (not priorities) in the **stack** is: $[3, 4, 7, 8, 6, 5, 2, 1].$.

<br>

**After step 2**:
![condensation-5](/img/condensation-5.png)

<br>

**After step 3**:
![condensation-6](/img/condensation-6.png)

<br>

### Why Kosaraju’s algorithm works?
The **most important property** of the *condensation graph*:
- *condensation graph* is **DAG**;
- if $SCC$ is **source** in *condensation* of $G$ then it *becomes* **sink** in *condensation* of $`G^T`$;
- if $SCC$ is **sink** in *condensation* of $G$ then it *becomes* **source** in *condensation* of $`G^T`$;

<br>

Let *SCC* $C1$ has the **highest** value of **exit time** ($C1.out$).<br>
Let $C1.out > C2.out$.<br>

It means that $C1$ is the **source** *SCC* in *condensation* of $G$. So, $C1$ becomes **sink** in *condensation* of $`G^T`$ and it will be traversed **fisrt**, because it has **highest priority**. 
In following invocations of *DFS* (for example for $C2$) it will **never** enter into $C1$, because $C1$ **was traversed** and all its vertices are **black** (**completely finished**).

<br>

## Tarjan’s algorithm

<br>

# Topological sorting
A **topological sort** is a **graph traversal** in which each node $v$ is **visited** only **after all** its **dependencies** are **visited**.<br>
A *topological ordering* is possible **iif** the graph has **no** *directed cycles*, that is, if it is a **DAG** (directed acyclic graph).<br>

<br>

There are 2 algorithms:
1. **Kahn's algorithm** uses queue;
2. **Tarjan’s algorithm** - uses DFS;

<br>

The **worst-case time complexity** of both algorithms is ${\displaystyle O(\left|{V}\right|+\left|{E}\right|)}$.<br>

<br>

## Kahn's algorithm (BFS)
### Idea of Kahn's algorithm
Given graph ${G=(V,E)}$.<br>
Let $`{A(v),v\in V}`$ denotes the set of vetices from which **exist** edges to $v$.<br>
Let $L$ is a list that will contain the **sorted** elements.<br>

$A(v)=∅$ for node with **no incoming** edge.<br>

```rust
while |L| < |V|
    select any v, such that A(v)=∅ AND v∉L
    add v to L
    remove v from all A(u) where u != v
```

- if the graph is a **DAG**, the $L$ will contain the **sorted** elements;
- if graph contains **at least one cycle** this code `take any v, such that A(v)=∅ AND v∉L` will **broke** at some iteration and algoirthm **terminates**;

<br>

Given graph:<br>
![tsort-example-1](/img/tsort-example-1.png)

<br>

**Steps**:<br>
|Step|Selected vertex $v$|$A(a)$|$A(b)$|$A(c)$|$A(d)$|$A(e)$|L|
|:---|:------------------|:-----|:-----|:-----|:-----|:-----|:-|
|0|-|$∅$|$a$|$a$|$a,b,c$|$a,c,d$|$∅$|
|1|$a$|$∅$|$∅$|$∅$|$b,c$|$c,d$|$a$|
|2|$c$|$∅$|$a$|$a$|$b$|$d$|$a,c$|
|3|$b$|$∅$|$a$|$a$|$∅$|$d$|$a,c,b$|
|4|$d$|$∅$|$a$|$a$|$∅$|$∅$|$a,c,b,d$|
|5|$e$|$∅$|$a$|$a$|$∅$|$∅$|$a,c,b,d,e$|

<br>

### Pseudocode
The `tsort_bsf` procedure returns **queue** $R$ that contains the **topologically sorted vertices**.<br>

```rust
tsort_bsf(G)
    R = empty list of topologically sorted vertices
    Q = empty queue

    // Loop below inits a queue Q with all vertices of in-degree 0
    for each vertex u in G.V
        u.indegree = 0
        for each vertex v in G.Adj[u]     // G.Adj[i] contains adjacency list for vertex i
            u.indegree = u.indegree + 1   // Count indegree for every node.
        if u.indegree == 0
            enqueue(Q, u)                 // Put to Q nodes with indegree equal to 0 (nodes with no incoming edge)
    
    while Q is not empty
        u = dequeue(Q)

        add u to the end of R

        for each vertex v in G.Adj[u]     // G.Adj[i] contains adjacency list for vertex i
            v.indegree = v.indegree - 1
            if v.indegree == 0
                enqueue(Q, u)             // Put to Q nodes with indegree equal to 0 (nodes with no incoming edge)
        
    if len(R) != len(G.V)
        Error("Digraph contains cycle: not all verteces are processed and we cannot take next vertex with indegree equal to 0.")
    
    return R
```

<br>

## DFS based
### DFS based: color nodes
The `dfs_tsort` procedure calls the `dfs(G)` which for **every** vertex $v$ computes **time** $v.f$ when the vertex was **finished**.<br>
The `dfs(G)` is **modified** version of [**original dfs**](https://github.com/carmenere/easy-cs/blob/main/algorithms/dsf-bsf.md#pseudocode-1) that **fails** if it visit **gray** vertex **twice**, because it means **cycle** exists in digraph.<br>
Then it `dfs_tsort` **sorts** verteces in **reverse order** by their $v.f$ values and the **result** contains the **topologically sorted vertices**.<br>

<br>

#### Pseudocode
```rust
dfs_tsort(G)
    dfs(G)
    reverse_sort(G, key=v.f)  // Sort verteces in reverse order of their v.f and the result contains the topologically sorted vertices
```

<br>

### DFS based: use stack
When vertex is **visited** we add it to **stack**. At the end, **reverse** the **stack** and we get **topologically sorted vertices** as the **result**.<br>

**Step 1** (function call for the `node 0`):
- **mark** the current `node 0` as **visited**;
- visit next node connected to `node 0` that still **unvisited**;
- visit `node 2`;

![tsort-dfs-stack-1](/img/tsort-dfs-stack-1.png)

**Step 2** (function call for the `node 2`):
- **mark** the current `node 2` as **visited**;
- visit the nodes connected to `node 2` that are marked **unvisited**;

![tsort-dfs-stack-2](/img/tsort-dfs-stack-2.png)

**Step 3** (function call for the `node 4`):
- **mark** the current `node 4` as **visited**;
- **all** the nodes connected to `node 4` have already been **visited**;
- **Push** `node 4` into the **stack** and return to the function call of `node 2`;

![tsort-dfs-stack-3](/img/tsort-dfs-stack-3.png)

**Step 4** (function call for the `node 2`):
- **all** the nodes connected to `node 2` have already been **visited**;
- **Push** `node 2` into the **stack** and return to the function call of `node 0`;

![tsort-dfs-stack-4](/img/tsort-dfs-stack-4.png)

**Step 5** (function call for the `node 0`):
- visit next node connected to `node 0` that still **unvisited**;
- visit `node 5`;

![tsort-dfs-stack-5](/img/tsort-dfs-stack-5.png)

**Step 6** (function call for the `node 5`):
- **mark** the current `node 5` as **visited**;
- **all** the nodes connected to `node 5` have already been **visited**;
- **Push** `node 5` into the **stack** and return to the function call of `node 0`;

![tsort-dfs-stack-6](/img/tsort-dfs-stack-6.png)

**Step 7** (function call for the `node 0`):
- **all** the nodes connected to `node 0` have already been **visited**;
- **Push** `node 0` into the **stack** and return to the function call of `node 0`;

![tsort-dfs-stack-7](/img/tsort-dfs-stack-7.png)

...

**Step 12** (function call for `node 1`):
- **all** the nodes connected to `node 1` are **visited**;
- **Push** `node 1` into the **stack** and return;

![tsort-dfs-stack-12](/img/tsort-dfs-stack-12.png)