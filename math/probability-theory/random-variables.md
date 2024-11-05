# Random variable
A **random variable** is one whose value is determined by the **outcome** of a **random experiment**.<br>

<br>

## Notation
`X` or `ξ` (*Xi* letter) denotes a **random variable** and `x` or `ω` denotes a **value** of the *random variable* in an experiment (**outcome**).<br>
**Random variable** represents an **event** that is a **subset** of the **sample space**.<br>
In general, a **random variable** takes on various values within the range `−∞ < x < ∞`.<br>

The **values of a random variable** are **real numbers ssociated** with the **outcomes** of an **experiments**.<br>

#### Example
Consider an experiment **A** in which **two** coins are tossed simultaneously.<br>
Let `S` be the sample space of possible outcomes associated with the experiment: `S = {HH, HT, TH, TT}`.<br>

If we define a random variable `X`, as the number of heads, then the values of `X` are `0`, `1`, and `2` corresponding to the outcomes.<br>

So, we have the following table:<br>
|Variable|Value of outcome TT|Value of outcome TH|Value of outcome HT|Value of outcome HH|
|:-------|:------------------|:------------------|:------------------|:------------------|
|X = x<sub>i</sub>|0|1|1|2|

<br>

## Discrete vs. continuous
Random variables can be discrete, i.e., taking any of a specified finite or countable list of values.
Random variables can be continuous, taking any numerical value in an interval or collection of intervals.

<br>

## Scalar and vector
**Random variables** may be both **scalar** and **vector**.

In correspondence with general definition of a vector we shall call a vector random variable or a random vector any ordered set of scalar random variables. Thus, for instance, an 

A **n-dimensional random vector** (**n-dimensional vector random variable**) `X` is a set of `n` scalar **random variables** `X1`, ..., `Xn`.
These **random variables** `X1`, ..., `Xn` are called the **components** of the **random vector** `X`.

<br>

# Probability distribution
The mathematical function describing the possible values of a random variable and their associated probabilities is known as a **probability distribution**.

**Cumulative distribution function** (**CDF** or just **distribution function**) of a *random variable* `ξ`, is the **probability** that `ξ` will take a value **less than or equal** to `x`: `F(x) = P(ξ <= x)`.

A *random variable* `ξ` has a **probability density function** (**PDF**, **density function** or just **density**) if and only if it **continuous** *random variable* `ξ` and its **CMF** `F(x)` is **absolutely continuous**.

**Definite integral** of **PDF** shows the **probability** that **continuous** *random variable* `ξ` will take a values from range `[a, b]`.

It is conventional to use a capital `F` for a **cumulative distribution function**, in contrast to the lower-case `f` used for **probability density functions** (**PDF**) and **probability mass functions** (**PMF**).

<br>

Relationship:
- the **PDF** of a **continuous** *random variable* `ξ` is a **derivative** of its **CMF**;
- the **CDF** of a **continuous** *random variable* `ξ` ia an **integral** of its **PDF**;

<br>

Any **probability distribution** defines a **probability measure**.<br>
**Probability distributions** is defined in different ways for **discrete** or **continuous** variables.<br>

Both can be described by **cumulative distribution function** (**CDF**). **CDF** describes the **probability** that the **random variable** is **no larger** than a **given value**.<br>

Also:
- **continuous** probability distributions can also be described by the **probability density function** (**PMF**).
- **discrete** probability distributions can also be described by the **probability mass function** (**PDF**).

<br>

The **probability** that the **outcome lies in a given interval** can be computed by integrating the **PDF** over that interval.

<br>

## Examples
### PDF
![PDF](/img/pspace_pdf.jpeg)

<br>

### CDF
![CDF](/img/pspace_cdf.jpeg)

The value at `a` equals the **area** under the **probability density** curve **to the left of** `a`.

<br>

# Expected value
**Expected value** (or **first moment**) is the **arithmetic mean** of the possible values a random variable can take, weighted by the probability of those outcomes.<br>
Denoted `E(X)`, `E[X]`, `EX` or `M[X]`.<br>

The **sample mean** is the **average** of the values of a variable in a **sample**, which is the sum of those values divided by the number of values.

<br>

# Variance
The **variance** (`D[X]`) of a random variable `X` is the **expected value** of the **squared deviation** from the `D[X] = E[(X - E[X])^2]`.<br>

**Sample variance**
- **corrected** sample variance (or unbiased sample variance);
- **uncorrected** sample variance (or biased sample variation);

<br>

# Standard deviation
- Corrected sample standard deviation
- Uncorrected sample standard deviation

<br>

# Standard error
