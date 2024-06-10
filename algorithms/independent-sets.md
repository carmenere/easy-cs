# Independent sets
## Independent set
**Independent set** is a **set of vertices** `V` such that **any two vertices** in `V` are **not adjacent**. In other words, the **induced subgraph** by `V` an **edgeless** graph, i.e. it consists of **isolated** vertices.<br>
There can be **more than one** *independent sets* for a given graph.<br>

<br>

## Maximum independent set
**Maximum independent set** is an **independent set** of **largest cardinality**.<br>

The **independence number** of graph `G` is the **cardinality** of its **maximum independent set**.<br>
The **independence number** of graph `G` is denoted by `α(G)`.<br>

<br>

## Dominating set
**Dominating set** for a graph `G = (V, E)` is a **subset** `D` of `V` such that every vertex **not** in `D` is adjacent to **at least one** member of `D`.<br>

There can be **more than one** dominating sets for a given graph.<br>

The **domination number** of graph `G` is the **number of vertices** in its **smallest dominating set**.<br>
The **domination number** of graph `G` is denoted by `γ(G)`.

<br>

## Maximal independent set
**Maximal independent set** (**MIS**) is an **independent set** that is **not** a subset of any other independent set.<br>

There can be **more than one** MIS for a given graph.<br>

**Every maximum independent set is MIS** but the converse is **not** always true.<br>

The given graph has **6 different MIS** shown as the red vertices, **2** of them are **maximum**:
<br>

![max-independant-set-1](/img/max-independant-set-1.png)

<br>

Any MIS is also a **dominating set** in the graph, and every dominating set that is independent must be maximal, so MISs are also called independent dominating sets.

<br>

## Examples
### Example 1
![max-independant-set-2](/img/max-independant-set-2.png)

<br>

### Example 2
![graph-2](/img/graph-2.png)

<br>

All the possible **independent sets** for the given graph:
- `{}`;
- `{1}`;
- `{2}`;
- `{3}`;
- `{4}`;
- `{5}`;
- `{1, 4}`;
- `{1, 5}`;
- `{2, 3}`;
- `{2, 5}`;
- `{3, 5}`;
- `{2, 3, 5}`;

<br>

All the possible **maximum independent sets** for the given graph:
- `{2, 3, 5}`;

<br>

All the possible **maximal independent sets** for the given graph:
- `{1, 4}`;
- `{1, 5}`;

<br>

**But** `{2, 3}`, `{2, 5}` and `{3, 5}` are all **subsets** of another independant set `{2, 3, 5}` and thus they are **not maximal** by defenition.<br>

<br>