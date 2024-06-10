# Table of contents
- [Table of contents](#table-of-contents)
- [Greedy algorithms](#greedy-algorithms)
- [Dynamic programming](#dynamic-programming)
- [Examples](#examples)
  - [Greedy algorithms](#greedy-algorithms-1)
  - [Dynamic algorithms](#dynamic-algorithms)

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



<br>

# Examples
## Greedy algorithms
- Prim's algorithm;
- Kruskal's algorithm;
- Dijkstra's algorithm;
- Ford–Fulkerson algorithm;

<br>

## Dynamic algorithms
- Bellman–Ford algorithm;
- Floyd–Warshall algorithm;

<br>