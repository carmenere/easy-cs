# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Parsing](#parsing)
- [Language](#language)
  - [Theorem: $`2^{S}`$ is **uncountable**](#theorem-2s-is-uncountable)
  - [Turing machines vs. Languages](#turing-machines-vs-languages)
    - [Theorem: *non-recursively enumerable* languages exist](#theorem-non-recursively-enumerable-languages-exist)
- [Grammar](#grammar)
  - [Theorem: number of languages is **uncountable**](#theorem-number-of-languages-is-uncountable)
  - [Formal grammars](#formal-grammars)
<!-- TOC -->

<br>

# Parsing
**Parsing** is the process of structuring a *linear representation* in a accordance with a *given grammar*.<br>
The *linear representation* may be a sentence, a who;e program, a piece of music, in short any linear sequence in which the preciding elements in some way restrict the next element.

Such *linear representations* are called **sentences**.<br>

<br>

# Language
A **language** is a **set of sentences** (aka **strings**) and *each sentence* is a *sequence of symbols*.<br>
The word **sequence** means that the symbols in each sentence are in a **fixed order**.<br>
Sentences in a language are composed of **finite set** of symbols called **alphabet**.<br>
*Every language* has an **alphabet**. The **alphabet** is denoted by $`\Sigma`$.<br>

*Sentences* are consist of **words** called **tokens**.<br>

There is **no limit** on *size of sentence*. So, a languange can be possibly of **infinite** *set of sentences* that can be made by combining letters from the **finite** *alphabet* $`\Sigma`$.<br>

Consider alphabet $`\Sigma = \{a, b\}`$. The language $`\Sigma^{*}`$ is an **infinite set** of all possible sentences that can be made from $`\Sigma`$.<br>
The ***** means that language contains **empty word** denoted by $`\varepsilon`$ which consists of zero symbols.<br>

So, $`\Sigma^{*} = \{\varepsilon, a, b, aa, bb, ab, ba, aaa, ... \}`$.<br>

The language $`\Sigma^{+}`$ means language without empty word: $`\Sigma^{+} = \Sigma^{*} - \varepsilon`$.<br>

The language $`\Sigma^{*}`$ has **interesting property** that **all languages using alphabet** $`\Sigma`$ **are subset of** $`\Sigma^{*}`$.<br>
In other words, the language $`\Sigma^{+}`$ contains any possible sentence over $`\Sigma^{*}`$.<br>

More formal:
- **any** *language* is a **some** *subset of* $`\Sigma^{*}`$;
- **any** *subset of* $`\Sigma^{*}`$ is a **language**;
- so, the **set of all languages** over $`\Sigma`$ is a **powerset** $`2^{\Sigma^{*}}`$;

<br>

## Theorem: $`2^{S}`$ is **uncountable**
Let $`S`$ is an **infinite** *countable set*. The its **powerset** $`2^{S}`$ is **not** *countable*.<br>
Let $`S = \{s_{1}, s_{2}, ... , s_{i}, ... \}`$. Then **any element** $`t`$ of $`2^{S}`$ is a **set of some elements** from $`S`$, for example: $`t_{j} = \{s_{4}, s_{100}\}`$.<br>
Any element $`t`$ of $`2^{S}`$ can be represented by sequence of $`0`$ and $`1`$ in which:
- $`1`$ is in position $`i`$ if $`s_{i}`$ is in the $`t`$;
- $`0`$ is in position $`i`$ if $`s_{i}`$ is **not** in the $`t`$;

For example, $`t_{j} = \{s_{2}, s_{3}\}`$ can be represented by: $`\{0, 1, 1, 0, 0, ... \}`$.<br>

Suppose that $`2^{S}`$ were countable, then its elements could be written in some order, say: $`t_{1}`$, $`t_{2}`$, ... , $`t_{i}`$, ... . We could writen then in column as follows:

|t||||||
|:-|:-|:-|:-|:-|:-|
|$`t_{1}`$|**1**|0|0|0|...|
|$`t_{2}`$|1|**1**|0|0|...|
|$`t_{3}`$|1|1|**0**|0|...|
|$`t_{3}`$|1|1|0|**0**|...||

Consider elements in the main diagonal $`{1, 1, 0, 0, ... }`$.
If we **inverse** elements in main diagonal we will get new sequense of $`0`$ and $`1`$ which represents some element $`t_{w}`$ in the $`2^{S}`$.<br>

But $`t_{w}`$ cannot be $`t_{1}`$, because it differs from $`t_{1}`$ in the first index, for the same reason it cannot be $`t_{2}`$, $`t_{3}`$ and so on.<br>

We got **contradiction**. So, $`2^{S}`$ is **uncountable**.<br>

<br>

## Turing machines vs. Languages
We can show that there are **fewer** *Turing machines* (TM) **than** *languages*. In other words, there are exists languages, that **cannot** be accepted by TM.<br>

A language $`L`$ over $`\Sigma`$ is said to be **recursively enumerable** if there exists a TM $`M`$ that **accepts** $`L`$.<br>

<br>

### Theorem: *non-recursively enumerable* languages exist
For any **non-empty** $`\Sigma`$ there are exist languages that are **not** **recursively enumerable**.<br>
Since $`\Sigma^{*}`$ is a infinite countable set, then $`2^{\Sigma^{*}}`$ is **uncountable**.<br>
But the set of all TM is **countable**.<br>

<br>

# Grammar
A **grammar** is something that **somehow describes a language**.<br>
A **grammar** is a **finite** *description* of **infinite** *set of sentences*.<br>

Can all languages be described by finite grammars? In fact there is nothing wrong with getting a single **infinite** set from a single **finite** description.<br>
For example, the **"set of all positive integers"** is a **finite** *description* of a an **infinite** *set*.<br>

<br>

## Theorem: number of languages is **uncountable**
*Descriptions of languages* **can be** *enumerated*. But *languages* **cannot be** *enumerated*.<br>
Consider **language** $`\Sigma^{*}`$. **All** its **subsets** are **languages**.<br>
We can describe every *subset* (or *language*) in $`2^{\Sigma^{*}}`$ as $`L_{i} = 0110...`$, where $`0`$ means sentence from $`\Sigma^{*}`$ is not included in language $`L_{i} `$.<br>

So, we get:
|L||||||
|:-|:-|:-|:-|:-|:-|
|$`L_{1}`$|**1**|0|0|0|...|
|$`L_{2}`$|1|**1**|0|0|...|
|$`L_{3}`$|1|1|**0**|0|...|
|$`L_{3}`$|1|1|0|**0**|...||

where $`L_{i}`$ is a **description of language** which is **enumerated by** $`i`$.<br>

We see that **number of languages is uncountable**: we have listed all possible descriptions, but there are **at least one language** that **has no description** in the list.<br>

## Formal grammars
**Tokens** are called **terminals** or **terminal symbols**.<br>
The intermediate symbols are called **non-terminals** or **grammar variables** or **syntactic categories** in linguistic context.<br>

A **grammar** is a tuple: $`G = (V, T, S, P)`$, where
- $`V`$ is a **finite set** of **non-terminals** or **variables**;
- $`T`$ is a **finite set** of **terminals**;
- $`S \in V`$ is a **start variable** (must be non-terminal);
- $`P`$ is a **finite set of productions** (aka **production rules** or just **rules**);

<br>

**Properties**:
- sets $`V`$ and $`T`$ are **not empty**;
- sets $`V`$ and $`T`$ are **disjoint**: $`V \cap T = \emptyset`$;

<br>

The **production rules** have form:
- $`x \rightarrow y`$, where $`x \in (V  \cup T)^{+}`$ and $`y \in (V  \cup T)^{*}`$
  - $`x`$ is a **left-hand** side of rule and it must consist of sequences of **one** or **more** *terminals* or *non-terminals*;
    - or more formal: $`x \in (V  \cup T)^{+}`$
  - $`y`$ is a **right-hand** side of rule and it must consist of sequences of **zero** or **more** *terminals* or *non-terminals*;
    - or more formal: $`y \in (V  \cup T)^{*}`$