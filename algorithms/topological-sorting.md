# Topological sorting
A **topological sort** is a **graph traversal** in which each node $v$ is **visited** only **after** **all** its **dependencies** are **visited**.<br>
A *topological ordering* is **possible** iif the graph has **no** *directed cycles*, that is, if it is a **DAG** (directed acyclic graph).<br>

<br>

There are 2 algorithms:
1. **Kahn's algorithm** uses queue;
2. **Tarjan’s algorithm** - uses DFS;

<br>

The **worst-case time complexity** of both algorithms is ${\displaystyle O(\left|{V}\right|+\left|{E}\right|)}$.<br>

<br>

# Kahn's algorithm
Given graph ${G=(V,E)}$. Let $`{A(v),v\in V}`$ denotes the set of vetices from which **exist** edges to $v$. Let $L$ is a list that will contain the sorted elements.<br>

$A(v)=∅$ for node with **no incoming** edge.<br>

<br>

## Pseudocode
### Variant 1
```rust
while |L| < |V|
    select any v, such that A(v)=∅ AND v∉L
    add v to L
    remove v from all A(u) where u != v
```

- if the graph is a **DAG**, the $L$ will contain the sorted elements;
- if graph contains **at least one cycle** this code `take any v, such that A(v)=∅ AND v∉L` will **broke** at some iteration and algoirthm terminates;

<br>

### Variant 2
```rust
L ← Empty list that will contain the sorted elements
S ← Set of all nodes with no incoming edge

while S is not empty do
    remove a node n from S
    add n to L
    for each node m with an edge e from n to m do
        remove edge e from the graph
        if m has no other incoming edges then
            insert m into S

if graph has edges then
    return error   (graph has at least one cycle)
else 
    return L   (a topologically sorted order)
```

<br>

## Example
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

# Tarjan’s algorithm

### Pseudocode
```rust
L ← Empty list that will contain the sorted nodes

while exists nodes without a permanent_mark do
    select an unmarked node n
    visit(n)

function visit(node n)
    if n has a permanent_mark then
        return
    
    if n has a temporary_mark then
        Error("graph has at least one cycle")

    mark n with a temporary_mark

    for each node m with an edge from n to m do
        visit(m)

    remove temporary_mark from n
    mark n with a permanent_mark
    add n to head of L
```