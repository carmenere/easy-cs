# Order statistics
The `i-th` **order statistic** of a set of `n` elements is the `i-th` **smallest** element.<br>

The `i-th` **order statistic** is **greater than or equal to** than **exactly** `i-1` other elements in `A`.<br>

The **minimum** of a set of `n` elements is the **first order statistic** (`i=1`).<br>
The **maximum** of a set of `n` elements is the **n-th order statistic** (`i=n`).<br>

<br>

Regardless of parity of `n`, there are **lower median** at `i`=`⌊(n+1)/2⌋` and **upper median** `i`=`⌈(n+1)/2⌉`.<br>

We can find the `i-th` **order statistic** in **O(n·log<sub>2</sub>n)** simply by sorting the lements and then outputting the `i-th` element in the **sorted array**.<br>

<br>

# Min and Max

```rust
minimum(A)
  min = 0
  for i in [0, n-1]
    if A[i] < min
      min = A[i]
  return min
```

<br>

```rust
maximum(A)
  max = 0
  for i in [0, n-1]
    if A[i] > max
      max = A[i]
  return max
```
