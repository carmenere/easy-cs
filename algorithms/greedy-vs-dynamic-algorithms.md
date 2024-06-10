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

**Greedy algorithms** *typically* (but *not always*) **don't** find the **globally optimal solution**. Nevertheless, they are **useful** because they are **quick** and **often** give **good approximations** to the optimum in a **reasonable** amount of time.<br>

The **change-making problem**: find the **minimum** number of **coins** (of certain denominations) to change $N$ cents.<br>

The **greedy algorithms** that solves this problem:
1. On every step $i$ it takes $x_{i}$ coins in denominations of $a_n$: $x_{i}=\lfloor N/a_{i}\rfloor$.
2. Run until $\sum x_{i} \ne N$.

<br>

Consider 2 examples:
1. Available coins are in denominations of `1`, `5`, `10` and `25` cents, aka **canonical coin system**. For **canonical coin systems** greedy algorithms **find** optimal result.
2. Available coins are in denominations of `1`, `5` and `7` cents. For such coin system greedy algorithms **doesn't** find optimal result. For $N=24$ greedy algorithm finds **3** coins in denominations of `7`, **3** coins in denominations of `1`. But the **right solution** is **2** coins in denominations of `7`, **2** coins in denominations of `5`.

<br>

Problems for which *greedy algorithms* **don't** find an optimal solution:
- travelling salesman problem;
- clique problem;

<br>

For example, a greedy strategy for the **travelling salesman problem** (which is **transcomputational problem**) is the following heuristic /hjʊəˈrɪs.tɪk/: "At each step of the journey, visit the nearest unvisited city". This heuristic **doesn't** find the **best** solution, but it **terminates** in a **reasonable** number of steps.<br>

<br>

# Dynamic programming
**Dynamic programming** is a method used to solve complex problems by **breaking** them down into **simpler** subproblems. By solving each subproblem **only once** and **storing the results**, it avoids redundant computations, leading to more efficient solutions for a wide range of problems.<br>

**When to use** *dynamic programming*? Dynamic programming is an optimization technique used when solving problems consists of the following characteristics:: **optimal substructure** and **overlapping sub-problems**.
If a problem **can** be solved by combining **optimal solutions** to **non-overlapping sub-problems**, the strategy is called **divide and conquer** instead. This is why *merge sort* and *quick sort* are **not** classified as *dynamic programming* problems.<br>

A **problem** is said to have **optimal substructure** if a **optimal solution** can be **constructed** from **optimal solutions** of its **subproblems**. In other words, the **combination** of many **local optimums** equals to the one **global optimum**.<br>
A **problem** is said to have **overlapping subproblems** if the problem can be **broken down** into **subproblems** which can be **reused several times**.<br>

<br>

**Approaches** of *dynamic programming*:
- **top-down approach** (aka **memoization**): algorithm starts with the **original** problem and **recursively break** it **down** into **smaller** subproblems. To avoid redundant calculations, algorithm **stores** the **results** of solved subproblems; **suitable** when the number of subproblems is **large** and many of them are **reused**;
- **bottom-up approach** (aka **tabulation**): algorithm starts with the **smallest** subproblems and **gradually build up** to the final solution of the **original** problem; **suitable** when the number of subproblems is **small** and the **global optimal solution** can be directly computed from the **local optimal solutions**;

<br>

For example, the problem of computing the **Fibonacci sequence** or **factorial function** has **overlapping subproblems**.<br>
A **naive recursive approach** of computing the **Fibonacci sequence**:
```rust
fn fib_naive(n: u32) -> u32 {
    if n == 0 {
        return 0
    }
    else if n == 1 {
        return 1
    }
    else {
        return fib_naive(n-1) + fib_naive(n-2)
    }
}
```

The `fib_naive` computes previous numbers `F(n − 1)` and `F(n − 2)` **again** and **again**.<br>
The **time complexity** is $O(2^n)$ and the **space complexity** is $O(n)$.<br>

The problem of computing the `n-th` *Fibonacci number* `F(n)`, can be broken down into the subproblems of computing `F(n − 1)` and `F(n − 2)`.<br>

A **top-down approach** of computing the **Fibonacci sequence** is `fib_memo`. The `fib_memo` uses a **memoized version** of `fib_naive`, i.e. `fib_memo` caches **all** intermediate results.<br>
The **time complexity** is $O(n)$ and the **space complexity** is $O(n)$.<br>

A **bottom-up approach** of computing the **Fibonacci sequence** is `fib_tab`. The `fib_tab` caches only **two** previous results.<br>
The **time complexity** is $O(n)$ and the **space complexity** is $O(1)$.<br>

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