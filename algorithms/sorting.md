# Table of contents
- [Table of contents](#table-of-contents)
- [Sorting algorithms](#sorting-algorithms)
- [Selection sort](#selection-sort)
- [Insertion sort](#insertion-sort)
- [Merge sort](#merge-sort)
  - [Merging sorted lists](#merging-sorted-lists)
    - [Trivial case](#trivial-case)
    - [More general case](#more-general-case)
  - [Complexity](#complexity)
    - [Time complexity](#time-complexity)
    - [In-place merging](#in-place-merging)
  - [Code](#code)
- [Quicksort](#quicksort)
  - [Partition](#partition)
  - [Code](#code-1)
    - [Quicksort](#quicksort-1)
    - [Lomuto’s partition](#lomutos-partition)
    - [Hoare partition](#hoare-partition)
    - [Optimal pivot](#optimal-pivot)
- [Heapsort](#heapsort)
- [Bubble sort](#bubble-sort)
- [Counting sort](#counting-sort)
- [Radix sort](#radix-sort)
  - [Code](#code-2)
    - [Radix sort that uses counting sort](#radix-sort-that-uses-counting-sort)

<br>

# Sorting algorithms
Sorting algorithms can be classified by:
1. **Computational complexity**.
   - best;
   - worst;
   - average;
2. **Memory usage**. In particular, some sorting algorithms need only O(1) memory and are called in-place. Sometimes O(log n) additional memory is considered in-place.
3. **Recursion**. Some algorithms are either recursive or non-recursive, while others may be both (e.g., merge sort).
4. **Stability**. Stable sorting algorithms maintain the relative order of records with equal keys (i.e., values).
5. **General method**.
   - insertion;
   - exchange; 
   - selection;
   - merging;
   - partitioning; 

<br>

An **internal sort** is any data sorting process that takes place entirely within the main memory of a computer. This is possible whenever the data to be sorted is small enough to all be held in the main memory. For sorting larger datasets, it may be necessary to hold only a chunk of data in memory at a time, since it won’t all fit.<br>

**External sorting** is a class of sorting algorithms that can handle massive amounts of data. External sorting is required when the data being sorted do not fit into the main memory of a computing device (usually RAM) and instead they must reside in the slower external memory, usually a hard disk drive. Thus, external sorting algorithms are external memory algorithms and thus applicable in the external memory model of computation.<br>

**External sorting** algorithms generally fall into two types, distribution sorting, which resembles quicksort, and external merge sort, which resembles merge sort. The latter typically uses a hybrid sort-merge strategy. In the sorting phase, chunks of data small enough to fit in main memory are read, sorted, and written out to a temporary file. In the merge phase, the sorted subfiles are combined into a single larger file.<br>

|Name|Best|Average|Worst|Memory|Stable|Swaps|General method|
|:---|:---|:------|:----|:-----|:-----|:----|:-------------|
|**Selection sort**|n<sup>2</sup>|n<sup>2</sup>|**n<sup>2</sup>**|O(1)|No|O(n)|Selection|
|**Insertion sort**|n|n<sup>2</sup>|**n<sup>2</sup>**|O(1)|Yes|O(n<sup>2</sup>)|Insertion|
|**Merge sort**|n·log<sub>2</sub>n|n·log<sub>2</sub>n|**n·log<sub>2</sub>n**|O(n)|Yes|O(n·log<sub>2</sub>n)|Merging|
|**Quicksort**|n·log<sub>2</sub>n|n·log<sub>2</sub>n|**n<sup>2</sup>**|O(log<sub>2</sub>n)|No|O(n·log<sub>2</sub>n)|Partitioning+Exchanging|
|**Heapsort**|n·log<sub>2</sub>n|n·log<sub>2</sub>n|**n·log<sub>2</sub>n**|O(1)|No|O(n·log<sub>2</sub>n)|Selection|
|**Bubble sort**|n|n<sup>2</sup>|**n<sup>2</sup>**|O(1)|Yes|O(n<sup>2</sup>)|Exchanging|
|**Counting sort**|n|n|n|O(n)|Yes|No||
|**Radix sort**|k·n|k·n|k·n|O(n)|Yes|No||

<br>

**Notes**:
- **Quicksort** is a good default choice. It has **O(n<sup>2</sup>)**, but it tends to be fast in practice, because **O(n<sup>2</sup>)** has low probability.
- **Heapsort** is a good choice if you can't tolerate a worst-case time complexity of **O(n<sup>2</sup>)** or need low space costs. The Linux kernel uses heapsort instead of quicksort for both of those reasons (https://github.com/torvalds/linux/blob/master/lib/sort.c).
- **Merge sort** is a good choice if you handle data sets that can't fit in RAM, where the bottleneck cost is reading and writing the input on disk, not comparing and swapping individual items.
- **Radix sort** is fast. But, if you're using it to sort **binary numbers**, then there's a **hidden constant factor** that's usually **32** or **64** (depending on how many bits your numbers are). That's often way bigger than, meaning radix sort **tends to be slow in practice**.
- **Counting sort** is a good choice in scenarios where the biggest value in array is not greater then `n` and can be used as index.

<br>

# Selection sort


<br>

# Insertion sort
- In each iteration:
  - the algorithm sorts the imput **in place**;
  - the algorithm take **next** element called **key** from *right*, **unsorted part** of array (`[j..n-1]`);
  - **check** wheter key is in **right place** or **wrong**;
  - if **key** in **right** place the algorithm goes to next iteration;
    - if **key** in **wrong** place the algorithm finds **proper position** for **key** in *left*, **sorted part** of array (`[0, j-1]`):
      - it moves `A[j-1]`, `A[j-2]` and so on by **one position to the right** until it finds **proper position** for **key** (`A[j]`);
  - 

<br>

```rust
insertion_sort(A)
   for j = 1 to A.length-1
      key = A[j]
      i = j - 1
      while i > 0 and A[i] > key
         A[i+1] = A[i]
         i = i -1
      A[i+1] = key
```

<br>

Visualition of an algorithm:

<br>

![insertion-sort](/img/insertion-sort-1.png)

<br>

In **each** iteration:
- the **blue rectangle** holds the **key** taken from `A[i]`, which is compared with the values in **tan rectangles** to its **left**;
- **orange arrows** show array values **moved one position to the right**;
- **blue arrows** indicate **new position** of **key**;

<br>

# Merge sort
**Merge sort** is a sorting algorithm that follows a **divide-and-conquer** (/dɪˈvaɪd/, /ˈkɒŋ.kər/) approach and works like this:
- **recursively splits** (**divides**) the **input array** in **2 halfs** until **input array** can be divided:
  - `q` is a **midpoint** of **input** array `A` and `q=⌊(p+r)/2⌋`;
  - then **left** **subarray** `L = A[p, q]` and **right** subaray `R = A[q+1, r]`;
- **sorts** each **subarray** starting from the **bottom level of recursion**, the recursion **cotinuous** until `p<r`, when `p=r` the recursion **stops**, because **subarray** contains excatly **one** element;
- **merges** the sorted subarrays back together;

<br>

Here is how the entire merge sort algorithm unfolds:
![merge-sort-overall](/img/merge-sort-overall.png)

<br>

## Merging sorted lists
The **key operation** of the merge sort algorithm is the **merging two sorted subarrays** in the *combine* stage.<br>

<br>

### Trivial case
There are a number of **1-item arrays** at the **bottom level** of recursion. Every array that consist of **one element** is **trivial** and considered **sorted**.<br>
So, **merging** two **1-item arrays** is easy — the smaller item goes first and the larger item goes second in resulting **2-items array**:

![merge-two-sorted-lists-1](/img/merge-two-sorted-lists-1.svg)

<br>

### More general case
The algorithm chooses the **smallest** element of **two subarrays**, then copy it to **resulting array**, then somehow **mark** it in subarray to **skip** it in futher iterations:<br>
![merge-two-sorted-lists-2](/img/merge-two-sorted-lists-2.png)
![merge-two-sorted-lists-3](/img/merge-two-sorted-lists-3.png)

<br>

Once one of subarrays is **empty**, they **remaining items** from the other one can be **appended** to the end of the **resulting array**:
![merge-two-sorted-lists-4](/img/merge-two-sorted-lists-4.png)

<br>

## Complexity
### Time complexity
On each recursion level merge sort algorithm takes **O(n)** time.<br>
The number of all recursion levels is **O(log<sub>2</sub>N)**, where **N** is a size of whole **original array**.<br>

<br>

### In-place merging
Consider array `A=[1,5,10,2,4,7]`. Its both halfs **left** `L=[1,5,10]` and **right** `R=[2,4,7]` are **sorted**. How to merge them **without** creation **new** array?<br>
When element `2` will be moved to position of element `5` where to save `5`? To the previous position of `2`? So, the **right** half becomes **unsorted**.<br>
So, **in-place merge requires extra movements** inside original array and **leads to degradation**.

<br>

## Code
Teh `merge` function:
```rust
merge(A,p,q,r)
  l = q - p                // index of last element in L, so len(L) = l +1
  r = r - (q + 1)          // index of last element in R, so len(R) = r + 1

  for i in [0, l]          // copy A[p..q] to L[0..l]
    L[i] = A[p+i]

  for i in [0, r]          // copy A[q+1..r] to R[0..r]
    L[i] = A[q+1+i]

  k = p                    // index of current element in A and it begins with p
  n = 0                    // index of current element in L
  m = 0                    // index of current element in R

  while n <= l and m <= r  // continue until one of L or R become empty
    if L[n] <= R[m]        // find min element of L and R
      A[k] = L[n]          // if L[n] is min then copy it to A[k] and increment pointer n
      n = n + 1
    else
      A[k] = R[m]          // if R[m] is min then copy it to A[k] and increment pointer m
      m = m + 1
    k = k + 1
  
  if n <= l                 // if (n <= l) is true it means L is not empty
    for i in [n, l]         // so, copy remaining elements in L to A[k..r]
      A[k] = L[i]
      k = k + 1
  else                      // if (m <= r) is true it means R is not empty
    for i in [m, r]         // so, copy remaining elements in R to A[k..r]
      A[k] = R[i]
      k = k + 1 
```

<br>

Teh `merge_sort` function:
```rust
merge_sort(A,p,r)      // A is pointer to original aray
  if not(p < r)        // If p = r, then A[p] and A[r] is the same element, so array A[p..r] consists of one element
    return
  q = ⌊(p+r)/2⌋        // Finds midpoint of array A[p..r] which splits it on 2 halfs left A[p..q] and right A[q+1..r]
  merge_sort(A,p,q)    // Recursively calls merge_sort and pass it left half A[p..q]
  merge_sort(A,q+1,r)  // Recursively calls merge_sort and pass it right half A[q+1..r]
  merge(A,p,q,r)       // Merge two sorted halfs A[p..q] and A[q+1..r]
```

<br>

To **sort** the **entire array A** `A = [A[0], A[1], A[2], ... , A[n-1]]` we make **initial call** `merge_sort(A,0,len(A)-1)`.<br>

<br>

There possible 2 cases:
1. The length of `A` is **even**.
2. The length of `A` is **odd**.

![merge-sort-cases](/img/merge-sort-cases.png)

In case 1:
- the `merge_sort` is called with following arguments: `merge_sort(A, p=5, r=9)` for **right** half `R`;
- inside `merge_sort`:
  - `q = ⌊(5+9)/2⌋ = 7`;
  - when `A[5..7]` and `A[8..9]` are become sorted, the `merge` is called with following arguments `merge(A, p=5, q=7, r=9)`;
  - inside `merge`:
    - `l = q - p = 7 - 5 = 2`;
    - `r = r - (q + 1) = 9 - (7 + 1) = 1`;

<br>

# Quicksort
**Quicksort**, like merge sort, follows a **divide-and-conquer** (/dɪˈvaɪd/, /ˈkɒŋ.kər/) approach and works like this:

- **Divide**: **partitions** (aka **rearranges**) the array into **two** (*possibly empty*) subarrays `A[p..q-1]` (the **low side**) and `A[q+1..r]` (the **high side**) such that each element in the **low** side is **less than or equal to** the **pivot** `A[q]`, which is, in turn, **strictly less than** each element in the **high** side. Compute the index of the pivot as part of this `partition` procedure;
- **Conquer**: sorts each of the subarrays by calling `quicksort` procedure **recursively**;
- **Combine**: here `quicksort` does nothing, because the two subarrays are already **sorted**, no work is needed to combine them;

<br>

## Partition
The **key operation** of the *quicksort* algorithm is the **partition**. There are 2 partition schemas:
- **Lomuto partition**;
- **Hoare partition**;

<br>

Notes:<br>
- The *Lomuto partition* **separates** the **pivot** value **from** the two partitions it forms.
- The *Hoare partition* procedure, on the other hand, **always places** the **pivot** value **into** one of the two partitions.
- *Hoare partition* is **more efficient** than *Lomuto partition*.

<br>

## Code
### Quicksort
Teh `quicksort` function:
```rust
quicksort(A,p,r)
  if !(p < r)
    return
  
  q = partition(A,p,r)
  quicksort(A,p,q-1)
  quicksort(A,q+1,r)
```

To **sort** the **entire array A** `A = [A[0], A[1], A[2], ... , A[n-1]]` we make **initial call** `quicksort(A,0,A.length-1)`.<br>

### Lomuto’s partition
```rust
partition(A,p,r)
  pivot = A[r]
  i = p - 1            // highest index in the low side
  for j in [p, r-1]    // r-1 means we exclude pivot from loop
    if A[j] <= pivot   // A[j] <= pivot means that A[j] belongs to low side
      i = i + 1        // advance index in slow side
      swap(A[i], A[j]) // move A[j] to new position (i = i + 1 ) in slow side, and move A[i] to high side
  swap(A[r], A[i+1])   // move pivot right after last element in slow side
  return i+1           // return new index of pivot
```

<br>

The two cases for one iteration of procedure `partition`:
- If `A[j] > pivot`, the only action is to **increment** `j`:
![quicksort-partition-case-1](/img/quicksort-partition-case-1.png)
- If `A[j] <= pivot`, index `i` is **incremented**, `A[i]` and `A[j]` are **swapped**, and then `j` is **incremented**:
![quicksort-partition-case-2](/img/quicksort-partition-case-2.png)

<br>

The **overall Lomuto’s partition**:<br>
![quicksort-partition-overall](/img/quicksort-partition-overall.png)

<br>

### Hoare partition
**Quicksort** was invented by **Hoare**.<br>
Original partition procedure by Tony Hoare uses **two pinters**: **left** and **right** that are moved towards the middle.<br>
The **left** and **right** pointers will **never** stop on the **same item**:
- the **left** pointer is moved until it hits something **greater than or equal to** the **pivot**;
- the **right** item is moved until it hits something **less than or equal to** the **pivot**;
- if they stopped on the **same** item, then that item would have to **simultaneously** be **larger** than the pivot and **smaller** than the pivot. That's **impossible**!

This steps are repeated until the **left** and **right** pointers **crossed each-other**.
<br>

```rust
hpartition(A,p,r)
  pivot = A[r]
  i = p - 1              // left pointer
  j = r + 1              // right pointer
  while TRUE
    repeat
      j = j - 1
    until A[j] <= pivot  // the left pointer found something on the left that belongs on the right.
    repeat               // the right must find something on the right that belongs on the left to swap the two of them.
      i = i + 1
    until A[i] >= pivot
    if i < j
      swap(A[i], A[j])
    else                 // j > i it means that left and right pointers crossed each-other
      return j
```

<br>

**Visualization**:<br>

![hoare-1](/img/hoare-1.png)

<br>

### Optimal pivot
**Quicksort** **degrades** to **O(n<sup>2</sup>)** for **already sorted input**, if the **pivot** was chosen as the **first** or the **last** element.<br>
**Median-of-three** approach for **Lomuto partition**:<br>
```rust
mid := ⌊(lo + hi) / 2⌋
if A[mid] < A[lo]
    swap(A[lo], A[mid])
if A[hi] < A[lo]
    swap(A[lo], A[hi])
if A[mid] < A[hi]
    swap(A[mid], A[hi])
pivot := A[hi]
```

It puts a **median** into `A[hi]` first, then that **new value** of `A[hi]` is used for a **pivot**.<br>

<br>

# Heapsort
Heapsort is similar to selection sort - we're repeatedly choosing the largest item and moving it to the end of our array. But we use a heap to get the largest item more quickly.<br>

Notes about algorithm:
- The **heapsort algorithm** starts by calling the `build_max_heap` procedure to build a **max-heap** on the input array `A[0..n-1]`.
- The **maximum** element of the array is stored at the root `A[0]`, so `heapsort` procedure can place it into its **correct** final position by exchanging `A[0]` with `A[n-1]`.
- Then `heapsort` procedure decreease `A.heap_size` to discard node from the heap.
- The children of the root are **max-heaps**, but the **new root** element might **violate** the **max-heap property**, so `heapsort` procedure calls `max_heapify(A,0)` to **restore** the max-heap property.

<br>

```rust
heapsort(A)
  build_max_heap(A,A.length-1)
  for i in [n-1, 1]
    swap(A[1], A[i])
    A.heap_size = A.heap_size - 1
    max_heapify(A,0)
```

<br>

# Bubble sort
traverse from left and compare adjacent elements and the higher one is placed at right side. 
```rust
bubble_sort(A)
  for i in [0, n-2]
    swapped = false
    for j in [0, n-2-i]
        if A[j] > A[j + 1] {
            swap(A[j], A[j + 1])
            swapped = true

    // If no two elements were swapped inside INNER loop, then break
    if (swapped == false)
        return
}
```

<br>

# Counting sort
**Counting sort** assumes that **each** of the `n` input elements is an integer in the **range** `[0, k]`. **Counting sort** runs in `O(n)` time.<br>
**Counting sort** returns sorted output in the array `B[0..n-1]` and uses an array `C[0..k-1]` for temporary working storage.<br>
For **each** input element `x`, it finds out the **number** of elements **less than or equal to** `x`.<br>
It then uses this information to place element `x` **directly** into its index in the resulting array `B`.<br>

*Counting sort* is **not** a comparison sort. In fact, **no** comparisons between input elements occur anywhere in the code. Instead, *counting sort* uses the actual **values of the elements** as **indexes** in `C`.<br>

<br>

```rust
counting_sort(A,B,k)
  new array B[0..n-1]
  new array C[0..k-1]
  for i in [0, k]
    C[i] = 0

  for j in [0,n-1]
    C[A[j]] = C[A[j]] + 1   // A[j] contains some element i. The element i acts as index in C, so C[i] now contains the number of elements equal to i

  for i in [0,k]
    C[i] = C[i] + C[i-1]    // C[i] now contains the number of elements less than or equal to i

  for j in [n-1,0]
    B[C[A[j]]] = A[j]       // C[A[j]] stores the index for A[j], so copy A[j] to B at index C[A[j]]
    C[A[j]] = C[A[j]] - 1   // to exclude situations when duplicates are put in the same position

  return B
```

<br>

![counting-sort-1](/img/counting-sort-1.png)

- **(a)**: each element in `C` with index `i` (`C[i]`) contains the number of elements **equal** to `i` in `A`;
- **(b)**: each element in `C` with index `i` (`C[i]`) contains the number of elements **less than or equal to** `i`;
- **(c)-(e)**: copying elements from `A` to `B` using appropriate indexes from `C`;
- **(f)**: the final sorted array;

<br>

# Radix sort
**Radix sort** is **digital sort**, it sorts values by digits:<br>
![radix-sort-1](/img/radix-sort-1.png)

<br>

## Code
### Radix sort that uses counting sort
```rust
radix_sort(A,d)
  for i in [0,d-1]                // i is an index of digit
    radix_counting_sort(A,B,k,i)  // modified counting_sort that uses digit instead whole number
```

<br>

Given `n` **d-digit** numbers in which each digit can take on up `k` possible values, radix sort sorts these numbers in `O(d(n+k))` time.<br>
When d is constant and `k = O(n`) radix sort works at linear time `O(n+k)`.<br>
