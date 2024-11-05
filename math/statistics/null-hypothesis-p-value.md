# Hypothesis
The **null hypothesis** (often denoted as `H₀`) is a **statement** that there is **no** significant difference, effect, or relationship in the population. In other words, `H₀` represents the **status quo** or the **default assumption about something**.<br>
The **null hypothesis** is typically formulated to assert **no** relationship exists between two sets of data or variables being analyzed.<br>
An **alternative hypothesis** (often denoted as `H₁`) is an **opposite statement** to the **null hypothesis**.<br>

So, `H₀` and `H₁` work as a **complementary pair**, each stating that the other is wrong.<br>

<br>

# p-value
A **p-value** (aka **probability value**) is the **probability** of obtaining the **sample results** from some **experiment**, **given the assumption** that `H₀` is **true**.<br>

A **low** *p-value* means that the *sample results* would be **unlikely** if the `H₀` were **true** and leads to the **rejection** of the `H₀`.<br>
A **high** *p-value* means that the *sample results* would be **likely** if the `H₀` were **true** and leads to the **retention** of the `H₀`.<br>

If *p-value* is **very small**, that means
- We are so "lucky" to get this **very rare** *sample results*.
<br>
OR
- Our assumption (`H₀` is *true*) is **incorrect** and we must **reject** `H₀`.

<br>

But **how low must the p-value be** before the *sample results* is considered **unlikely enough** to **reject** the `H₀`?<br>

There is a **threshold** called the **significant level** (often denoted as `α`) or **alpha**.<br>
The **significance level** is a **criterion** (/kraɪˈtɪərɪən/) to **reject** or **not** the **null hypothesis**:
- if `p-value ≤ α`, we **reject** the `H₀`;
- if `p-value > α`, we **fail to reject** the `H₀`;

<br>

The **significant level** `α` must be chosen **before** experiment. The **significance level** is typically set to `5%` (`0.05`).<br>

When the `H₀` is **rejected** we say the result of the test is **statistically significant**. 

<br>

# Hypothesis: **accept** or **fail to reject**?
There are two ways to indicate that the `H₀` is **not rejected**: **accept** the `H₀` and **fail to reject** the `H₀`. Both these phrases mean the same, but in statistics, the meanings are somewhat different.<br>
The phrase **accept** the `H₀`' implies that the `H₀` is **by nature true**, and it is **proved**.<br>
Thus, it is **always advisable** to state **fail to reject** the `H₀` instead of **accept** the `H₀`.

<br>

# P(R|H₀)` vs. `P(H₀|R)
`P(Observed result|H₀)` is **not equal** to `P(H₀|Observed result)`.