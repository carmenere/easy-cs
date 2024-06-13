# Table of contents
- [Table of contents](#table-of-contents)
- [Edge types in graph traversal](#edge-types-in-graph-traversal)
- [BFS vs. DFS](#bfs-vs-dfs)
- [BFS](#bfs)
  - [Pseudocode](#pseudocode)
- [DFS](#dfs)
  - [Pseudocode](#pseudocode-1)

<br>

# Edge types in graph traversal
When **DFS** traverses a **graph**, it discovers a **set of non-intersecting trees** (aka **spanning forest of DFS**, **DFS-forest**).<br>

There are **4 types** in **graph traversal**:
- **Tree edge**: it is an *edge* which is **present** in the **tree** obtained after applying DFS on the graph (**DFS-forest**).
- **Forward edge**: it is an *edge* $(u, v)$ such that $v4 is a **descendant** but **not** part of the **DFS-forest**.
- **Back edge**: it is an *edge* $(u, v)$ such that $v$ is the **ancestor** of node $u$ but is **not** part of the **DFS-forest**. **Presence** of **back edge** indicates a **cycle** in **directed graph**.
- **Cross edge**: it is an *edge* that **connects two nodes** such that they **don't** have any **ancestor** and a **descendant relationship** between them. 

<br>

> **Note**: depending on which vertex we start DFS for, the classification of edges may change. For example, some back edges may become tree edges and vice versa, and some cross edges may become tree edges.<br>

<br>

While **vertex** $v$ is being processed and *DFS* **discovers**:
- an **unvisited** **adjacent** *vertex*, $u$, then *edge* $(v, u)$ is a **tree edge**;
- a **visited** but **not** yet **fully processed** **adjacent** *vertex*, $w$, then *edge* $(v, w)$ is a **back edge**;
- a **fully processed** **adjacent** *vertex*, $t$, then *edge* $(v, t)$ is a **cross edge**;

<br>

Consider **digraph**:<br>
![edge-types-dfs-1](/img/edge-types-dfs-1.png)

<br>

The **DFS-forest** of the digraph after DFS is completed:<br>
![edge-types-dfs-2](/img/edge-types-dfs-2.png)

- the **tree edges** are depicted with **solid lines**;
- the **back edges** depicted with **dashed lines**;
- the **cross edges** depicted with **dotted lines**;

<br>

The **DFS-forest** of the graph consists of two trees:
- The **first tree** consists of the set of **vertices** $`\{A, B, C, D, E, F\}`$ and the set of **edges** $`\{(A, B), (B, C), (B, D), (D, E), (E, F)\}`$;
- The **second tree** consists of the set of **vertices** $`\{G, H, I, J\}`$ and the set of **edges** $`\{(G, H), (H, I), (I, J)\}`$;

<br>

# BFS vs. DFS
|  |DFS|BFS|
|:-|:--|:--|
|**Data structure**|**Stack**|**Queue**|
|**Edge types**|**Tree** and **back** edges|**Tree** and **cross** edges|
|Efficiency for **adjacency matrix**|$`O(\|V\|^2)`$|$`O(\|V\|^2)`$|
|Efficiency for **adjacency lists**|$O(\|V\|+\|E\|)$|$O(\|V\|+\|E\|)$|

<br>

# BFS
**Best-case and worst-case time complexity** is $O(|V|)$, where $|V|$ is the number of nodes. You need to traverse all nodes.<br>
**Worst-case space complexity** is $O(|V|)$ since at **worst** case you need to hold all vertices in the **queue**.<br>

<br>

## Pseudocode
The `bfs` procedure recieves **starting node** `s` (it is **initially gray**). The `BFS` uses **queue** for **both** **trees** and **graphs**.<br>
The `BFS` **colors** vertices during the search to indicate their state. **Each vertex** is initially **white**, is **gray** when it is **discovered**, and is **black** when it is **finished**. Node is **finished** means that its adjacency list has been **examined completely**.<br>

The **queue** contains only **gray** nodes.<br>

```rust
bsf(G, s)
    for each vertex u in G.V\{s} // exclude vertex s from G.V
        u.color = white
    s.color = gray
    
    Q = empty queue
    enqueue(Q, s)

    while Q is not empty
        u = dequeue(Q)
        
        for each vertex v in G.Adj[u] // G.Adj[i] contains adjacency list for vertex i
            if v.color == white
                v.color = gray
                enqueue(Q,v)
        
        u.color = black
```

<br>

# DFS
**Best-case and worst-case time complexity** is $O(|V|)$, where $|V|$ is the number of nodes. You need to traverse all nodes.<br>

**Space complexity** - depends on the implementation:
- **recursive implementation** has **worst-case space complexity** $O(h)$, where $h$ is the **maximal depth** of **tree**. For a **balanced tree** $h=log \kern3pt N$;
- **stack based implementation** of DFS has **worst-case space complexity** $O(|V|)$;

Trees **don't** require **mark** (or **color**), because tree **guarantees** that there is **no cycles** in tree.<br>
But **graph can have cycles** and to traverse them we need **mark** (or **color**) alredy **visited** nodes.<br>

<br>

## Pseudocode
The `DFS` **colors** vertices during the search to indicate their state. **Each vertex** is initially **white**, is **gray** when it is **discovered**, and is **black** when it is **finished**. Node is **finished** means that its adjacency list has been **examined completely**.<br>

The `DFS` also assign **sequence number** to **each vertex**. Each vertex has **two** *sequence numbers*:
- the **first** sequence number $v.d$ is assigned to vertex when it is colored to **gray**;
- the **second** sequence number $v.f$ is assigned to vertex when it is colored to **black**;

<br>

These *sequence numbers* provide important information about the **structure** of the graph and can be used for **topological sorting**.<br>
But **instead** *sequence numbers* we can use **stack** to store **visited** nodes.<br>

<br>

```rust
dfs(G)
    for each vertex u in G.V
        u.color = white
    seq_number = 0
    for each vertex u in G.V
        if u.color == white
            visit(G, u)
```

<br>

```rust
visit(G, u)
    u.color = gray                //  white vertex u has just been discovered
    time = time + 1
    u.d = time

    for each vertex v in G.Adj[u] // G.Adj[i] contains adjacency list for vertex i
        if v.color == white
            visit(G,v)
    
    u.color = black                //  white vertex u has just been finished
    time = time + 1
    u.f = time
```

<br>
