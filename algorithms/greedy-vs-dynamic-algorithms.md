# Table of contents
- [Table of contents](#table-of-contents)
- [Greedy algorithms](#greedy-algorithms)
- [Dynamic programming](#dynamic-programming)
- [Flow network](#flow-network)
- [Greedy algorithms](#greedy-algorithms-1)
  - [Prim's algorithm](#prims-algorithm)
  - [Kruskal's algorithm](#kruskals-algorithm)
  - [Dijkstra's algorithm](#dijkstras-algorithm)
  - [The Ford–Fulkerson algorithm](#the-fordfulkerson-algorithm)
- [Dynamic algorithms](#dynamic-algorithms)
  - [Bellman–Ford algorithm](#bellmanford-algorithm)
  - [Floyd–Warshall algorithm](#floydwarshall-algorithm)

<br>

# Greedy algorithms
A **greedy algorithm** is any algorithm that makes the **locally optimal decisions at each stage** optimistically assuming that the **final solution** will also be **optimal**.<br>

<br>

**Greedy algorithms** *typically* (but *not always*) **don't** find the **globally optimal solution** because they usually **don't** operate **exhaustively** on **all** the data. Nevertheless, they are **useful** because they are **quick** and **often** give **good approximations** to the optimum in a **reasonable** amount of time.<br>

Examples of problems for which greedy algorithms **don't** find an optimal solution:
- travelling salesman problem;
- clique problem;

<br>

For example, a greedy strategy for the **travelling salesman problem** (which is **transcomputational problem**) is the following heuristic /hjʊəˈrɪs.tɪk/: "At each step of the journey, visit the nearest unvisited city". This heuristic **doesn't** find the **best** solution, but it **terminates** in a **reasonable** number of steps.<br>

<br>

# Dynamic programming
There are two key attributes that a problem must have in order for **dynamic programming** to be applicable: **optimal substructure** and **overlapping sub-problems**.
If a problem **can** be solved by combining **optimal solutions** to **non-overlapping sub-problems**, the strategy is called **divide and conquer** instead. This is why *merge sort* and *quick sort* are **not** classified as *dynamic programming* problems.<br>

A **problem** is said to have **optimal substructure** if a **optimal solution** can be **constructed** from **optimal solutions** of its **subproblems**. In other words, the **combination** of many **local optimums** equals to the one **global optimum**.<br>
A **problem** is said to have **overlapping subproblems** if the problem can be **broken down** into **subproblems** which can be **reused several times**.<br>

For example, the problem of computing the **Fibonacci sequence** or **factorial function** has **overlapping subproblems**. The problem of computing the `n-th` *Fibonacci number* `F(n)`, can be broken down into the subproblems of computing `F(n − 1)` and `F(n − 2)`, and then adding the two. A **naive recursive approach** to such a problem generally fails due to an **exponential complexity**, cause it compute previous numbers `F(n − 1)` and `F(n − 2)` again and again. But **memoized version** of the **Fibonacci sequence** or **factorial function** has **linear complexity**.
follows: using **dynamic programming** we can **cash intermediate results** (aka **memoization**).

<br>

# Flow network
In graph theory, a **flow network** (also known as a **transportation network**) is a **directed graph** where each edge has a **capacity** and each edge receives a **flow**. The **amount of flow** on an edge **cannot** exceed the **capacity** of the edge.<br>
Often in operations research, a *directed graph* is called a **network**, the *vertices* are called **nodes** and the *edges* are called **arcs**.<br>
A flow must satisfy the restriction that the *amount of flow* **into** a node **equals** the *amount of flow* **out** of it, unless it is a source, which has only outgoing flow, or sink, which has only incoming flow.<br>

<br>

# Greedy algorithms
## Prim's algorithm
The **Prim's algorithm** is a **greedy algorithm** that finds a **minimum spanning tree** for a **edge-weighted** graph.

<br>

## Kruskal's algorithm
**Kruskal's algorithm** is a **greedy algorithm** that finds a **minimum spanning forest** of an undirected **edge-weighted** graph. If the graph is **connected**, it finds a **minimum spanning tree**.
Worst-case performance **O(|E|log|V|)**.

<br>

## Dijkstra's algorithm
**Dijkstra's algorithm** (/ˈdaɪkstrəz/ DYKE-strəz) finds the **shortest path** from a given **source** node to **every other node** in a **edge-weighted** *graph*.<br>
It can also be used to find the shortest path to a specific destination node, by terminating the algorithm once the shortest path to the destination node is know.<br>
With using a **Fibonacci heap priority queue** the **worst-case performance**: **O(|E| + |V|log|V|)**.<br>

*Dijkstra's algorithm* uses a **priority queue** to **greedily** select the **closest vertex** that has **not** yet been processed. So, Dijkstra's algorithm is a **greedy algorithm** because at each step, it selects the vertex with the smallest distance from the source vertex and adds it to the set of vertices that have been visited. This choice is made without considering the overall path or the global optimal solution.

The *Dijkstra's algorithm* is **greedy** because it makes **locally optimal choices at each step**.<br>

For example, if the nodes of the graph represent **cities**, and the costs of edges represent the average **distances** between pairs of cities connected by a direct road, then *Dijkstra's algorithm* can be used to find the **shortest route** between one city and all other cities.<br>

<br>

## The Ford–Fulkerson algorithm
The **Ford–Fulkerson algorithm** is a **greedy algorithm** that computes the maximum flow in a flow network.<br>

<br>

# Dynamic algorithms
## Bellman–Ford algorithm
The **Bellman–Ford algorithm** finds the **shortest path** from a given **source** vertex to **all** other vertices in a **edge-weighted** *digraph*.<br>
**Worst-case performance O(|V||E|)**.<br>

It is **slower** than *Dijkstra's algorithm*, but it can handle graphs in which some of the edge **weights** are **negative** numbers.<br>

The *Bellman-Ford algorithm* is **dynamic** because it uses a **dynamic programming approach** to compute the shortest paths by **solving subproblems** and iteratively updating the distances.<br>

<br>

## Floyd–Warshall algorithm
The **Floyd–Warshall algorithm** (aka the WFI algorithm) finds **shortest paths** between **all** pairs of vertices in a **edge-weighted** *digraph* with **positive** or **negative** edge weights (but with **no negative cycles**).<br>
**Worst-case** and **best-case performance**: **O(|V|<sup>3</sup>)**.<br>
**Worst-case space** complexity: **O(|V|<sup>2</sup>)**.<br>

The *Floyd–Warshall algorithm* is an example of **dynamic programming**.<br>