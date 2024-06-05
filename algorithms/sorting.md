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
- [Quicksort](#quicksort)
- [Heapsort](#heapsort)
- [Bubble sort](#bubble-sort)

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
|**Shell sort**|n·log<sub>2</sub>n|n<sup>4/3</sup>|**n<sup>3/2</sup>**|O(1)|No|O(n<sup>2</sup>)|Insertion|
|**Bubble sort**|n|n<sup>2</sup>|**n<sup>2</sup>**|O(1)|Yes|O(n<sup>2</sup>)|Exchanging|

<br>

Optimized variants of the **bubble sort**:
- **cocktail sort** (aka **shaker sort**);
- **comb sort**;

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
  - then **left** subarray `L = A[p, q]` and **right** subaray `R = A[q+1, r]`;
- **sorts** each subarray starting from the **bottom level of recursion**, the recursion is **cotinuous** until `p<r`, when `p=r` the recursion **stops**, because **subarray** contains excatly **one** element;
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

# Quicksort


<br>

# Heapsort
Heapsort is similar to selection sort - we're repeatedly choosing the largest item and moving it to the end of our array. But we use a heap to get the largest item more quickly.

<br>

# Bubble sort


<br>