<!-- TOC -->
* [Control flow analysis](#control-flow-analysis)
* [Dominators](#dominators)
* [Dominator tree](#dominator-tree)
<!-- TOC -->

<br>

# Control flow analysis
**Control-flow analysis** (**CFA**) is a static-code-analysis technique for **determining the control flow** of a program.<br>
The **control flow** is expressed as a **control-flow graph** (**CFG**).<br>
In a **CFG** we assume **no information** about data values. But **CFG** can be used in **data flow analysis**.<br>

**Each node** of CFG represents a **basic block**.<br>

**Basic block** can be:
- **single statement**;
- straight-line **sequence of statements** where
  - **no** branches or jumps occur within the _basic block_;
  - only the **first statement** of a _basic block_ can be a **target of a branch**;
  - only the **last statement** of a _basic block_ can be a **branch statement**;

<br>

Two special _basic blocks_:
- the **entry block**, through which control enters into the flow graph;
- the **exit block**, through which all control flow leaves;

<br>

# Dominators
Let $`G = \langle N, E, s \rangle`$ be a **CFG**, where:
- $`N`$: set of **nodes** (**basic blocks**);
- $`E`$: set of **edges**;
- $`s`$: **entry node**;

<br>

Let $`p \in N`$, $`q \in N`$.<br>

<br>

**Definitions**:
- node $`p`$ **dominates** node $`q`$, written $`p \le q`$, if **every path** _from_ the **entry node** _to_ $`q`$ **goes through** $`p`$
  - node $`p`$ is a **dominator** of node $`q`$;
  - node $`q`$ is **dominated by** node $`p`$;
- **dominator set** of a node $`q`$, written $`DOM(q)`$, is formed by **all nodes** that dominate $`q`$;
  - if node $`p`$ **dominates** node $`q`$ then $`p \in DOM(q)`$;
- node $`p`$ **strictly dominates** node $`q`$, written $`p < q`$ **if** $`p \le q`$ and $`p \ne q`$;
- node $`p`$ **immediately** (or **directly**) **dominates** node $`q`$, written $`p <_{d} q`$ or $`p = IDOM(q)`$ **if** $`p < q`$ and there is **no** $`t \in N`$ such that $`p < t < q`$;
  - in other words, the **immediate dominator** $`IDOM(q)`$ of some node $`q`$ is the dominator **closest** to the object $`q`$;
  - each node has a **unique** immediate dominator;
- **each node dominates itself** by definition, thus $`q \in DOM(q)`$;
- **dominance frontier** of node $`p`$, written $`DF(p)`$, is the set of all nodes $`w`$ such that $`p`$ **dominates** a **predecessor** of $`w`$, but $`p`$ **doesn't** dominate $`w`$;

The **entry block** _dominates_ **all** blocks.<br>

<br>

# Dominator tree
The **dominator tree** depicts the **dominator relationships**. It is a tree, since each block has a **unique** immediate dominator.<br>
The **dominator tree** is rooted at the entry block.<br>
**Dominance frontiers** are used to **finding all places** where to put **phi node**: if node $`p`$ defines a variable $`a`$, then every node in $`DF(p)`$ needs a **phi** function for $`a`$.<br>

The **Lengauer-Tarjan algorithm** builds the dominator tree for directed graph.<br>