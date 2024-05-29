# Table of contents
- [Table of contents](#table-of-contents)
- [Sorting algorithms](#sorting-algorithms)
- [Selection sort](#selection-sort)
- [Insertion sort](#insertion-sort)
- [Merge sort](#merge-sort)
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


<br>

# Merge sort


<br>

# Quicksort


<br>

# Heapsort


<br>

# Bubble sort


<br>