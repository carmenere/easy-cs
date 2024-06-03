# Table of contents
- [Table of contents](#table-of-contents)
- [Bounds](#bounds)
  - [Upper and lower bounds](#upper-and-lower-bounds)
  - [Tight bounds](#tight-bounds)
  - [Bound of functions](#bound-of-functions)
- [Algorithms and Data structures](#algorithms-and-data-structures)
- [Asymptotic notations](#asymptotic-notations)
  - [Complexity](#complexity)
  - [Asymptotic notations](#asymptotic-notations-1)
    - [Formal defenitions](#formal-defenitions)
    - [Big O](#big-o)
    - [Omega](#omega)
    - [Theta](#theta)
    - [Examples](#examples)

<br>

# Bounds
## Upper and lower bounds
Let `S` is some set of numbers.<br>
**Lower bound** of `S` is a number that **less than or equal to** (`≤`) every element of `S`.<br>
**Upper bound** of `S` is a number that **greater than or equal to** (`≥`) every element of `S`.<br>

Let `S = {5, 10, 50, 70, 100}`, then:
- `5` is a **lower bound** for the set `S`, and so is `4` and `3`;
- `101` is a **upper bound** for the set `S`, and so is `102` and `103`;
- `7` is **not** a **lower bound** for the set `S` because it is **not** *less than or equal to* every element of `S`;

<br>

## Tight bounds
**Tight lower bound** (aka **infimum**) is the **greatest** *lower bound*, i.e. there is no **greater** value that can be *lower bound*.<br>
**Tight upper bound** (aka **supremum**) is the **least** *upper bound*, i.e. there is no **smaller** value that can be *upper bound*.<br>

Let `S = {5, 10, 50, 70, 100}`, then:
- `5` is a **infimum**, `6` is **not** *lower bound*, so `5` is the **greatest** *lower bound*;
- `100` is a **supremum**, `99` is **not** *upper bound*, so `100` is the **lowest** *upper bound*;

<br>

## Bound of functions
The defenition of **bound** can be generalized to functions.<br>
A function `g` defined on domain `D` is an **lower bound** of `f` if `g(x) ≤ f(x)` for each `x` in `D`.
A function `g` defined on domain `D` is an **upper bound** of `f` if `g(x) ≥ f(x)` for each `x` in `D`.

<br>

# Algorithms and Data structures
An **algorithm** /ˈælɡərɪðəm/ is any well-defined **computational** /kɒm.pjəˈteɪ.ʃən.əl/ **procedure** /prəʊˈsiːdʒər/ that takes some value, or set of values, as **input** and produces some value or set of values, as **output**. In other words, an **algorithm** is a **finite sequence of steps** that **transforms** the **input** to **output**.<br>

An **algorithm** is said to be **correct** if it **halts** with the **correct output** for **every input**.<br>
We say that a *correct algorithm* **solves** *computational problem*.<br>

<br>

A **data structure** is a way to **store** and **organize** data in order to **facilitate access** and **modifications**.<br>

<br>

# Asymptotic notations
## Complexity
**Complexity** of an algorithm is the amount of **computational resources** required to run an algorithm.<br>
**Time complexity** measures **running time** of an algorithm.<br>
**Space complexity** measures **amount of memory** required by an algorithm.<br>

The usual units of time (**seconds**, **minutes** etc.) are **not** used because they are too dependent on a specific computer. Algorithm’s running time is measured in **steps**. **Number of steps** in an algorithm is its **invariant**, it’s an **intrinsic** feature of the algorithm.<br>

Both **time complexity** and **space complexity** are generally expressed as **function** depending on the **size** of the **input**: `f(n)`.<br>

There are 3 type of complexity:
- **average-case**;
- **worst-case** (the **longest** running time or the **maximum** of RAM for **any** input of size `n`);
- **best-case** (the **shortest** running time or the **minimum** of RAM for **any** input of size `n`);

<br>

Knowing **worst-case** provides a **guarantee** that the algorithm will never take any longer.<br>

<br>

## Asymptotic notations 
**Asymptotic notations** desribe how the **complexity** (**running time** or **space**) of an algorithm **grows** as the *input size* `n` **increases**.<br>
So, **asymptotic notations** allows **compare** the **asymptotic efficiency** of algorithms.<br>

**Asymptotic notations** of the **complexity** of an algorithm is also called **rate of growth** or **order of growth**.<br>

In **asymptotic notations** all **constants** and **lower-order terms** are ignored since they are **less significant** than the rate of growth of **leading term** of function for large values of `n`.<br>

But, an algorithm whose running time has a **higher** *order of growth* might take **less time** for **small inputs** than an algorithm whose running time has a **lower** *order of growth*. **But** on **large enough inputs** an algorithm witha **higher** *order of growth* **will lose**.<br>

<br>

There are **5 asymptotic notations**:
|Notation|Notation’s name|Description|
|:-------|:--------------|:----------|
|`fn(n) = O(g(n))`|**Big O**|It is like `<=`. It is **asymptotic upper bound** of the complexity. It describes the **worst** case, or **maximum** computational time.|
|`fn(n) = Ω(g(n))`|**Omega**|It is like `>=`. It is **asymptotic lower bound** of the complexity. It describes the **best** case, or **minimum** computational time.|
|`fn(n) = Θ(g(n))`|**Theta**|It is like `==`. It is a **asymptotic tight bound** of the complexity (aka **asymptotic exact bound**). It describes the **average** case.|
|`fn(n) = o(g(n))`|**Little o**|It is like `<`.|
|`fn(n) = ω(g(n))`|**Little Omega**|It is like `>`.|

**Asymptotic tight** bound implies that **asymptotic upper** and **asymptotic lower** bounds of an algorithm are **the same**.<br>

<br>

### Formal defenitions
- **Big O** denotes the **set of functions**: `O(g(n)) = {f(n): there exists constants c>0 and n₀>0 such that 0 ≤ f(n) ≤ c*g(n) for all n ≥ n₀}`;
- **Omega** denotes the **set of functions**: `Ω(g(n)) = {f(n): there exists constants c>0 and n₀>0 such that 0 ≤ c*g(n) ≤ f(n) for all n ≥ n₀}`;
- **Theta** denotes the **set of functions**: `Θ(g(n)) = {f(n): there exists constants c1>0, c2>0 and n₀>0 such that 0 ≤ c1*g(n) ≤ f(n) ≤ c2*g(n) for all n ≥ n₀}`;
- **Little o** denotes the **set of functions**: `o(g(n)) = {f(n): for any c>0 there is exists n₀>0 such that 0 ≤ f(n) < c*g(n) for all n ≥ n₀}`;
- **Little ω** denotes the **set of functions**: `ω(g(n)) = {f(n): for any c>0 there is exists n₀>0 such that 0 ≤ c*g(n) < f(n) for all n ≥ n₀}`;

**where**:
- `f(n)` represents the function is being analyzed, typically algorithm complexity (**running time** or **space**);
- `g(n)` represents function that bounds `f(n)`;
- `c`, `c1`, `c2` are constants;
- `n₀` is the minimum input size beyond which inequalities are hold;

<br>

**Notes**:
- `Θ(g(n)) = O(g(n)) ∩ Ω(g(n))` (`Θ` is an **intersection** of `O` and `Ω`);
- `o(g(n)) ⊊ O(g(n))` (`o` is a **proper subset** of `O`);
- `ω(g(n)) ⊊ Ω(g(n))` (`ω` is a **proper subset** of `Ω`);

<br>

**Difference** between **little** and **big**:
- The main difference between **little o** and **big O** is that in **big O** the bound inequality holds for **some** constant `c>0`, but in **little o** the bound inequality holds for **all** constants `c>0`.
- The main difference between **little ω** and **omega** is that in **omega** the bound inequality holds for **some** constant `c>0`, but in **little ω** the bound inequality holds for **all** constants `c>0`.
- The **big O** shows that some function `f(n)` **doesn't** grow **faster** than another function `g(n)`.
- The **little O** shows that some function `f(n)` grows **slower** than another function `g(n)`.
- If **two functions** grow at the **same rate**, **big O can** be used, but **little o cannot**.
- If `f(n) = o(g(n))` we say that `f(n)` is **asymptotically smaller** than `g(n)`.
- If `f(n) = ω(g(n))` we say that `f(n)` is **asymptotically larger** than `g(n)`.

<br>

Let some algorith has **running time** that grows as `n²`, then both `n² = O(n³)` and `n² = O(n⁴)` are **true**, but the former **more precise**.<br>
So, if someone estimates that upper bound of **running time** of an algorithm is `O(n⁴)` there is **no guarantee** that `n⁴` is the *best approximation* of **running time**.<br>

<br>

Because asymptotic notation denotes **set of functions** , we, for example, could write `f(n) ∈ O(g(n))` to indicate that `f(n)` is a member of `O(g(n))`. **Instead** we usually write `f(n) = O(g(n))`.<br>

The **asymptotic upper bound** provided by **big O may** or **may not** be **asymptotically tight**. The bound `2n² = O(n²)` is *asymptotically tight*, but the bound `2n = O(n²)` is **not**. We use **little o** to denote an **upper bound** that is **not** *asymptotically tight*.<br>

<br>

### Big O
![Big O](/img/big_o.png)

<br>

### Omega
![Omega](/img/omega.png)

<br>

### Theta
![Theta](/img/theta.png)

<br>

### Examples
Consider function `f(n) = n²`, then all statements below are **true**:
- `f(n) = O(n³)`;
- `f(n) = O(n²)`;
- `f(n) = o(n⁴)`;
- `f(n) = Θ(n²)`;
- `f(n) = Ω(n)`;
- `f(n) = Ω(n²)`;

<br>