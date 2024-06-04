# Table of contents
- [Table of contents](#table-of-contents)
- [Heap](#heap)
- [Insertion](#insertion)
- [Deletion](#deletion)
  - [Operations](#operations)
    - [Parent, left, right](#parent-left-right)
    - [Max heapify](#max-heapify)
    - [Build max heap](#build-max-heap)
      - [Example](#example)
    - [Prove that binary heap of size `n` has `⌈n/2⌉` leaves](#prove-that-binary-heap-of-size-n-has-n2-leaves)

<br>

# Heap
The **binary heap** or just **heap** is a **complete binary tree** in which all **nodes** must **satisfy** a **heap property**.<br>

There are **two kinds** of heaps: **max-heaps** and **min-heaps**.<br>

A **heap property** depends on the **kind of heap**:
- in a **max-heap**, the **heap property** is that **every node** `i` is **less than or equal to** its **parent**: `H[i] ≤ H[parent(i)]` and **greater than or equal to** its **descendants**;
  - so, in the **max-heap** the **root** is a **largest** element in the heap;
- in a **min-heap**, the **heap property** is that **every node** `i` is **greater than or equal to** its **parent**: `H[i] ≥ H[parent(i)]` and **less than or equal to** its **descendants**;
  - in the **min-heap** the **root** is a **smallest** element in the heap;

<br>

|**min-heap**|**max-heap**|
|:--|:--|
|![min-heap](/img/min-heap.png)|![max-heap](/img/max-heap.png)|

<br>

**Heap** `H` **can be viewed** as a **array**:
- **root** at index `1`:
  - the **root** of the **tree** is `H[1]`;
  - the **parent** of `H[i]` is `⌊i/2⌋`;
  - the **left child** of `H[i]` is `2i`;
  - the **right child** of `H[i]` is `2i + 1`;
- **root** at index `0`:
  - the **root** of the **tree** is `H[0]`;
  - the **parent** of `H[i]` is `⌊(i-1)/2⌋`;
  - the **left child** of `H[i]` is `2i + 1`;
  - the **right child** of `H[i]` is `2i + 2`;

<br>

**Special atrbiutes**:
  - `length` represent the **max** size of **array**;
  - `heap_size` represent the **actual** size of **heap**:
    - **only** elements in `H[0..(heap_size-1)]`, where `0` ≤ `heap_size` ≤ `length`, are **valid** elements of the heap;

<br>

Mapping **heap** to **array**:

<br>

![heap-layers](/img/heap-layers.png)

<br>

![heap-as-array](/img/heap-as-array.png)

<br>

Consider **heap** that has `k` levels, then every level `i` adds **2<sup>i</sup>** nodes to **heap**:
|Level|Number of elements in level|Starting index of level|
|:----|:--------------------------|:----------------------|
|**0**|**2<sup>0</sup>** = **1**|`0`|
|**1**|**2<sup>1</sup>** = **2**|**2<sup>0</sup>** = `1`|
|**2**|**2<sup>2</sup>** = **4**|**2<sup>0</sup>** + **2<sup>1</sup>** = `3`|
|**3**|**2<sup>3</sup>** = **8**|**2<sup>0</sup>** + **2<sup>1</sup>** + **2<sup>2</sup>** = `7`|
|...|...|...|
|**k**|**2<sup>k</sup>**|**2<sup>0</sup>** + **2<sup>1</sup>** + ... + **2<sup>k-1</sup>**|

<br>

Thus, **disatance** between **current node** and **its childs** is **grows up** from node to node because **every previous node** in **current** level **derives new 2 children** in **next** below level.<br>

For example,
- index of node **at the beginig** of level `k` is **2<sup>0</sup>** + **2<sup>1</sup>** + ... + **2<sup>k-1</sup>**;
  - index of its **left** child (it is also the **first** node in next level `k+1`) is **2<sup>0</sup>** + **2<sup>1</sup>** + ... + **2<sup>k-1</sup>** + **2<sup>k</sup>**;
- index of node **at the end** of level `k` is **2<sup>0</sup>** + **2<sup>1</sup>** + ... + **2<sup>k-1</sup>** + **2<sup>k</sup>** - **1**;
  - index of its **left** child (it is also the **last** node in next level `k+1`)  is **2<sup>0</sup>** + **2<sup>1</sup>** + ... + **2<sup>k-1</sup>** + **2<sup>k</sup>** + **2<sup>k+1</sup>**;

<br>

# Insertion
![insert-heap](/img/insert-heap.png)

<br>

# Deletion
![del-heap](/img/del-heap.png)

<br>

## Operations
**Operations** for heap are depends on kind of heap.<br>

Operations for **max heaps**:
|Operation|Complexity|Description|
|:--------|:---------|:----------|
|`parent`|**O(1)**|returns **index of parent** for node with index `i`|
|`left`|**O(1)**|returns **index of left child** for node with index `i`|
|`right`|**O(1)**|returns **index of right child** for node with index `i`|
|`max-heapify`|**O(log<sub>2</sub>N)**|**maintains** the **heap property**|
|`build-max-heap`|**O(n)**|**builds** a **heap** from an **unordered** array|
|`heapsort`|**O(n·log<sub>2</sub>N)**|**sorts** an array in place|
|`max-heap-insert`|**O(log<sub>2</sub>N)**||
|`heap-get-max`|**O(log<sub>2</sub>N)**||
|`heap-extract-max`|**O(log<sub>2</sub>N)**||
|`heap-increase-key`|**O(log<sub>2</sub>N)**||

<br>

### Parent, left, right
```rust
parent(i)
  return ⌊(i-1)/2⌋
```

<br>

```rust
left(i)
  return 2i + 1
```

<br>

```rust
right(i)
  return 2i + 2
```

<br>

### Max heapify
```rust
swap(H,i,largest)
  tmp = H[i]
  H[i] = H[largest]
  H[largest] = H[i]

max_heapify(A,i)
  l = left(i)
  r = right(i)
  if l ≤ heap_size AND H[l] > H[i]
    largest = l
  else
    largest = i
  if r ≤ heap_size AND H[r] > H[largest]
    largest = r
  if largest != i
    swap(H,i,largest)
    max_heapify(A,largest)
```

**Notes**:
- At each step, the **largest** of the elements `H[i]`, `H[left(i)]` and `H[right(i)]` is determined and **its index** is stored in `largest`:
  - If `H[i]` is **largest**, then the subtree rooted at node `i` is **already** max-heap and procedure **terminates**.
  - Otherwise, one of two children of the node `i` has the **largest** element and `H[i]` is swapped with `H[largest]`, then the node `H[i]` and its children **satisfy** the **max-heap property**. The node `H[largest]`, however, has the **original** value of `H[i]`, and thus the subtree rooted at `largest` might **violate** the **max-heap property**. Consequently, we call `max_heapify` **recursively** on taht subtree.

<br>

### Build max heap
If we consider **0** as **starting index of heap**, then **all leaves** are between `⌊n/2⌋` and `n-1`.

<br>

```rust
heap_size = len(H)

build_max_heap(H,i)
  for i = ⌊heap_size⌋-1 downto 0
    max_heapify(H,i)
```

**Notes**:
- Index **⌊n/2⌋** devides array on **2 halfs**:
  - the elements in the **right half** of array with indexes `[⌊n/2⌋, n-1]` are **all leaves** and every of them satisfy heap property;
  - the elements in the **left half** of array with indexes `[0, ⌊n/2⌋-1]` are **all non-leaves**;
- We call `max_heapify` on **each** node in the **left half** of array moving from **bottom** to **up** layers to convert an array `H` into **max heap**;
- **Each** call of `max_heapify` costs **O(log<sub>2</sub>N)** time and `build_max_heap` makes **O(n)** such calls. Thus the running time is **O(n·log<sub>2</sub>N)**;
- **O(n·log<sub>2</sub>N)** is **upper bound**, but it is **not tight bound**;
- It can be proved that **tight bound** in **linear time**. So, we can **build heap** from an **arbitrary array** in **linear time**;

<br>

#### Example
|Index|0|1|2|3|4|5|6|7|8|9|10|
|:----|:-|:-|:-|:-|:-|:-|:-|:-|:-|:-|:-|
|Input array|1|3|5|4|6|13|10|9|8|15|17|
|Array representation of **heap**|17|15|13|9|6|5|10|4|8|3|1|

<br>

- **Indexes** of **leaves**: `[⌊11/2⌋, 11-1] = [5, 10]`;
- **Indexes** of **non-leaves**: `[0, ⌊11/2⌋-1] = [0, 4]`;

<br>

```rust
Final Heap:
             17
           /    \
         15      13
        /  \     / \
       9     6  5   10
      / \   / \
     4   8 3   1
```

<br>

### Prove that binary heap of size `n` has `⌈n/2⌉` leaves
Consider heap that has `k+1` levels, then `k` is **penultimate level** /pəˈnʌl.tɪ.mət/, and `k+1` is the **last level**. **Leaf nodes** (**leaves**) can come only from **last two levels**: `k` and `k+1`:
- the **penultimate level** `k` contains **2<sup>k</sup>** nodes and *but* **number of leaves** at this level **depends** on how many nodes at below level `k+1`;
- the **last level** `k+1` contains `x` nodes and they are **all leaves**;

Node at **penultimate level** can have `0`, `1` or `2` children.<br>
If the **last level** `k+1` conatins `x` node, there are possible 2 cases (below `p` is a **number of parents** at the level `k`):
- `x` is **even**, then **every** parent at **previous level** `k` has **2** children
  - so, `p` **parents** at `k` derive `2p` **children** at the level `k+1`:
    - `2p = x` => `p = x/2`, also we know that `⌈y/2⌉ = y/2` **if** `y` is **even** integer, => `p = ⌈x/2⌉`;
- `x` is **odd**, there is **exactly one** parent at `k` level that has **1** *children* and **all others** have **2** *children*:
  - so, `p-1` **parents** at `k` derive `2·(p-1)` **children** at `k+1` and `1` **parent** at `k` derive `1` **children** at `k+1`:
    - `2·(p-1) + 1 = 2p - 2 + 1 = 2p - 1 = x` => `p = (x + 1)/2`, also we know that `⌈y/2⌉` = `(y+1)/2` **if** `y` is **odd** integer, => `p = ⌈x/2⌉`;

<br>

So, **p = ⌈x/2⌉**.<br>

<br>

1. Let denote **total leaves** in **whole heap** as **l**, then **l = x + (2<sup>k</sup> - p)**, because:
   - `k` **level** has **2<sup>k</sup>** nodes of which only **2<sup>k</sup> - p** are leaves;
   - `k+1` **level** has **x** leaves;
2. Let denote **total nodes** in **whole heap** as **n**, then **n = x + (2<sup>k+1</sup> - 1)**:
   - **(2<sup>k+1</sup> - 1)** is the **number of nodes** in **nested tree** of **height** `k` (**from** level `0` **up to** `k`);
   - `k+1` **level** has **x** leaves and **nested tree** of **height** `k` **doesn't** include them;

<br>

Thus:<br>
1. **n** = **x + (2<sup>k+1</sup> - 1)**
   - = **2·2<sup>k</sup> = n - x + 1**
   - = **2<sup>k</sup> = (n - x + 1)/2**
2. **l** = **x + (2<sup>k</sup> - p)** 
   - = **x + 2<sup>k</sup> - ⌈x/2⌉** 
   - = **x + (n - x + 1)/2 - ⌈x/2⌉** 
   - = **x + n/2 - x/2 + 1/2 - ⌈x/2⌉** 
   - = **x/2 + n/2 + 1/2 - ⌈x/2⌉**

<br>

So, **l** = **x/2 + n/2 + 1/2 - ⌈x/2⌉**<br>

<br>

Also we know that:
- `⌈y/2⌉ = y/2` **if** `y` is **even** integer;
- `⌈y/2⌉` = `(y+1)/2` **if** `y` is **odd** integer;

<br>

There are possible **2 cases**:
1. if `x` is **even**, then `n` is **odd**:
   - **l** = **x/2 + n/2 + 1/2 - ⌈x/2⌉**
   - = **x/2 + n/2 + 1/2 - x/2**
   - = **n/2 + 1/2**
   - = **⌈n/2⌉**;
   - so, **l** = **⌈n/2⌉**;
2. if `x` is **odd**, then `n` is **even**:
   - **l** = **x/2 + n/2 + 1/2 - ⌈x/2⌉**
   - = **x/2 + n/2 + 1/2 - (x+1)/2**
   - = **x/2 + n/2 + 1/2 - (x/2 + 1/2)**
   - = **x/2 + n/2 + 1/2 - x/2 - 1/2**
   - = **n/2**, **n/2** is **integer**, because `n` is **even**
   - = **⌈n/2⌉**;
   - so, **l** = **⌈n/2⌉**;

<br>

So, there `⌈n/2⌉` leaves in **binary heap** of `n` nodes.
