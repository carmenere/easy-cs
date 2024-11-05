# Probability
**Probability** is the branch of mathematics concerning **events** and numerical descriptions of **how likely they are to occur**.<br>
The **probability of an event** is a number in range `[0, 1]`: the larger means the more likely an event is to occur.<br>

<br>

# Probability space
**probability space** is a **mathematical triplet** `(Ω, 𝓕, P)` that provides a **formal model** of a **random process** or **experiment**.
A **probability space** consists of **three elements**:
1. A **sample space**, `Ω`, which is the **set of all possible outcomes** of our probabilistic **experiment**. An **outcome** `ω` is the **result** of a **single** *experiment*.
2. An **event space**, `𝓕`, which is a **set of events**. An **event** is a **set** of *zero or more* **outcomes**. So, an **event** is a **subset** of the **sample space**.
3. A **probability function**, `P`, which assigns each event in the event space a **probability**, which is a number between `0` and `1`. More formally, `P` a function on `𝓕`: `P: 𝓕 -> [0,1]`.

<br>

## Sample space
![probability_space_outcomes](/img/probability_space_outcomes.jpeg)

<br>

## Event space
![probability_space_events](/img/probability_space_events.jpeg)

<br>

More formally, the **event space** is a **sigma-algebra** (`σ-algebra`, `σ-field`) **on** the **sample space**.

For **finite** sample space it contains `2^Ω` subsets of `Ω`.<br>
The elements of `σ-algebra` are called **measurable sets**.<br>

The event space is a sigma-algebra on the sample space. It is a **subset** of the **power set** of `Ω` (`𝒫(Ω)`) whose elements called **events**.<br>
When `Ω` is **finite** or **countable**, the `𝒫(Ω)` is a valid `σ-algebra` that can be used as the **event space**.<br>
When `Ω` is **uncontinue**, the `𝒫(Ω)` is `borel σ-algebra`.<br>

**Event space** `𝓕 ⊆ 𝒫(Ω)` properties:
- the **sample space** itself is an **event** (aka **sure event**): `Ω ∈ 𝓕`;
- `A ∈ 𝓕` => the **complement of a set A** (`A'`) is **closed under complementation**;
- `A1,A2, ..., A𝑖 ∈ 𝓕, ` => `∪𝑖A𝑖 ∈ 𝓕` **closed under countable unions**;

<br>

## Probability measure
![probability_space_measure](/img/probability_space_measure.jpeg)

<br>

A pair `(Ω, 𝓕)` is called a **measurable space**.<br>
Any element of a `σ-algebra` is called a **measurable set**.<br>
This is because we can define a function that **measures** the size of the elements of **all** the elements of `𝓕`.<br>

<br>

Suppose you have a measurable space `(Ω, 𝓕)`, then function `𝑓: 𝓕 -> [0, +∞)` is called a measure on `(Ω, 𝓕)` if it **returns zero to the empty set** and if it is **countably additive**:
- `P(∅) = 0`
- `P(∪𝑖A𝑖) = 𝚺𝑖 P(A𝑖)` for `A𝑖 ∈ 𝓕`

<br>

A **probability measure** is a special case of a measure with **additional properties**:
- it maps `𝓕` to `[0, 1]` rather than to `[0, +∞)`.<br>
- the **measure of the whole probability space** is **equal to one**: `P(Ω) = 1`.<br>

<br>

### Example

#### Coin toss
**Experiment**: a single **toss of a coin**. Its **possible outcomes**: `ω1 = Head` and `ω2 = Tail`.<br>
Its **sample space** to be `Ω = {1,2,3,4,5,6}`.

<br>

#### Die
**Experiment**: a single **throw of a die**. Its **possible outcomes**: `ω1 = 1`, `ω2 = 2`, `ω3 = 3`, `ω4 = 4`, `ω5 = 5`, `ω6 = 6`.<br>
Its **sample space** to be `Ω = {1,2,3,4,5,6}`.<br>

For the **event space**, we could simply use the **set of all subsets** of the sample space, which would then contain simple events such as `{5}` as well as complex events such as `{2,4,6}`.
For the **probability function**, we would map each event to its probability (the number of outcomes in that event divided by `6`). For example, `{5}` would be mapped to `1/6`, and `{2,4,6}` would be mapped to `3/6 = 1/2`.

When an **experiment** is **conducted**, we imagine that nature selects a single **outcome**, `ω`, from the sample space `Ω`.<br>
All the events in the event space `𝓕` that contain the selected outcome `ω` are said to **have occurred**.<br>
This selection happens in such a way that **if the experiment were repeated many times**, the **number of occurrences of each event** would most likely tend **towards** the **probability** assigned to that event by the probability function.

<br>

The Soviet mathematician Andrey Kolmogorov introduced the notion of probability space, together with other axioms of probability, in the 1930s.<br>

<br>

# Complete probability space
A probability space `(Ω, 𝓕, P)` is said to be a **complete probability space** if for all `B ∈ 𝓕` with `P(B) = 0` and all `A ⊂ B` the `𝓕` also contains `A`: `A ∈ 𝓕`. 

<br>

# Random variable
A **random variable** is also called a **measurable function**.<br>

Suppose you have two measurable spaces. One could be the pair of sample space and event space `(Ω, 𝓕)`, and the other one could be some other arbitrary pair of a set and of its sigma algebra `(E, 𝓔)`, although usually we choose `E = R` and `𝓔 = B(R)`.<br>

Then a **measurable function** (or **random variable** `X`) is a function that maps elements in `Ω` to elements in `E`: `X: Ω -> E` with additional properties.<br>
This function maps **outcomes** of `Ω` to **elements** of `E`, it **does not map events**.<br>

Additional properties:
- the **preimage** `X^-1(B)` of any **𝓔-measurable** set `B ∈ 𝓔` is a **𝓕-measurable** set: `X^-1(B) = {ω ∈ Ω: X(ω) ∈ B } ∈ 𝓕 ∀B ∈ 𝓔`;

<br>

In the diagram below we can see how the set `B`, which is an element of `𝓔` has a **preimage**, which we call `X^-1(B)`, which is an element of `𝓕`:
![Examples](/img/probability_space_random_variable.jpeg)

<br>

# Examples
![Examples](/img/pspace_examples.jpeg)
