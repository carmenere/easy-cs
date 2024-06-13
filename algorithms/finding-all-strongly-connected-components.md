# Table of contents
- [Table of contents](#table-of-contents)
- [Finding all strongly connected components](#finding-all-strongly-connected-components)
  - [Kosaraju’s algorithm](#kosarajus-algorithm)
  - [Tarjan’s algorithm](#tarjans-algorithm)

<br>

# Finding all strongly connected components
**Kosaraju's algorithm** for **strongly connected components** (**SCC**) applies *DFS* **twice**, *first* time to the **given graph** and a *second* time to its **transpose graph**. **Time Complexity**: $O(V+E)$ + $O(V+E)$ + $O(V+E)$ ~ $O(V+E)$.<br>
**Tarjan’s algorithm** for **strongly connected components** (**SCC**) applies *DFS* **once**. **Time Complexity**: $O(V+E)$.<br>

<br>

## Kosaraju’s algorithm
1. In the first step, we perform a **DFS traversal** to define the **priorities** of the vertices. For each vertex, we remember when DFS finishes processing. We push the vertices onto a stack during **DFS traversal** according to the **priorities**. 
2. In the second step, we build the **transpose** $`G^T`$ of G.
3. In the third step, we perform a **DFS traversal** on $`G^T`$. Each DFS invocation finds one SCC. So, the number of SCCs is equal to the number of times we invoke DFS on TG. Each time we call a DFS procedure, we use the non-visited node with the highest priority.

<br>

For **each vertex** $v$ algorithm **keeps track** of **exit time** $v.out$. These **exit times** have a **key role** in an algorithm.<br>
Let define **exit time** $C.out$ from the **SCC** $C$ as **maximum** of **all** values $v.out$ such that $v \in C$.<br>
Let define **entry times** $C.in$ into the **SCC** $C$ as **minimum** of **all** values $v.in$ such that $v \in C$.<br>

<br>

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

## Tarjan’s algorithm

<br>
