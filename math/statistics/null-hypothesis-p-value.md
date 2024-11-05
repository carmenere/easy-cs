# Hypothesis
The **null hypothesis** (often denoted as $`H_{0}`$) is a **statement** that there is **no** significant difference, effect, or relationship in the population. In other words, $`H_{0}`$ represents the **status quo** or the **default assumption about something**.<br>
The **null hypothesis** is typically formulated to assert **no** relationship exists between two sets of data or variables being analyzed.<br>
An **alternative hypothesis** (often denoted as $`H_{1}`$) is an **opposite statement** to the **null hypothesis**.<br>

So, $`H_{0}`$ and $`H_{1}`$ work as a **complementary pair**, each stating that the other is wrong.<br>

<br>

# p-value
A **p-value** (aka **probability value**) is the **probability** of obtaining the **sample results** from some **experiment**, **given the assumption** that $`H_{0}`$ is **true**.<br>

A **low** *p-value* means that the *sample results* would be **unlikely** if the $`H_{0}`$ were **true** and leads to the **rejection** of the $`H_{0}`$.<br>
A **high** *p-value* means that the *sample results* would be **likely** if the $`H_{0}`$ were **true** and leads to the **retention** of the $`H_{0}`$.<br>

If *p-value* is **very small**, that means
- we are so "lucky" to get this **very rare** *sample results*;

**OR** 
- our assumption ($`H_{0}`$ is *true*) is **incorrect** and we must **reject** $`H_{0}`$;

<br>

But **how low must the p-value be** before the *sample results* is considered **unlikely enough** to **reject** the $`H_{0}`$?<br>

There is a **threshold** called the **significant level** (often denoted as `α`) or **alpha**.<br>
The **significance level** is a **criterion** (/kraɪˈtɪərɪən/) to **reject** or **not** the **null hypothesis**:
- if $`p{\text-}value ≤ α`$, we **reject** the $`H_{0}`$;
- if $`p{\text-}value > α`$, we **fail to reject** the $`H_{0}`$;

<br>

The **significant level** `α` must be chosen **before** experiment. The **significance level** is typically set to `5%` (`0.05`).<br>

When the $`H_{0}`$ is **rejected** we say the result of the test is **statistically significant**. 

<br>

# Hypothesis: **accept** or **fail to reject**?
There are two ways to indicate that the $`H_{0}`$ is **not rejected**: **accept** the $`H_{0}`$ and **fail to reject** the $`H_{0}`$. Both these phrases mean the same, but in statistics, the meanings are somewhat different.<br>
The phrase **accept** the $`H_{0}`$' implies that the $`H_{0}`$ is **by nature true**, and it is **proved**.<br>
Thus, it is **always advisable** to state **fail to reject** the $`H_{0}`$ instead of **accept** the $`H_{0}`$.

<br>

# $`P(R|H_{0})`$ vs. $`P(H_{0}|R)`$
$`{\displaystyle P(Observed \space result|H_{0})}`$ is **not equal** to $`P(H_{0}|Observed \space result)`$.