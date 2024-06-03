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