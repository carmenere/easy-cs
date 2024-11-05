# Logical reasoning
## Claim vs. Statement vs. Proposition
- A **statement** is a **sentence** which can be `true` or `false`.
- A **proposition** is a **statement** that the is *proposed for proof*.
- A **claim** is a **proposition** that is *claimed to be* `true`.

<br>

> **Note**:<br>
> They **all** can be used **interchangeably** in the meaning of **statement**.<br>

<br>

## Logical reasoning
**Logical reasoning** is a moving from **premises** to **logical consequences** (or **conclusions**). Inferred **logical consequences** also can be used futher as **premises** in next iterations of **logical reasoning**.<br>

**Logical reasoning** happens in the form of **logical arguments** (or just **arguments**).<br>

A **argument** is a **series of statements**, **some** of which are called **premises** and **one** is the **conclusion**. So, an **argument**  consists of **premises** and **conclusion**.<br>

The sign `⊢` is a **sign** of **logical consequence**.<br>

<br>

## Validity and soundness
- An argument is **valid** *if and only if* when all its **premises** are `true` the **conclusion** **must** also be `true`. In a **valid argument** if *at least one* **premise** is `false` then the **conclusion** must be `false`.
- An argument is **sound** *if and only if* it is **valid** and **all** its premises are `true`.

<br>

**Validity** and **soundness** apply only to **arguments**, **not** to **premises**.

<br>

An example of a **valid** and **sound** argument:
1. All men are mortal. (**premise**, `True`)
2. Socrates is a man. (**premise**, `True`)
3. Therefore, Socrates is mortal. (**conclusion**, `True`)

<br>

## Inductive reasoning vs. Deductive reasoning
1. **Inductive reasoning**: `Observation` -> `Pattern` -> `Hypothesis` -> `Theory`.
2. **Deductive reasoning**: `Theory` -> `Hypothesis` -> `Observation` -> `Confirmation`.

<br>

## Rules of inference
A **rule of inference** (or **inference rule**) is a logical form consisting of a function which takes **premises**, analyzes their syntax, and **returns a conclusion**.<br>

<br>

Popular **rules of inference**:
- **modus ponens**;
- **modus tollens**;
- **contraposition**;

<br>

Popular notations **rules of inference**:
- **standard form** (or **vertical notation**);
- **sequent notation** (`⊢`);

<br>

### Standard form (or vertical notation)
```bash
Premise_1
Premise_2
...
Premise_N
----------
Conclusion
```

<br>

### Sequent notation (⊢)
```bash
Premise_1, Premise_2, ..., Premise_N  ⊢  Conclusion
```

<br>

## Modus ponens /ˈmoʊdəs ˈpoʊnɛnz/
`A → B, A ⊢ B`

<br>

Human-readable script:
1. If hypothesis `A` is true then consequent `B` is also `true`.
2. We know that hypothesis `A` is `true`.
3. So, consequent `B` is `true`.

<br>

## Modus tollens /ˈmoʊdəs ˈtɒlɛnz/
`A → B, not B ⊢ not A`
<br>

Human-readable script:
1. If hypothesis `A` is true then consequent `B` is also `true`.
2. We know that consequent `B` is `false`.
3. So, hypothesis `B` is `false`.

<br>

## The law of contraposition
The **law of contraposition** says that a **conditional statement** is `true` if, and only if, **its contrapositive** is `true`: `A → B ⊢ not B → not A`.
