# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Data-flow analysis](#data-flow-analysis)
* [Liveliness analysis](#liveliness-analysis)
  * [Uses and Defs](#uses-and-defs)
* [UD and DU chains](#ud-and-du-chains)
* [Computing liveness](#computing-liveness)
  * [Backtracking algorithm](#backtracking-algorithm)
  * [Algorithm that solves data-flow equations](#algorithm-that-solves-data-flow-equations)
* [Static Single Assignment](#static-single-assignment)
  * [Relation to dominator trees](#relation-to-dominator-trees)
<!-- TOC -->

<br>

# Data-flow analysis
**Data-flow analysis** is the process of collecting information about the way the variables are **defined** and **used** in the program.<br>
**Data-flow analysis** forms the foundation for a wide variety of compiler **optimizations** and program **verification techniques**.<br>

<br>

# Liveliness analysis
A _variable_ is **live** _at particular point_ in the program if it holds a **value** that will be used in the **future**, otherwise _variable_ is **dead**.<br>

_CFG_ in which every **basic block** (**node**) is a **single statement** is usually used for **liveness analysis**.<br>

<br>

**Definitions**:
- _CFG_ node has **in-edges** coming **from predecessor** nodes;
- _CFG_ node has **out-edges** leading **to successor** nodes;
- $`pred[n]`$ is the **set** of **all predecessors** for node $`n`$;
- $`succ[n]`$ is the **set** of **all successors** for node $`n`$;
- a _variable_ is **live-in** at node $`n`$ if it's live on any **in-edges** of node $`n`$;
- a _variable_ is **live-out** at node $`n`$ if it's live on any **out-edges** of node $`n`$;
- $`in[n]`$ is the **set** of **all live-in** _variables_ of a node $`n`$;
- $`out[n]`$ is the **set** of **all live-out** _variables_ of a node $`n`$;

<br>

**Example**:<br>
![liveness_1](/img/liveness-1.png)

In the example above:
- statements $`3`$ and $`4`$ **use** $`b`$, so $`b`$ is **live on edges** $`2 \rightarrow 3`$ and $`3 \rightarrow 4`$;
- statement $`2`$ **assigns** $`b`$ new value, so $`b`$ is **dead on edges** $`1 \rightarrow 2`$ and $`5 \rightarrow 2`$, because any value of $`b`$ on these edges is **not** needed;
- $`succ[5] = {2,6}`$;
- $`pred[5] = {4}`$;
- $`pred[2] = {1,5}`$;

<br>

**Formal definition of liveness**: a variable $`v`$ is **live on** CFG **edge** $`e`$ if:
- exists a directed path from that **edge** $`e`$ to a **node** where the variable $`v`$ is **used** (node in $`use[v]`$)
- and that path does **not** go through any **def** of $`v`$ (node in  $`def[v]`$);

<br>

> **Note**:<br>
> We have **liveness on edges**: **before** and **after** each node.<br>

<br>

## Uses and Defs
Every **l-value** occurrence of variable $`v`$ is called **def** (or **definition**, **modification**, **writing**).<br>
Every **r-value** occurrence of variable $`v`$ is called **use** (or **reading**).<br>

<br>

**Definitions**:
- $`def[v]`$ is the **set of all nodes** in _CFG_ that **define** variable $`v`$;
- $`use[v]`$ is the **set of all nodes** in _CFG_ that **use** variable $`v`$;
- $`def[n]`$ is the **set of variables** that are **modified** (**written**) at node $`n`$ (aka **kill set**);
- $`use[n]`$ is the **set of variables** that are **used** (**read**) at node $`n`$ (aka **gen set**);

<br>

**Example**:
![statement](/img/statement.png)

For above node $`n`$:
- $`use[n] = {c, b}`$
- $`def[n] = {c}`$

<br>

**Statements** are labeled using the following conventions: $`s(i)`$ where $`i`$ is an integer in $`[1, n]`$ and $`n`$ is the number of statements in the **basic block**.<br>
A _definition_ at statement $`s(i)`$ with $`i < j`$ is **alive** at $`j`$, if it has a **use** at a statement $`s(k)`$ with $`k ≥ j`$.<br>
A _definition_ at statement $`s(i)`$ **kills** all previous definitions for the same variables ($`s(k)`$ with $`k < i`$.<br>

<br>

## UD and DU chains
A **use-def chain** (or **UD chain**) for each **use** of variable stores the set of **all defs** that reachable **backward** from that **use without** any other **intervening defs**.<br>
A **def-use chain** (or **DU chain**) for each **def** of variable stores the set of **all uses** that reachable **forward** from that **def without** any other **intervening defs**.<br>

<br>

![du-chains](/img/du-chains.png)

<br>

**DU chains**:

| Var | Def | Uses  |
|:----|:----|:------|
| x   | 1   | 2,4,5 |
| x   | 3   | 4,5   |
| y   | 5   | 6     |

<br>

**UD chains**:

| Var | Use | Defs |
|:----|:----|:-----|
| x   | 2   | 1    |
| x   | 4   | 1,3  |
| x   | 5   | 1,3  |
| y   | 6   | 5    |


<br>

# Computing liveness
**Computing liveness** means **compute the nodes where a variable is live**.<br>

**Liveness** _flows_ **backward** in the _CFG_: behaviour of next nodes determines liveness at a previous node.<br>
In other words, to **compute liveness** at a given point, we need to look into the future.<br>

<br>

## Backtracking algorithm
The **formal definition of liveness** leads naturally to a **backtracking algorithm** for **computing liveness**.<br>
**Backtracking algorithm**: to **compute liveness** traverse all simple paths in the CFG that **start** from a **use** of the variable and that do **not** include a **def**.<br>
This algorithm is **inefficient** because it requires traversing an **exponential number** of paths.<br>

<br>

## Algorithm that solves data-flow equations
The sets $`in[n]`$ and $`out[n]`$ for any node $`n`$ in _CFG_ **must satisfy** the **data-flow equations**.<br>

<br>

Consider some node $`n`$ in _CFG_, then **data-flow equations**:
- $`out[n] = \bigcup_{\forall s \in succ[n]} in[s]`$
  - explanation:
    - a variable is **live-out** at node $`n`$ if it's **live-in** on **any successor node** of $`n`$;
- $`in[n] = use[n] \cup (out[n] - def[n])`$
  - explanation:
    - variables that are **used** in a node $`n`$ are **live-in** for $`n`$;
    - variables that are **live-out** from a node $`n`$ must also be **live-in** to this node $`n`$ too; 
    - **except** for the variables that are in $`def[n]`$;

<br>

_Data-flow equations_ are used to build a **system of equations** for all nodes of CFG.<br>

Algorithm **sets up data-flow equations for each node** and forms **system of equations** and then **solves it**.<br>

The most common way of **solving** the _data-flow equations_ is by using an **iterative algorithm**:
- **solving** the _data-flow equations_ **starts** with **initializing all in-states** and **out-states** to the **empty set**, in other words, **all variables are considered dead**;
- then, the **out-states** are then **computed** by applying the **transfer functions** on the **in-states**:
  - $`out_{b}=trans_{b}(in_{b})`$
- then, the **in-states** are **updated** by applying the **join operations**:
  - $`in_{b}=join_{p\in pred_{b}}(out_{p})`$

The **last two steps** are **repeated locally for each node until** algorithm reaches the so-called **fixpoint**: the situation when the **in-states do not change** (and the out-states in consequence).


<br>

**Algorithm**:
```latex
// Init all in[n] and out[n] by empty set
for n in CFG {
  in[n] = {}
  out[n] = {}
}

repeat {
  for n in CFG<in reverse topological order> {
    // Save old values of in[n] and out[n]
    in'[n] = in[n]
    out'[n] = out[n]
    // Compute new values for in[n] and out[n] using data-flow equations
    out[n] = Union{for all s in succ[n]} in[s]
    in[n] = use[n] union (out[n] - def[n])
  }
} until in'[n] = in[n] and out'[n] = out[n] for each n in CFG
```

<br>

Algorithm finds a **fixpoint** of the $`in[n]`$ and $`out[n]`$ equations. When the algorithm terminates, all equations are satisfied.<br>

<br>

# Static Single Assignment
Several modern compilers use **static single-assignment form** (**SSA**) as the method for analysis of variable dependencies.<br>
**Static Single Assignment** (**SSA**) form is a type of IR where each variable is assigned **exactly once**.<br>
The name of every variable consists of 2 parts: **name** and **version**. Each assignment to variable **increments** its **version**.<br>
Branches are changed by **phi nodes** (aka **phi functions**): **phi node** selects one of its input.<br>

<br>

**Advantages of SSA**: it **simplifies** _data flow analysis_ & many _compiler optimizations_ due to **explicit** & **simplified def-use chains**.<br>
**Each use has exactly one definition**.<br>

<br>

**Linear code**:
```shell
x = 0
y = 1
x = 5
```

```shell
x_0 = 0
y_0 = 1
x_1 = 5
```

<br>

**Control flow**:
```shell
if (...)
  x = 1
 else
  x = 5
y = x
```

```shell
if (...)
  x_0 = 1
 else
  x_1 = 5
x_2 = Phi(x_0, x_1)
y_0 = x_2
```

<br>

**Loops**:
```shell
i = 0;
do
  i = i + 1
while (i < 50)
```

```shell
i_0 = 0;
do
  i_1 = Phi(i_0, i_2)
  i_2 = i_1 + 1
while (i_2 < 50)
```

<br>

## Relation to dominator trees
One property of SSA is that defs dominates uses. So, dominator trees are used to build SSA.<br>
