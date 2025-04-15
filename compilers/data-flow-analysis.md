<!-- TOC -->
* [Data-flow analysis](#data-flow-analysis)
* [UD and DU chains](#ud-and-du-chains)
* [Liveliness Analysis](#liveliness-analysis)
  * [Uses and Defs](#uses-and-defs)
* [Computing liveness](#computing-liveness)
  * [Backtracking algorithm](#backtracking-algorithm)
  * [Algorithm that solves data-flow equations](#algorithm-that-solves-data-flow-equations)
* [Static Single Assignment](#static-single-assignment)
<!-- TOC -->

<br>

# Data-flow analysis
**Data-flow analysis** is the process of collecting information about the way the variables are **defined** and **used** in the program.<br>
**Data-flow analysis** forms the foundation for a wide variety of compiler **optimizations** and program **verification techniques**.<br>

<br>

# UD and DU chains
A **definition-use chain** (or **DU chain**), which consists of a definition D of a variable and all the uses U reachable from that definition without any other intervening definitions.


A **use-definition chain** (or **UD chain**) is a data structure that consists of a use U, of a variable, and all the definitions D of that variable that can reach that use without any other intervening definitions.
A **definition-use chain** (or **DU chain**), which consists of a definition D of a variable and all the uses U reachable from that definition without any other intervening definitions.
Both UD and DU chains are created by using a form of static code analysis known as data flow analysis.

**Statements** are labeled using the following conventions: $`s(i)`$ where i is an integer in $`[1, n]`$ and $`n`$ is the number of statements in the **basic block**.<br>

A definition at statement $`s(i)`$ with $`i < j`$ is **alive** at $`j`$, if it has a **use** at a statement $`s(k)`$ with $`k ≥ j`$.<br>
A definition at statement $`s(i)`$ **kills** all previous definitions for the same variables ($`s(k)`$ with $`k < i`$.<br>

<br>

# Liveliness Analysis
A _variable_ is **live** _at particular point_ in the program if it holds a **value** that will be used in the **future**, otherwise _variable_ is **dead**.<br>

_CFG_ in which every **basic block** (**node**) is a **separate statement** is usually used for **liveness analysis**.<br>

<br>

**Definitions**:
- nodes in such _CFG_ can have **in-edges** and/or **out-edges**:
  - **in-edges** come **from predecessor** nodes;
  - **out-edges** lead **to successor** nodes;
- $`pred[n]`$ is the **set** of **all** **predecessors** of node $`n`$;
- $`succ[n]`$ is the **set** of **all** **successors** of node $`n`$;
- a _variable_ is **live-in** at node $`n`$ if it's live on any **in-edges** of node $`n`$;
- a _variable_ is **live-out** at node $`n`$ if it's live on any **out-edges** of node $`n`$;
- $`in[n]`$ is the **set** of **all** **live-in** _variables_ of a node $`n`$;
- $`out[n]`$ is the **set** of **all** **live-out** _variables_ of a node $`n`$;

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

**Formal definition of liveness**: a variable $`v`$ is **live on** CFG **edge** $`e`$ if:
- exists a directed path from that **edge** $`e`$ to a **node** where the variable $`v`$ is **used** (node in $`use[v]`$)
- and that path does **not** go through any **def** of $`v`$ (node in  $`def[v]`$);

<br>

**Example**:
![statement](/img/statement.png)

For above node $`n`$:
- $`use[n] = {c, b}`$
- $`def[n] = {c}`$

<br>

# Computing liveness
**Computing liveness** means **compute the nodes where a variable is live**.<br>

We have **liveness on edges**: **before** and **after** each node.<br>
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
- **solving** the _data-flow equations_ **starts** with **initializing** **all** **in-states** and **out-states** to the **empty set**, in other words, **all variables are considered dead**;
- then, the **out-states** are then **computed** by applying the **transfer functions** on the **in-states**:
  - $`out_{b}=trans_{b}(in_{b})`$
- then, the **in-states** are **updated** by applying the **join operations**:
  - $`in_{b}=join_{p\in pred_{b}}(out_{p})`$

The **last two steps** are **repeated locally for each node until** algorithm reaches the so-called **fixpoint**: the situation when the **in-states do not change** (and the out-states in consequence).


<br>

**Algorithm**:
- **for** all **node** $`n \in CFG`$ do
  - $`in[n] = \emptyset`$
  - $`out[n] = \emptyset`$
- **end for**
- **repeat**
  - **for** all **node** $`n \in CFG`$ in reverse topological order do
      - $`out_{OLD}[n] = out[n]`$
      - $`in_{OLD}[n] = in[n]`$
      - $`out[n] = \bigcup_{\forall s \in succ[n]} in[s]`$
      - $`in[n] = use[n] \cup (out[n] - def[n])`$
  - **end for**
- **until** **no changes in** $`out[n]`$ and $`out[n]`$ 
  - (more formally $`\forall n \in CFG \enspace out_{OLD}[n] = out[n] \enspace and \enspace in_{OLD}[n] = in[n]`$)

<br>

Algorithm finds a fixpoint of the $`in[n]`$ and $`out[n]`$ equations. When the algorithm terminates, all equations are satisfied.<br><br>

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
