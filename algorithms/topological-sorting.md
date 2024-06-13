# Table of contents
- [Table of contents](#table-of-contents)
- [Topological sorting](#topological-sorting)
- [Kahn's algorithm (BFS)](#kahns-algorithm-bfs)
  - [Idea of Kahn's algorithm](#idea-of-kahns-algorithm)
  - [Pseudocode](#pseudocode)
- [DFS based](#dfs-based)
  - [DFS based: color nodes](#dfs-based-color-nodes)
    - [Pseudocode](#pseudocode-1)
  - [DFS based: use stack](#dfs-based-use-stack)

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

# Kahn's algorithm (BFS)
## Idea of Kahn's algorithm
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

## Pseudocode
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

# DFS based
## DFS based: color nodes
The `dfs_tsort` procedure calls the `dfs(G)` which for **every** vertex $v$ computes **time** $v.f$ when the vertex was **finished**.<br>
The `dfs(G)` is **modified** version of [**original dfs**](https://github.com/carmenere/easy-cs/blob/main/algorithms/dsf-bsf.md#pseudocode-1) that **fails** if it visit **gray** vertex **twice**, because it means **cycle** exists in digraph.<br>
Then it `dfs_tsort` **sorts** verteces in **reverse order** by their $v.f$ values and the **result** contains the **topologically sorted vertices**.<br>

<br>

### Pseudocode
```rust
dfs_tsort(G)
    dfs(G)
    reverse_sort(G, key=v.f)  // Sort verteces in reverse order of their v.f and the result contains the topologically sorted vertices
```

<br>

## DFS based: use stack
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