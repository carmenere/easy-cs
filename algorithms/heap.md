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
    - [Number of leaves in binary heap is ⌈n/2⌉](#number-of-leaves-in-binary-heap-is-n2)

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

# Insertion
![insert-heap](/img/insert-heap.png)

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
Consider heap that has `k` levels, then every level adds **2<sup>k</sup>** nodes:
|Level|Number of elements in level|Starting index of level|
|:----|:--------------------------|:----------------------|
|**0**|**2<sup>0</sup>** = **1**|`0`|
|**1**|**2<sup>1</sup>** = **2**|**2<sup>0</sup>** = `1`|
|**2**|**2<sup>2</sup>** = **4**|**2<sup>0</sup>** + **2<sup>1</sup>** = `3`|
|**3**|**2<sup>3</sup>** = **8**|**2<sup>0</sup>** + **2<sup>1</sup>** + **2<sup>2</sup>** = `7`|
|...|...|...|
|**k**|**2<sup>k</sup>**|**2<sup>0</sup>** + **2<sup>1</sup>** + ... + **2<sup>k-1</sup>**|

<br>

Thus, **disatance** between **current node** and **its childs** is **grown up** and depends on how many elements before current node, thus it is depend on index of current node, because **every previous node** in **current** level **derives new 2 children** in **next** below level.<br>
For example, node at the beginig of level `k` has index equal to **2<sup>0</sup>** + **2<sup>1</sup>** + ... + **2<sup>k-1</sup>**, all nodes in previous level derive new childs in current level between curent node and its childs.

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

### Number of leaves in binary heap is ⌈n/2⌉
The **binary heap** (and **every complete binary tree**) has **⌈n/2⌉ leaves**:
1. A **perfect tree** of height **h** has exactly **2<sup>h+1</sup>−1** nodes.
2. Assume that the heap has height **k**. Thus
   - all levels from **0** to **(k−1)** actually form **perfect tree** with **height** **(k-1)**;
   - this **perfect tree** has **2<sup>h+1</sup>−1 = 2<sup>k-1+1</sup> = 2<sup>k</sup>−1** nodes;
3. There are exactly **n − (2<sup>k</sup>−1) = n − 2<sup>k</sup>+1** nodes on the **last level k**, and they are **all leaves**;
4. Each leaf on the **k-th** level has a **parent**. Moreover, each two consecutive leaves have the same parent (maybe except for the last node, whose parent has only one child);
5. So, there are **2<sup>k-1</sup>** nodes on the level **k−1**:
   - **⌈(n−2<sup>k</sup>+1)/2⌉** of them are **parents**;
   - the rest **2<sup>k-1</sup>−⌈(n−2<sup>k</sup>+1)/2⌉** are **leaves**;
6. The **total amount of leaves** is **n−2<sup>k</sup>+1** + **2<sup>k-1</sup>−⌈(n−2<sup>k</sup>+1)/2⌉**;





<br>

**Leaf nodes** can come only from **last two levels**. Given heap that has `k+1` levels, then `k` is **penultimate level** /pəˈnʌl.tɪ.mət/, and `k+1` is the **last level**:
- the **penultimate level** `k` contains **2<sup>k</sup>** nodes and **but** number of leaves at this level **depends** on how many nodes at below level `k+1`;
- the **last level** `k+1` contains **x** nodes and they are **all leaves**;

Node at **penultimate level** can have 0, 1 or 2 children. If last level conatins `x` node, there are 2 cases:
- `x` is **even**, then every parent at `k` level has **2** children, so, `p` parents on the level `k` derive `2p` children on the level `k+1`:
  - `2p = x`, so, **number of parents** `p`: `p = x/2`;
- `x` is **odd**, there is **exactly one** parent at `k` level that has **1** children and all others have **2** children:
  - `2·(p-1) + 1 = 2p - 2 + 1 = 2p - 1 = x`, so, **number of parents** `p`: `p = (x + 1)/2 = x/2 + 1/2`;

<br>

More general `p = ⌈x/2⌉`:
- when `x` is **even**, then `⌈x/2⌉` = `x/2` and `x/2` is integer;
- when `x` is **odd**, then `⌈x/2⌉` equal to `⌊x/2⌋ + 1`;

Let denote **total leaves** in whole heap as `L`, then **L = x + (2<sup>k</sup> - ⌈x/2⌉)**.<br>
Let denote **total nodes** in whole heap as `n`, then **n = x + (2<sup>k+1</sup> - 1)**:
  - here **2<sup>k+1</sup> - 1** is the number of nodes in nested tree from `0` level until level `k`;
  - **x** is the number of nodes at the **last level** `k+1`;

<br>

Thus:
1. **n = x + (2<sup>k+1</sup> - 1)** => **2·2<sup>k</sup> = n - x + 1** => **2<sup>k</sup> = (n - x + 1)/2**<br>
2. **L = x + 2<sup>k</sup> - ⌈x/2⌉** => **x + (n - x + 1)/2 - ⌈x/2⌉** => **x + n/2 - x/2 + 1/2 - ⌈x/2⌉** => **x/2 + n/2 + 1/2 - ⌈x/2⌉**
3. **L = x/2 + n/2 + 1/2 - ⌈x/2⌉** => **n/2 + x/2 + 1/2 - (x/2 + 1/2)** => **L = n/2**<br>

<br>

There are possible **2 cases**:
1. `n` is **even** and `x` is **odd** => `n/2` is **integer**.
2. `n` is **odd** and `x` is **even** => `n/2` is **not integer** and we must take upper bound `⌈n/2⌉` as a result.
