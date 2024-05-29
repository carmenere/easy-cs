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
- the **root** of the **tree** is `H[1]`;
- the **parent** of `H[i]` is **floor** of `i/2`;
- the **left child** of `H[i]` is `2i`;
- the **right child** of `H[i]` is `2i + 1`;

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
