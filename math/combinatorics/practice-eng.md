# Table of contents
- [Table of contents](#table-of-contents)
- [Example 1](#example-1)
- [Example 2](#example-2)
- [Example 3](#example-3)
- [Example 4](#example-4)
- [Example 5](#example-5)

<br>

# Example 1
A restaurant offers 2 salads and 4 entrees.
1. How many different items can you choose if you get to choose 1 item?
2. How many different meals if you can choose 1 salad and 1 entree?

Solution<br>
To choose 1 item you can choose 1 salad **OR** 1 entree. There is no overlap. This means **AP**.<br>
So, $2+4=6$ items.<br>

You are choosing 1 salad **AND** 1 entree. You choose the second event **after** the first event occurs. This means **MP**.<br>
So, $2×4=8$ meals.<br>

<br>

# Example 2
A lock will open with the correct choice of 3 numbers from 0 to 99 inclusive.<br>
1. How many different sets of 3 numbers are there if the numbers **can** repeat?
2. How many different sets of 3 numbers are there if the numbers **cannot** repeat?

Solution<br>
All three numbers are needed. You need the first number **AND** the second number AND the third number. This is **MP**. There are 100 choices for each number.<br>
so, $100·100·100=1,000,000$.<br>

**Without** repeating means that after the first number is chosen, there is one less number available to choose. After the first two numbers are chosen, there are two less numbers to choose from.<br>
So, $100·99·98=970,200$.<br>

<br>

# Example 3
Find the **number of all solutions** of following inequality $`x^2 + y^2 <= 5`$.<br>
We can divide it into 6 disjoint cases: $`x^2 + y^2 = i`$, where $0 <= i <= 5$.<br>
Let $Si$ is set of all solutions for case $i$. Let denote **solution** as an **ordered** pair $(x,y)$.<br>
Then,<br>
- $i=0$: $|S0| = 1$, $`S_0 = \{(0,0)\}`$;
- $i=1$: $|S1| = 4$, $`S_1 = \{(1,0), (-1,0), (0,1), (0,-1)\}`$;
- $i=2$: $|S2| = 4$;
- $i=3$: $|S3| = 0$, $`S_3 = \{\} = ∅`$;
- $i=4$: $|S4| = 4$;
- $i=5$: $|S5| = 8$;

<br>

So, the **number of all solutions** = $`|S_0| + |S_1| + |S_2| + |S_3| + |S_4| + |S_5| = 1 + 4 + 4 + 0 + 4 + 8 = 21`$.

<br>

# Example 4
Consider sequence of cities: $A \rightarrow B \rightarrow C \rightarrow D$ and consider that there are **2 ways** from $A$ to $B$, **3 ways** from $B$ to $C$ and **4 ways** from $C$ to $D$.<br>
Let denote $S \rightarrow D$ set of all ways between **adjacent cities** $S$ and $D$.<br>
Obvious that all such sets in path $A \rightarrow B \rightarrow C \rightarrow D$ ($A \rightarrow B$, $B \rightarrow C$, $C \rightarrow D$) are **pairwise disjoint**.<br>
So, there are $2 * 3 * 4 = 24$ ways to choose **one** way $(a,b,c,d) \  \text{where} \  a∈A, b∈B, c∈C, d∈D$ from $A$ to $D$ or **equaly** the **total number** of ways from $A$ to $D$ is $2 * 3 * 4 = 24$.<br>

<br>

# Example 5
Let $`X = \{1,2, ..., 100\}`$ and let $`S = \{(a,b,c): a,b,c ∈ X, a \lt b \ \text{and} \  a \lt c\}`$. **Find** $|S|$.<br>

We can divide it into **99 disjoint cases**: $a=1, a=2, ..., a=99$.<br>
For every such case there are $(100 - a)$ choices for $b$ and $c$.
So, we use **AP** to sum results of **all** cases and **MP** to count variants for **concrete** case:
- the number of **ordered** triples $(a,b,c)$ for concrete value of $a$ is $(100-a) \times (100-a)$;
- the number of **ordered** triples $(a,b,c)$ for **all** values of $a$ is the sum of $`(100-i)^2$ for $i$ in $[1,99]`$;

So, $`|S| = 99^2 + 98^2 + ... + 1^2`$.