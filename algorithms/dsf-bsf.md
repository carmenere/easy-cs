# Table of contents
- [Table of contents](#table-of-contents)
- [Graph traversal](#graph-traversal)
  - [BFS traversal vs. DFS traversal](#bfs-traversal-vs-dfs-traversal)
  - [Edge types in graph traversal](#edge-types-in-graph-traversal)
- [BFS traversal](#bfs-traversal)
  - [Complexity](#complexity)
  - [Pseudocode](#pseudocode)
- [DFS traversal](#dfs-traversal)
  - [Complexity](#complexity-1)
  - [Pseudocode](#pseudocode-1)

<br>

# Graph traversal
There are 2 algorithms to traverse graphs:
- *BFS* traversal;
- *DFS* traversal;

<br>

## BFS traversal vs. DFS traversal
|  |DFS|BFS|
|:-|:--|:--|
|**Data structure**|**Stack**|**Queue**|
|**Edge types**|**Tree** and **back** edges|**Tree** and **cross** edges|
|**Worst-case time complexity**|$O(\|V\|+\|E\|)$|$O(\|V\|+\|E\|)$|
|**Worst-case space complexity**|$O(\|V\|)$|$O(\|V\|)$|

<br>

## Edge types in graph traversal
When **DFS** traverses a **graph**, it discovers a **set of non-intersecting trees** (aka **spanning forest of DFS**, **DFS-forest**).<br>

There are **4 types** in **graph traversal**:
- **tree edge** is a such *edge* $(u,v)$ for which vertex $v$ was first discovered, in other words, $v.π = u$ or $u$ is a predecessor of $v$;
- **forward edge** is an *edge* $(u,v)$ connecting a vertex $u$ to its **descendant** $v$.
- **back edge** is an *edge* $(u,v)$ connecting a vertex $u$ to its **ancestor** $v$. Existance of **back edge** indicates a **cycle**.
- **cross edge** is any other *edge*, in other words, **cross edges** connect two nodes which **don't** have any **ancestor** and a **descendant relationship** between them. 

<br>

> **Note**: depending on which vertex DFS is started for, the classification of edges may **change**. For example, some **back** edges may become **tree** edges and vice versa, and some **cross** edges may become **tree** edges.<br>

<br>

While *vertex* $u$ is being processed (**gray**):
- the *edge* $(u,w)$ is a **tree edge** if *vertex* $w$ is a **white**;
- the *edge* $(u,g)$ is a **back edge** if *vertex* $g$ is a **gray**;
- the *edge* $(u,b)$ is a **cross edge** if *vertex* $b$ is a **black**;

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

# BFS traversal
**BFS traversal** (**breadth first search traversal**) is an algorithm which systematically explores a graph $G=(V,E)$.<br>
It starts from a **starting vertex** $s$.<br>

*BFS* maintains **2 attributes** for every vertex:
- **predecessors** (or **parent**): $v.\pi$;
- **color**: $v.color$, possible values are **black**, **gray**, **white**;

<br>

**Initially**:
- for **every** vertex $v$:
  - $v.\pi = null$;
  - $v.color = white$;
- $s.color = gray$ ($s$ is a **starting vertex**);

<br>

The *BFS* **colors** vertices during it works:
- **each** *vertex* is **initially** colored to **white**;
- *vertex* is **recolored** to **gray** when it is **first discovered** during *DFS*;
- *vertex* is **recolored** to **black** when it is **finished**;

<br>

## Complexity
**Worst-case time complexity** is $O(|V| + |E|)$ because *BFS* visits **each** vertex exactly **once** and **reads its adjacency list** during the visit.<br>
**Worst-case space complexity** is $O(|V|)$ because at **worst** case *BFS* need to hold **all** vertices in the **queue**.<br>

<br>

## Pseudocode
The `bsf` procedure uses **queue** for **both trees** and **graphs**. The **queue** contains only **gray** nodes.<br>

```rust
bsf(G, s)
    for u in G.V\{s}                  // exclude vertex s from G.V
        u.color = white
    
    s.color = gray
    
    Q = empty queue
    enqueue(Q, s)

    while Q is not empty
        u = dequeue(Q)
        
        for v in G.Adj[u]             // G.Adj[i] contains adjacency list for vertex i
            if v.color == white
                v.color = gray
                v.π = u               // u is a predecessor of v
                enqueue(Q,v)
        
        u.color = black
```

<br>

# DFS traversal
**DFS traversal** (**depth first search traversal**) is an algorithm which systematically explores a graph $G=(V,E)$.<br>
It returns a **DFS forest** of $G$ and also computes **discovery-time** and **finish-time** for **each** vertex.<br>

<br>

*DFS* maintains **5 attributes** for every vertex:
- **start time** or **discovery time**: $v.s$ or $v.d$;
- **finish time**: $v.f$;
- **predecessor** (or **parent**): $v.\pi$;
- **color**: $v.color$, possible values are **black**, **gray**, **white**;
- **depth**: $v.depth$;

<br>

Vertices with *depth* **0** are called **root vertices**. When *DFS* **terminates**, **root vertices** have **no predecessor**, i.e. $v.\pi = null$.<br>

<br>

**Initially**, for **every** vertex $v$:
- $v.d = 0$;
- $v.f = 0$;
- $v.\pi = null$;
- $v.color = white$;
- $v.depth=0$;

<br>

*DFS* for *trees* **don't** require **mark** (or **color**), because *tree* **guarantees** that there is **no cycles** in tree.<br>
But **graph can have cycles** and to traverse them *DFS* need **mark** (or **color**) alredy **visited** nodes.<br>

<br>

The *DFS* **colors** vertices during it works:
- **each** *vertex* is **initially** colored to **white**;
- *vertex* is **recolored** to **gray** when it is **first discovered** during *DFS*;
- *vertex* is **recolored** to **black** when it is **finished**;

Node is **finished** means that its **adjacency list** has been **processed completely**.<br>

The *DFS* also assigns **sequence number** to **each vertex**. The **sequence number** represents the **logic timestamp** when vertex was **discovered** and **finished**.<br>

<br>

So, **each** vertex is assigned **two** t*imestamps* during *DFS*:
- the **first** *timestamps* $v.d$ is assigned to vertex when it is recolored to **gray**;
- the **second** *timestamps* $v.f$ is assigned to vertex when it is recolored to **black**;

<br>

These *timestamps* provide important information about the **structure** of the graph and can be used for **topological sorting**. But **instead** *timestamps* we can use **stack** to store **visited** nodes.<br>

<br>

## Complexity
**Worst-case time complexity** is $O(|V| + |E|)$ because *DFS* visits **each** vertex exactly **once** and **reads its adjacency list** during the visit.<br>

**Worst-case space complexity** is $O(|V|)$ because during *DFS*, the **function call stack** can have at most $|V|$ calls of `visit(G, u, d)`.<br>

For trees:
- for a **binary tree** it is $O(h)$, where $h$ is the **maximal depth** of **tree**;
- for a **balanced binary tree** it is $O(h)=O(log \kern3pt N)$;

<br>

## Pseudocode
```rust
dfs(G)
    for u in G.V
        u.color = white
    
    time = -1

    for u in G.V
        if u.color == white
            visit(G, u, 0)
```

<br>

```rust
visit(G, u, d)
    u.depth = d
    u.color = gray                //  white vertex u has just been discovered
    time = time + 1
    u.d = time

    for v in G.Adj[u]             // G.Adj[i] contains adjacency list for vertex i
        if v.color == white
            v.π = u               // u is a predecessor of v
            visit(G, v, d+1)
    
    u.color = black                //  white vertex u has just been finished
    time = time + 1
    u.f = time
```
