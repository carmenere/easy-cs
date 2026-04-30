# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Parsing](#parsing)
- [Language](#language)
  - [Theorem: $`2^{S}`$ is **uncountable**](#theorem-2s-is-uncountable)
- [Grammar](#grammar)
  - [Theorem: number of languages is **uncountable**](#theorem-number-of-languages-is-uncountable)
  - [Formal grammars](#formal-grammars)
- [Derivations](#derivations)
- [The Chomsky hierarchy of grammars and languages](#the-chomsky-hierarchy-of-grammars-and-languages)
  - [Type 0 grammar](#type-0-grammar)
  - [Type 1 grammar](#type-1-grammar)
    - [Example of CS grammar](#example-of-cs-grammar)
  - [Type 2 grammar](#type-2-grammar)
    - [Recursion](#recursion)
    - [BNF](#bnf)
    - [Extended CF grammar](#extended-cf-grammar)
      - [Example](#example)
  - [Type 3 grammar](#type-3-grammar)
    - [`lex`](#lex)
  - [Type 4 grammar](#type-4-grammar)
- [Languages and grammars](#languages-and-grammars)
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

There is **no limit** on *size of sentence*. So, a languange can be possibly of **infinite** *set of sentences* that can be made by combining letters from the **finite** *alphabet* $`\Sigma`$.<br>

Consider alphabet $`\Sigma = \{a, b\}`$. The language $`\Sigma^{*}`$ is an **infinite set** of all possible sentences that can be made from $`\Sigma`$.<br>
The **\*** means that language contains **empty word** denoted by $`\varepsilon`$ which consists of zero symbols.<br>

So, $`\Sigma^{*} = \{\varepsilon, a, b, aa, bb, ab, ba, aaa, ... \}`$.<br>

The language $`\Sigma^{+}`$ means language **without** *empty word*: $`\Sigma^{+} = \Sigma^{*} - \varepsilon`$.<br>

The language $`\Sigma^{*}`$ has **interesting property** that **all languages using alphabet** $`\Sigma`$ **are subset of** $`\Sigma^{*}`$.<br>
In other words, the language $`\Sigma^{*}`$ contains **any possible sentence** over $`\Sigma`$.<br>

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

# Grammar
A **grammar** is something that **somehow describes a language**.<br>
A **grammar** is a **finite** *description* of **infinite** *set of sentences*.<br>

Can all languages be described by finite grammars? In fact there is nothing wrong with getting a single **infinite** set from a single **finite** description.<br>
For example, the **"set of all positive integers"** is a **finite** *description* of a an **infinite** *set*.<br>

<br>

## Theorem: number of languages is **uncountable**
*Descriptions of languages* **can be** *enumerated*. But *languages* **cannot be** *enumerated*.<br>
Consider **language** $`\Sigma^{*}`$. **All** its **subsets** are **languages**.<br>
We can describe every *subset* (or *language*) in $`2^{\Sigma^{*}}`$ as $`L_{i} = 0110...`$, where $`0`$ means sentence from $`\Sigma^{*}`$ that is not included in language $`L_{i} `$.<br>

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
*Sentences* are consist of **words** called **tokens**.<br>
**Tokens** are called **terminals** or **terminal symbols**.<br>
The **intermediate symbols** are called **non-terminals** or **grammar variables** or **syntactic categories** in linguistic context.<br>

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
- $`x \rightarrow y`$
- where:
  - $`x`$ is a **left-hand** side of rule and it must consist of sequences of **one** or **more** *terminals* or *non-terminals*;
    - or more formal: $`x \in (V  \cup T)^{+}`$
  - $`y`$ is a **right-hand** side of rule and it must consist of sequences of **zero** or **more** *terminals* or *non-terminals*;
    - or more formal: $`y \in (V  \cup T)^{*}`$

<br>

# Derivations
Given a string $`w`$ of the form $`w=uxv`$. If we apply **production** $`x \rightarrow y`$ to this string $`x`$ will be **replaced** with $`y`$ and we will get **new** string $`z`$: $`z=uyv`$.<br>
We say that $`w`$ **derives** $`z`$ or that $`z`$ is **derived from** $`w`$. This is written as $`w \rArr z`$.<br>

A production can be used whenever it is applicable and it can be applied as many times as desired.<br>

If $`w_{1} \rArr w_{2} \rArr ... \rArr w_{n}`$ we say that $`w_{1}`$ derives $`w_{n}`$ and write $`w_{1} \stackrel{*}\rArr w_{n}`$. The **\*** indicates that **unspecified number of steps** can be taken to derive $`w_{n}`$ from $`w_{1}`$.<br>

A **given grammar** can generate many different strings by applying the production rules in a different order. The set of all such terminal strings is the language defined by grammar.<br>

Let $`G=(V,T,S,P)`$ is a grammar. Then $`L(G)=\{w \in T^{*}: S \stackrel{*}\rArr w\}`$ is the language $`L`$ generated by $`G`$.<br>

If the **sentence** (**terminal**) $`w`$ belongs to language $`L`$: $`w \in L(G)`$, then the sequence $`S \rArr w_{1} \rArr w_{2} \rArr ... \rArr w_{n} \rArr w`$ is a derivation of $`w`$. The strings $`S, w_{i}`$ which contain variables as well as terminals are called **sentential forms** of the derivation.<br>

Consider the grammar $`G=(\{S\}, \{a,b\}, S, P)`$, where $`P`$ contains 2 rules:
- $`S \rightarrow aSb`$
- $`S \rightarrow \varepsilon`$

Then $`S \rArr aSb \rArr aaSbb \rArr aabb`$, so we can write $`S \stackrel{*}\rArr aabb`$.<br>
The **string** $`aabb`$ is a **sentence** (**terminal**) in the language generated by $`G`$.<br>
The **string** $`aaSbb`$ is a **sentential form** in the language generated by $`G`$.<br>

<br>

# The Chomsky hierarchy of grammars and languages
## Type 0 grammar
A grammar $`G`$ is a **type 0 grammar** (aka **unrestricted grammar**, **phrase structure grammar**, **PS grammar**) if its all production rules are of form:
$`x \rightarrow y`$, where
  - $`x \in (V \cup T)^{+}`$
  - $`y \in (V \cup T)^{*}`$

<br>

There is only **one restriction**: $`\epsilon`$ (**empty sentence**) is **not allowed** in the **left side** of production.<br>

<br>

In other words, a *type 0 grammar* contains rules that transform an arbitrary numbers of symbols into an arbitrary number (possibly zero) of symbols.<br>

<br>

## Type 1 grammar
There are 2 equalent definition of **type 1 grammar**:
1. A grammar $`G`$ is a **type 1 monotonic** if it contains **no rules** in which **left-hand side** (**lhs**) contains **more** symbols than the **right-hand side** (**rhs**):
   - $`lhs \rArr rhs`$ where $`|lhs| \le |rhs|`$;
   - **examples**:
     - $`ab \rightarrow cde`$ ✅
     - $`ab \rightarrow cd`$ ✅
     - $`abc \rightarrow de`$ ❌
2. A grammar $`G`$ is a **type 1 context-sensetive** (**CS grammar**, **CSG**) if **all its rules** are **context-sensetive**. A rule is a **context-sensetive** it **replaces only one non-terminal** in its **left-hand side** (**lhs**):
   - $`uvw \rightarrow avw`$, where
     - $`u`$, $`v`$ and $`w`$ **3 non-terminals**, but only **one** of them $`u`$ was **replaced** by rule;
     - $`a`$ can be **terminal** or **non-terminal**;

<br>

### Example of CS grammar
Consider **production rules**:
- $`S \rightarrow abc | aSQ`$
- $`bQc \rightarrow abc | bbcc`$
- $`cQ \rightarrow Qc`$

**Derivations**:
- apply the first rule 3 times: $`S \rArr aSQ \rArr aaSQQ \rArr aaabcQQ`$;
- then rule 3: $`aaabcQQ \rArr aaabQcQ`$
- then rule 2: $`aaabQcQ \rArr aaabbccQ`$
- then rule 3 twice: $`aaabbccQ \rArr aaabbcQc \rArr aaabbQcc`$
- then rule 2: $`aaabbQcc \rArr aaabbbccc`$

So, this grammar generates **language** that consist of all **words** that all consist of **equal number of symbols** $`a`$, $`b`$ and $`c`$. It is called $`a^{n}b^{n}c^{n}`$ language.<br>

<br>

## Type 2 grammar
A grammar $`G`$ is a **type 2 grammar** (aka **context-free grammar**, **CF grammar**, **CFG**) if all its productions have the form:
- $`A \rightarrow x`$, where
  - $`A \in V`$
  - $`x \in (V \cup T)^{*}`$

In other words **CF grammar** can contain only rules that have a **single non-terminal** on their **left-hand side** (**lhs**).<br>

Since there is **only one symbol** on the lhs **all rhs sides** for a given non-terminal **can** always **be collected** in **one grammar rule**.<br>

<br>

### Recursion
If **non-terminal** $`A`$ produces string that **again contains** $`A`$ such **non-terminal** is called **recursive**.<br>
The **non-terminal** $`A`$ can be **directly recursive** or **indirectly recursive**:
- consider 1 production rule:
  - $`A \rightarrow Az`$;
  - here $`A`$ is **directly recursive**;

- consider 2 production rules:
  - $`A \rightarrow Bc`$;
  - $`B \rightarrow dA`$;
  - here $`A`$ is **indirectly recursive**;

<br>

The **non-terminal** $`A`$ can be **righ-recursive**, **left-recursive** or **self-embedding**:
- the **non-terminal** $`A`$ is **righ-recursive** if it can produce something that has $`A`$ at the **right end**:
  - $`A \rightarrow zA`$
- the **non-terminal** $`A`$ is **left-recursive** if it can produce something that has $`A`$ at the **left end**:
  - $`A \rightarrow Az`$
- the **non-terminal** $`A`$ is **self-embedding** if it can produce something that has $`A`$ at the **middle**:
  - $`A \rightarrow \alpha A \beta`$

<br>

So, the **basic property** of **CFG** is that they **describe things** that can be **nested**.<br>

<br>

Some authors and some parsing algorithms require a **CF grammar** to be **monotonic** or $`\epsilon`$ **free grammar**.<br>

The grammar is not $`\epsilon`$ **free** if it has **productions with empty righ-hand side** (such rules are called $`\epsilon`$ rules), examples:
- $`B \rightarrow \epsilon`$
- $`C \rightarrow \epsilon`$

<br>

### BNF
The **BNF** is a **notation** for **CFG**.<br>
In **BNF** the **angle brackets** are used to enclose **non-terminals**.<br>

Example:
```text
<name> ::= Tom | d
<list> ::= <name>
```

<br>

### Extended CF grammar
Consider rules:
- $`S \rightarrow a | b | c`$
- $`Sequence\_of\_S \rightarrow S | Sequence\_of\_S`$

In **extended CFG** we **don't need** to give a rule for $`Sequence\_of\_S`$, instead we can give only **one rule** for $`S`$ and then use $`S^{+}`$. The $`S^{+}`$ is implicitly means rule $`S^{+} \rightarrow S | S^{+}`$.<br>

<br>

An **extended CFG** provides **regular expressions**:
- $`+`$ means **1** or **more**;
- $`n+`$ means **1** or **max n**;
- $`*`$ means **0** or **more**;
- $`n*`$ means **0** or **max n**;
- $`?`$ means **0** or **1**;
- $`(...)`$ **parentheses** treat its content as whole;

<br>

Examples:
- $`S \rightarrow A^{+} \; B \; C^{+}`$
- $`X \rightarrow C^{*}`$
- $`W \rightarrow D^{?}`$
- $`X \rightarrow (A;)^{?}`$

<br>

#### Example
Consider **production rule**: $`Book \rightarrow \; Preface \; Chapter^{+} Conclusion`$.<br>

The above production rule is equal to **2 rules**:
- $`Book \rightarrow Preface \; \alpha \; Conclusion`$
- $`\alpha \rightarrow Chapter \; | \; \alpha \; Chapter`$

<br>

These rules derives **many sentences** but they all have some **common structure**:
![cf_grammar](/img/cf_grammar_example.png)

<br>

## Type 3 grammar
A grammar $`G`$ is a **right-linear** or **righ-regular** if all productions have form:
- $`A \rightarrow zB`$
- $`A \rightarrow z`$
- where
  - $`A \in V`$
  - $`B \in V`$
  - $`z \in T^{*}`$

<br>

A grammar $`G`$ is a **left-linear** or **left-regular** if all productions have form:
- $`A \rightarrow Bz`$
- $`A \rightarrow z`$
- where
  - $`A \in V`$
  - $`B \in V`$
  - $`z \in T^{*}`$

<br>

A grammar $`G`$ is a **type 3 grammar** (aka **regular grammar**, **RE grammar**, **finite-state grammar**, **FS grammar**) if it is either **left-regular** or **righ-regular**.<br>

<br>

In other words, in the *regular grammar* **one non-terminal can produce**:
- **one terminal**;
- **one terminal** followed by **one non-terminal** or **one non-terminal** followed by **one terminal**;

<br>

In *regular grammars* **at most** *one non-terminal* can appear on the **right-hand side** (**rhs**) of any production. Furthermore, **that non-terminal** must consistently be either the **rightmost** or **leftmost** symbol of the **right-hand side** (**rhs**) of any production.<br>

<br>

So, *regular grammars* **restrict position of non-terminal** in productions.<br>

A **linear grammar** is a grammar in which **at most** *one non-terminal* (*variable*) can occur on the rhs of any production **without restriction** on the position of this *variable*.<br>

Consider grammar $`G = (\{S,A,B\}, \{a,b\}, S, P)`$ with productions:
- $`S \rightarrow A`$
- $`A \rightarrow aB | \epsilon`$
- $`B \rightarrow Ab`$

It is **not** *regular*, but it is **linear**.<br>

<br>

### `lex`
The `lex` util is a **parser generator** for **regular grammars**.<br>

<br>

## Type 4 grammar
A grammar $`G`$ is a **type 4 grammar** (aka **finite-choice grammar**, **FC grammar**, **FCG**) if all its productions **cannot** have variables on their **rhs**.<br>
In other words, **non-terminals** are **not allowed** on the **right-hand side** (**rhs**) of any production.<br> 

For example, the **set of keywords** in a programming language can be described through **FC grammar**.<br>

<br>

# Languages and grammars
A language is identified with the **highest class** of grammar, which can generate it.<br>
If language $`L`$ **can** be generated by *type 0 grammar* and *type 1 grammar*, but **cannot** be generated by *type 2 grammar*, it is called **type 1 language**.<br>
