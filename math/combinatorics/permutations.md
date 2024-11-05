# Table of contents
- [Table of contents](#table-of-contents)
- [k-tuples](#k-tuples)
- [Permutations](#permutations)
  - [Permutations without repetitions](#permutations-without-repetitions)
  - [Proof](#proof)
    - [Trivial case](#trivial-case)
    - [More general case](#more-general-case)
  - [Examples](#examples)
  - [Permutations with repetitions](#permutations-with-repetitions)
    - [Proof](#proof-1)
    - [Trivial case](#trivial-case-1)
    - [More general case](#more-general-case-1)
- [Permutations of multisets](#permutations-of-multisets)
  - [Number of ways to put objects into bins](#number-of-ways-to-put-objects-into-bins)

<br>

# k-tuples
A **tuple** is an **ordered collection** of elements taken from a set and may have **duplicate** elements.<br>
A **tuple** of size $k$ (**tuple** of $k$ elements) is called an $k-tuple$.<br>
**Notation**: **parentheses** $( )$ or **angle brackets** $⟨ ⟩$.<br>
**Example**: $(2,7,4,1,7)$ denotes a **5-tuple**.<br>
**Orderedness**: $(1,2,3)$ is **different** from $(3,2,1)$, i.e. $(1, 2, 3) ≠ (3, 2, 1)$ .<br>

The **general rule** for the identity of two **k-tuples** is $(a_1, a_2, …, a_n) = (b_1, b_2, …, b_n)$ **if and only if** $a_1 = b_1$, $a_2 = b_2$, ..., $a_n = b_n$.<br>

<br>

# Permutations
A **permutation** is an **arrangement** of items in a **particular order**, i.e. **order matters**.

<br>

## Permutations without repetitions
Consider a set $`A = \{1,2,3, ..., n\}`$.<br>
A **$k$-permutation of $n$** *without repetitions* (aka **partial permutations**, **k-arrangements of n**) is **all ordered tuples** of length $k, \text{where} \  k \le n$ that consist of **distinct** elements from some set $A, |A| = n$. In other words it is the number of **k-tuples** that can be selected from set $`A = \{1, 2, . . . , n\}`$ **without** repeating elements.<br>
The **number** of all such **$k$-permutation of $n$** is denoted as $P(n,k)$ or $`{\displaystyle P^{n}_{k}}`$ or $A(n,k)$ or $`{\displaystyle A^{n}_{k}}`$.<br>

When $k = n$, **$k$-permutation of $n$** is simply called **permutation of n**.<br>
The **number** of all such **permutations of n** is denoted as $P(n)$ or $P_n$.<br>

<br>

**Formulas**:
- $`{\displaystyle A^{n}_{k} = \frac{n!}{(n-k)!}}`$
- $`{\displaystyle P_n = n!}`$

<br>

## Proof
Main idea: when we form **$k$-permutation of $n$** *without repetitions* we select element from set **without** returning it back. So, after each such selection we get **new set** with **decremented** *cardinality*.<br>

<br>

### Trivial case
Consider finite set $A = \{1,2,3\}, |A| = 3$. Find number of all **3-tuples** $(a,b,c)$ *without repetitions* from $A$.<br>
Steps:<br>
1. We have $|A|$ **choices** to select element from $A$. Take element $1$ from $A$ **without returning** it back, then we get new set $`B = A \setminus \{1\} = \{2,3\}`$ with cardinality $|A| - 1 = 2$.
2. We have $|B|$ **choices** to select element from $B$. Take element $2$ from $B$ **without returning** it back, then we get new set $`C = B \setminus \{2\} = \{1\}`$ with cardinality $|B| - 1 = 1$.
3. We have $|C|$ **choices** to select element from $C$. Take element $1$ from $C$ **without returning** it back, then we get new **empty** set $`D = C \setminus \{1\} = \{\}`$ with cardinality $|C| - 1 = 0$.

The **total number** by **MP** of all **3-tuples** $(a,b,c)$ *without repetitions* from $A$ is equal to the cardinality of cartesian product of sets $A$, $B$, $C$:
- $A \times B \times C = |A| \times |B| \times |C| = 3 * 2 * 1 = 6$.<br>

<br>

After all steps we get **3** sets: $`A = \{1,2,3\}`$, $`B = \{1,2\}`$, $`C = \{1\}`$. We see that choice in next step is **independent** from all choices in previous steps. It implies **MP**.<br>

Consider **3** another sets: $`X = \{x_1,x_2,x_3\}`$, $`Y = \{y_1,y_2\}`$, $`Z = \{z_1\}`$.<br>

**MP** gives **6** following *3-tuples* $(x,y,z)$ *without repetitions* from $X \times Y \times Z$:
- $(x_1,y_1,z_1)$
- $(x_1,y_2,z_1)$
- $(x_2,y_1,z_1)$
- $(x_2,y_2,z_1)$
- $(x_3,y_1,z_1)$
- $(x_3,y_2,z_1)$

Consider we can assign to $`x_i`$, $`y_i`$ and $`z_i`$ values from the set $A$.<br>
Then, if we **require** that $`x_i ≠ y_i`$, $`y_i ≠ z_i`$, $`x_i ≠ z_i`$ it will the **same as** we choose **3-tuples** *without repetitions* from $A$.<br>

<br>

### More general case
Consider finite set $`A = \{1,2, ..., n\}, |A| = n`$. Find number of all **3-tuples** $(a,b,c)$ *without repetitions* from $A$.<br>
Steps:<br>
0. Consider $`A_1 = A`$, so $`|A_1| = n`$.
1. We have $`|A_1|`$ **choices** to select element from $`A_1`$. Take element from $`A_1`$ **without returning** it back, then we get new set $`A_2 = A_1 \setminus \{a\}`$ with cardinality $`|A_1| - 1 = n-1`$.
2. We have $`|A_2|`$ **choices** to select element from $`A_2`$. Take element from $`A_2`$ **without returning** it back, then we get new set $`A3 = A_2 \setminus \{a\}`$ with cardinality $`|A_2| - 1 = n-2`$.
3. And so on.

<br>

Then
- $`P^{n}_{k} = A_1 \times A_2 \times ... \times A_n = |A_1| \times |A_2| \times ... \times |A_n| = n * (n-1) * ... * (n - k + 1)`$

We can **complement** $`P^{n}_{k} = n * (n-1) * ... * (n - k + 1)`$ to $n!$ by **multiplying** it by $(n-k)!$, where $(n-k)! = (n-k) * (n-(k+1)) * (n-(k+2)) * 3 * 2 * 1$.<br>

So,
- $`P^{n}_{k} * (n-k)! = n!`$

<br>

**Finally**:
- $`{\displaystyle P^{n}_{k} = \frac{n!}{(n-k)!}}`$.

<br>

## Examples
Consider $`A = \{a\}`$. Then, there is only **one 1-permutaton** of $A$:
- $(a)$;

Consider $`A = \{a,b\}`$. Then, there are only **two 2-permutatons** of $A$:
- $(a,b)$;
- $(b,a)$;

Consider $`A = \{a,b,c\}`$. Find **all 3-permutatons** of $A$.
Divide problem into 2 disjoint cases. There are only **two 2-permutatons** of ${a,b}$. 
Count all new permutations for each permutation of ${a,b}$:
1. Adding $c$ to ordered pair $(a,b)$ gives us **three** new 3-tuples:
   - $(c,a,b)$
   - $(a,c,b)$
   - $(a,b,c)$
2. Adding $c$ to ordered pair $(b,a)$ gives us **three** new 3-tuples:
   - $(c,b,a)$
   - $(b,c,a)$
   - $(b,a,c)$

According **AP** we have $3 + 3 = 6$ total **3-tuples** of $`\{a,b\} + \{c\} = \{a,b,c\} = A`$.

<br>

## Permutations with repetitions
A **$k$-permutation of $n$** *with repetitions* is **all ordered pairs** of length $k, k \le n$ that consist of elements from $A, |A| = n$.<br>
The **number** of such **$k$-permutation of $n$** is denoted as $`{\displaystyle \overline{P(n,k)}}`$ or $`{\displaystyle \overline{P}^{n}_{k}}`$ or $\overline{A(n,k)}$ or $`{\displaystyle \overline{A^{n}_{k}}}`$.<br>

<br>

**Formula**:
- $`{\displaystyle \overline{A_{k}^{n}} = n^k}`$

<br>

### Proof
When we form **$k$-permutation of $n$** *with repetitions* we **return** back selected element to **original set**. So, on **every step** we use the **original set** and its cardinality isn't changed.<br>

<br>

### Trivial case
Example, consider finite set $`A = \{1,2,3\}, |A| = 3`$. Find number of all **3-tuples** $(a,b,c)$ *with repetitions* from $A$. <br>
Steps:<br>
1. Take element from $A$ then **return it back** to $A$. Here we have $|A|$ **choices** to select element from $A$.
2. Take element from $A$ then **return it back** to $A$. Here we have $|A|$ **choices** to select element from $A$.
3. Take element from $A$ then **return it back** to $A$. Here we have $|A|$ **choices** to select element from $A$.

<br>

Acording **MP** the **total number** of all **3-tuples** $(a,b,c)$ is equal to the *cardinality* of cartesian product of sets $A$, $A$, $A$:
- $A \times A \times A = |A| \times |A| \times |A| = 3^3 = 27$

<br>

### More general case
Consider finite set $`A = \{1,2, ..., n\}, |A| = n`$. Find all **k-tuples** *with repetitions* from $A$.<br>
Then:
- $`\overline{P}^{n}_{k} = A \times A \times ... \times A = |A| \times |A| \times ... \times |A| = n^k`$

<br>

# Permutations of multisets
A **permutations of multiset** $M$ is the number of **all ordered arrangements** of elements from **multiset** $M$. The **permutations of multiset** is also called **multinomial coefficient**.<br>
The **number of occurrences** $`m_i`$ of a specific element in $M$ is called its **multiplicity**. The **sum** of all **multiplicities** of the elements of **multiset** $M$ is **always equal** to the **cardinality** of $M$.<br>

<br>

**Formula**:
- $`{\displaystyle P^{n}_{m_1,m_2,...,m_k} = \frac{n!}{m_1!*m_2!*...*m_k!}}`$
  - where:
    - $`m_1`$, $`m_2`$, ..., $`m_k`$ are **multiplicities** of the elements of $M$
    - $n$ is the cardinality of $M$

<br>

## Number of ways to put objects into bins
The number of ways to **put objects into bins** is equal to the *multinomial coefficient* $`P^{n}_{m_1,m_2,...,m_k}`$, where:
- $k$ is a **number of all bins**;
- $`m_i`$ is a **number of elements in bin** $i$;
- $n$ is is the **number of all objects**;
