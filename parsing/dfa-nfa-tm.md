# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [DFA/NFA](#dfanfa)
- [TM](#tm)
  - [Recursive languages](#recursive-languages)
  - [Recursively enumerable languages](#recursively-enumerable-languages)
- [Undecidability](#undecidability)
  - [Turing machines vs. Languages](#turing-machines-vs-languages)
    - [Theorem: *non-recursively enumerable* languages exist](#theorem-non-recursively-enumerable-languages-exist)
  - [Undecidable languages (non RE languages)](#undecidable-languages-non-re-languages)
<!-- TOC -->

<br>

# DFA/NFA
An **automaton** (/ɔːˈtɒm.ə.tən/) is an abstract model of computation. An *automaton* whose output **response is limited** to a simple **yes** or **no** is called **acceptor**.<br>
An *acceptor* either **accepts** *input string* or **reject** it.<br>
A **transducer** is a more general *automaton* that can produce strings as output.<br>

<br>

A **deterministic finite automaton** (aka **deterministic finite acceptor**) (**DFA**) is a **finite-state machine** that **accepts** or **rejects** a given **string of symbols**, by running through a state sequence uniquely determined by the string.<br>

<br>

Formal definition of DFA: a DFM $`M`$ is a **5-tuple** $`M = (Q, \Sigma, \delta, q_{0}, F)`$, where
- $`Q`$ is a *finite set* of **internal states**;
- $`\Sigma`$ is a *finite set* of  symbols called **input alphabet**;
- $`\delta: Q \times \Sigma \rightarrow Q`$ is a **transition function**;
- $`q_{0} \in Q`$ is the **initial state**
- $`F \subseteq Q`$ is a *finite set* of **final states**;

<br>

**Transition function** maps every **pair** composed of **current state** $`q_{i}`$ and **input symbol** $a_{k}$ to some **new state** $`q_{j}`$: $`\delta(q_{i}, a_{k}) = q_{j}`$.<br>

For example, if DFA is in **current state** $`q_{0}`$ and **current input symbol** is $`a`$, then DFA will **go into state** $`q_{1}`$: $`\delta(q_{0}, a) = q_{1}`$.<br>

<br>

**Deterministic** means that for every pair $`(q_{i}, a_{l})`$ DFA has **exactly one next state** (**one transition**).<br>

**Nondeterministic Finite Automaton** (**NFA**) allow **zero**, **one**, or **multiple transitions**. In other words there are may exist more than one next states for pairs $`(q_{i}, a_{k})`$.<br>

**DFAs** are **faster** at processing input because they have a **single**, **unique path** for any input string.<br>

<br>

To visualize and represent finite automata we use **transition graphs**, in which the **vertices** represent **states** and the **edges** represent **transitions**, and **labels** on edges represent **input symbols** that transit FA to new allowed state. For example, **edge** $`(q_{0}, q_{1})`$ with **lable** $`a`$ represents the **transition** $`\delta(q_{0}, a) = q_{1}`$.<br>

<br>

The *set of final states* $`F`$ is denoted graphically by a **double circle**.<br>
The *initial state* is denoted graphically by an **arrow coming in from nowhere**.<br>

<br>

Example:
![dfa](/img/dfa.png)

<br>

**Extended transition function** $`\delta^{*}: Q \times \Sigma^{*} \rightarrow Q`$. The second argument $`\Sigma^{*}`$ of $`\delta^{*}: Q \times \Sigma^{*} \rightarrow Q`$ is a **string**, rather than a single symbol and transition gives the state the *automaton* will be in after reading such **string**.<br>

For example, if:
- $`\delta(q_{0}, a) = q_{1}`$
- $`\delta(q_{1}, b) = q_{2}`$
then
- $`\delta^{*}(q_{0}, ab) = q_{2}`$

<br>

A DFA operates in following manner. At the **initial time**, it is in the **initial state** $`q_{0}`$ with its *input mechanism* on the **left most symbol** of the **input string**.<br>
During **each step** the *input mechanism* **advances one position** to the right, so each step consumes **one input symbol**.<br>
When the end of input string is reached, the string is **accepted** if the *automaton* is in one of its **final state**. Otherwise the string is **rejected**.<br>
The *input mechanism* can move only **from left to right** and read **exactly one symbol** on each step.<br>

The **language** is the **set of all strings** accepted by the *automaton*.<br>

The language $`L`$ **accepted** by DFA $`M = (Q, \Sigma, \delta, q_{0}, F)`$ is denoted as $`L(M)`$.<br>
More formal: $`L(M) = \{w \subset \Sigma^{*}: \delta^{*}(q_{0}, w) \in F \}`$.<br>

**Nonacceptance** means that the DFA stops in a nonfinal state so that: $`\overline{L(M)} = \{ w \subset \Sigma^{*}: \delta^{*}(q_{0}, w) \not \in F \}`$.<br>

<br>

# TM
A **Turing machine** (**TM**) has **2 final states**:
- $`q_{accept} \in Q`$: is the **accept state**
- $`q_{reject} \in Q`$: is the **reject state**
- $`q_{accept} \ne q_{reject}`$

<br>

Let $`M`$ be a Turing machine and let $`w`$ be a **string**, then:
- $`M`$ **halts on** $`w`$ if it **stops** (**terminates**) in some **final state**;
- $`M`$ **accepts** $`w`$ if it **halts in** an **accept state** when run on $`w`$;
- $`M`$ **rejects** $`w`$ if it **halts in** a **reject state** when run on $`w`$;
- $`M`$ **loops infinitely** on $`w`$ (or just **loops** on $`w`$), in other words **never halts**;
- $`M`$ **doesn't accept** $`w`$ if it either **rejects** $`w`$ or **loops** on $`w`$.
- $`M`$ **doesn't reject** $`w`$ $`w`$ if it either **accepts** $`w`$ or **loops** on $`w`$;

<br>

## Recursive languages
Some TMs **always halt**, i.e. they **never** *go into an infinite loop*.<br>
A **decider** is a *Turing machine* such that any time you give it a **string** (**word**) $`w`$ from $`\Sigma^{*}`$ as input, it will **always halt**, either **accepting** or **rejecting** the string.<br>

Let $`M`$ be a Turing machine, then $`M`$ is called a **decider** for some language $`L`$ over $`\Sigma`$ if:
- for any $`w \in L`$ it **accepts** $`w`$;
- for any $`w \not\in L`$ (but $`w \in \Sigma^{*}`$) it **rejects** $`w`$;

<br>

So, for *deciders*, **accepting** is the same as **not rejecting** and **rejecting** is the same as **not accepting**.<br>

A language $`L`$ is called **recursive** (aka **R**, **decidable**) if there **exists** a **decider** for such language $`L`$.<br>

<br>

## Recursively enumerable languages
A **recognizer** (aka **semi-decider**) is a *Turing machine* such that
any time you give it a **string** (**word**) $`w`$ from $`\Sigma^{*}`$ as input, it will **halts only on strings from** $`L`$ ($`w \in L`$), but if string **is not from** $`L`$ ($`w \not\in L`$) it can either **reject** or **loop**.<br>

Let $`M`$ be a Turing machine, then $`M`$ is called a **recognizer** (aka **semi-decider**) for some language $`L`$ over $`\Sigma`$ if:
- for any $`w \in L`$ it **accepts** $`w`$;
- for any $`w \not\in L`$ (but $`w \in \Sigma^{*}`$) it **doesn't accept** $`w`$: **rejects** $`w`$, or **loops** on $`w`$;

The class  is the **set of all recognizable language**s. $`L`$ ∈ RE ↔ L is recognizable 

A language $`L`$ is called **recursively enumerable** (aka **RE**, **Turing-recognizable**, **recognisable**, **semi-decidable**) if there **exists** a **recognizer** for such language $`L`$.<br>

<br>

# Undecidability
## Turing machines vs. Languages
We can show that there are **fewer** *Turing machines* (*TM*) **than** *languages*. In other words, there are **exists languages**, that **cannot be accepted** by TM.<br>

<br>

### Theorem: *non-recursively enumerable* languages exist
For any **non-empty** $`\Sigma`$ there are exist languages that are **not recursively enumerable**.<br>
Since $`\Sigma^{*}`$ is a infinite countable set, then $`2^{\Sigma^{*}}`$ is **uncountable**.<br>
But the set of all TM is **countable**.<br>

<br>

## Undecidable languages (non RE languages)
A language $`L`$ is called **undecidable** (**non-recursively enumerable**, **non RE**) if there **doesn't exist** TM that **can even accept** $`L`$.<br>