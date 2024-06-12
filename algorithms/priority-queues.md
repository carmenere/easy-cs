# Table of contents
- [Table of contents](#table-of-contents)
- [Priority queuee](#priority-queuee)
  - [Maximum](#maximum)
  - [Extract-max](#extract-max)
  - [Increase-key](#increase-key)
  - [Insert](#insert)

<br>

# Priority queuee
One of the most popular applications of a *heap*: as an efficient **priority queue**.<br>
As with heaps, priority queues come in two forms: **max priority queues** and **min priority queues**.<br>

A **priority queue** is a data structure for maintaining a **set** `S` of elements, each with an associated value called a **key**.<br>
A **max priority queue** supports the following operations:
|Operation|Complexity|Description|
|:--------|:---------|:----------|
|`insert`|**O(log<sub>2</sub>N)**|**inserts** the element `x` with key `k` into the set `S`|
|`maximum`|**O(log<sub>2</sub>N)**|**returns** the element of `S` with the **largest** key|
|`extract-max`|**O(log<sub>2</sub>N)**|**removes** and **returns** the element of `S` with the **largest** key.|
|`increase-key`|**O(log<sub>2</sub>N)**|**increases** the value of element `x`’s key to the **new** value `k`, **new** value must be **greater than or equal to old**|

<br>

A **min priority queue** supports the following operations:
|Operation|Complexity|Description|
|:--------|:---------|:----------|
|`insert`|**O(log<sub>2</sub>N)**|**inserts** the element `x` with key `k` into the set `S`|
|`minimum`|**O(log<sub>2</sub>N)**|**returns** the element of `S` with the **smallest** key|
|`extract-min`|**O(log<sub>2</sub>N)**|**removes** and **returns** the element of `S` with the **smallest** key.|
|`decrease-key`|**O(log<sub>2</sub>N)**|**decreases** the value of element `x`’s key to the **new** value `k`, **new** value must be **less than or equal to old**|

<br>

The max priority queues can be used to **schedule** jobs. The max priority queue keeps track of the jobs to be performed and their relative priorities. When a job is finished or interrupted, the **scheduler** selects the **highest-priority job** by calling `extract-max`. The **scheduler** can **add** a **new job** to the queue at any time by calling `insert`.<br>

<br>

In summary, a **heap** can support **any** priority queue operation on a set of size `n` in **O(log<sub>2</sub>N)** time.

<br>

## Maximum
```rust
maximum(A)
    if A.heap_size < 1
        Error("heap underfow")
    return A[0]
```

<br>

## Extract-max
```rust
extract_max(A)
    max = maximum(A)
    A[0] = A[A.heap_size-1]
    A.heap_size = A.heap_size - 1
    max_heapify(A, 0)
    return max
```

<br>

## Increase-key
The `increase_key` procedure:
- verifies that the **new** key `k` will **not** cause the key in the object x to **decrease**, and if there is no problem, it assign `x` the **new** key `k`;
- then it finds the index `i` in the array corresponding to object `x`, so that `A[i]` is `x`;
- then it traverses a simple path from `A[i]` toward the root (`A[0]`) to find a **proper place** for the **newly increased key**, because **increasing** the key of `x` (`A[i]`) might **violate** the **max-heap property**;

<br>

```rust
increase_key(A, x, k)
    if k < x.key
        Error("new key is smaller than current key")
    x.key = k
    i = find the index i in array A that correspond to x
    while i > 0 and A[parent(i)].key < A[i].key
        swap(A[parent(i)], A[i])
        i = parent(i)
```

<br>

## Insert
The `insert` procedure:
- takes as inputs the **max-heap** `A`, the **new object** `x` to be inserted into the max-heap;
- then it verifies that the **max-heap** has space for the **new** element;
- then it expands the max-heap by adding to the tree a new leaf whose key is `-∞`;
- then it calls `increase_key(A, x, k)` to set the key of this new element to its correct value and maintain the max-heap property;

<br>

```rust
insert(A, x)
    if A.max_length == A.heap_size
        Error("heap overfow")
    A.heap_size = A.heap_size + 1
    k = x.key
    x.key = -∞
    A[A.heap_size-1] = x
    map to index heap-size in the array
    increase_key(A, x, k)
```