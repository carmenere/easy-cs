# Table of contents
- [Table of contents](#table-of-contents)
- [BFS](#bfs)
  - [Notes about BFS](#notes-about-bfs)
  - [Pseudocode](#pseudocode)
- [DFS](#dfs)
  - [Notes about DFS](#notes-about-dfs)
  - [Pseudocode](#pseudocode-1)

<br>

# BFS
**Best-case and worst-case time complexity** is $O(|V|)$, where $|V|$ is the number of nodes. You need to traverse all nodes.<br>
**Worst-case space complexity** is $O(|V|)$ since at **worst** case you need to hold all vertices in the **queue**.<br>

<br>

## Notes about BFS
The `bfs` procedure recieves **starting node** `s` (it is **initially gray**). The `BFS` uses **queue** for **both** **trees** and **graphs**.<br>
The `BFS` **colors** vertices during the search to indicate their state. **Each vertex** is initially **white**, is **gray** when it is **discovered**, and is **black** when it is **finished**. Node is **finished** means that its adjacency list has been **examined completely**.<br>

The **queue** contains only **gray** nodes.<br>

<br>

## Pseudocode
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

Trees **don't** require *stack* or *additional operations* over nodes, because tree **guarantees** that there is **no cycles** in tree.<br>
But **graph can have cycles** and to traverse them we need **mark** (or **color**) alredy **visited** nodes.<br>

<br>

## Notes about DFS
The `DFS` **colors** vertices during the search to indicate their state. **Each vertex** is initially **white**, is **gray** when it is **discovered**, and is **black** when it is **finished**. Node is **finished** means that its adjacency list has been **examined completely**.<br>

The `DFS` also assign **sequence number** to **each vertex**. Each vertex has **two** *sequence numbers*:
- the **first** sequence number $v.d$ is assigned to vertex when it is colored to **gray**;
- the **second** sequence number $v.f$ is assigned to vertex when it is colored to **black**;

<br>

These *sequence numbers* provide important information about the **structure** of the graph and can be used for **topological sorting**.<br>
But **instead** *sequence numbers* we can use **stack** to store **visited** nodes.<br>

<br>

## Pseudocode
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
