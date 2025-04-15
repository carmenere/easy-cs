<!-- TOC -->
* [The Chomsky hierarchy](#the-chomsky-hierarchy)
  * [Lexical analyzer. Syntax analyzer (Parser). Semantic analyzer](#lexical-analyzer-syntax-analyzer-parser-semantic-analyzer)
  * [Token. Lexeme. Pattern](#token-lexeme-pattern)
  * [Examples](#examples)
* [Arch of compiler](#arch-of-compiler)
* [IR](#ir)
  * [Advantages of IR](#advantages-of-ir)
    * [Case 1: no IR](#case-1-no-ir)
    * [Case 2: with IR](#case-2-with-ir)
* [LLVM](#llvm)
* [Optimizations](#optimizations)
<!-- TOC -->

<br>

# The Chomsky hierarchy
The **Chomsky hierarchy** defines **4** classes of formal grammars:
- **regular grammars** - can be processed with **lexical analysis**;
- **context-free grammars** - can be processed with **syntactic analysis**;
- **context-sensitive grammars** - can **not** be processed with **syntactic analysis**;
- **unrestricted grammar** (aka recursively enumerable);

<br>

## Lexical analyzer. Syntax analyzer (Parser). Semantic analyzer
**Most programming languages** are related to **context-sensitive grammars** and cannot be processed with just **syntactic analysis**.<br>
There is common approach to overcome this limitation: separate **syntax** of language (its _context-free part_) from its **semantic** (its _context-sensitive part_).<br>

The **lexical analyzer** *reads* a **stream of characters**, identifies the **lexemes** in the stream, and **categorizes** them into **tokens**. This is called **tokenizing**. If the lexer finds an **invalid** token, it will report an **error**.<br>

Then **parser** (aka **syntax analyzer**, **syntactic analyzer**) reads tokens, checks that they all satisfy **rules** of a formal **grammar** and finally build **AST**.<br>

An **AST** is a data structure that represents source code. An **AST** (**abstract syntax tree**) is usually the result of the **syntax analysis** phase of a compiler.<br>
The **semantic analyzer** performs **additional checks** which are impossible to describe in the EBNF and thus not easily detected during parsing.<br>
At the **semantic analysis** phase **semantic analyzer** processes **AST** and performs some actions and finally generates **IR**.<br>

<br>

## Token. Lexeme. Pattern
A **pattern** is a **rule** (for example, a *regular expression*) that is used to check if a **sequence of characters** is valid or not.<br>

**Lexeme** is a **sequence of characters** in the text that is **matched** by the **pattern** for a *token*. A **lexeme** is an **instance** of a *token*.<br>

A **token** is the smallest unit of meaningful data. A **token** represents a **sequence of characters** that **cannot** be decomposed further. A *token* can have an *optional* **token value**.<br>

<br>

## Examples
|**Token**|**Lexeme**|**Pattern**|
|:----|:-----|:------|
|**Keyword**|`while`|Characters `w` `h` `i` `l` `e`|
|**Integer**|`7`|Sequence of digits with at least one digit `[0-9]+`|
|**String**|`"Hi"`|Characters enclosed by `"` and `"` except `"`|
|**Punctuation**|`,`|`;` `,` `.` `!` etc.|
|**Identifier**|`foo1`|[A-Za-z0-9]+ A sequence of characters and numbers **initiated** by a character.|

<br>

# Arch of compiler
![compiler-stages](/img/compiler-stages.png)

<br>

Three parts of compiler:<br>
**Frontend**: transforms **source code** into **AST**, then **AST** to **IR**.<br>
**Middle-end** (aka **optimizer**): performs analysis and optimizations and transforms **IR** into better **IR**.<br>
**Backend**: converts **IR** into **target machine code**.<br>

<br>

# IR
An **intermediate representation** (**IR**) is the data structure or code used internally by a compiler or virtual machine to represent source code.
An IR is designed to be conducive to further processing, such as optimization and translation.
An IR must:
- represent the source code **without loss** of information; 
- do **not** depend on **source lang**;
- do **not** depend on **target lang**;

<br>

Some compilers can use **multiple IRs**: each IR is suited for particular optimizations.<br>
For example: **High-level IR** (**HIR**), **Mid-level IRs** (**MIRs**), **Low-level IRs** (**MIRs**).<br>

<br>

## Advantages of IR
Suppose we wish to build compilers for $`N`$ **source** languages and $`M`$ **targets**.<br>

<br>

### Case 1: no IR
We need $`N*M`$ compilers to support $`N`$ **source** languages and $`M`$ **targets**.<br>

![no-ir](/img/no-ir.png)

<br>

### Case 2: with IR
We need only $`N`$ **frontends** and $`M`$ **backends**.<br>

![with-ir](/img/with-ir.png)

<br>

# LLVM
It can be used to develop a frontend for any programming language and a backend for any instruction set architecture.<br>

<br>

**LLVM** is an umbrella name for a number of software components that can be used to build compilers:
- **LLVM frontends**:
  - transform **source code** into **LLVM IR**:
    - Clang (C)
    - Clang++ (C++)
    - Rust
- **LLVM Pass**:
  - LLVM applies a **chain of analyses** and **transformations** on the target program, each of these analyses or transformations is called a **pass**;
  - **LLVM Passes** are performed by **middle-end** (aka **optimizer**) which receives **LLVM IR** as input and generate new optimized **LLVM IR** as output;
- **LLVM backend** is a **target-independent** **machine code generator**;

<br>

LLVM has its own instruction set **LLVM IR**. Some characteristics of **LLVM IR**:
- RISC-like instruction set;
- strongly typed;
- explicit control flow;
- uses a virtual register set with infinite temporaries (%);
- uses **Static Single Assignment** (**SSA**) form;

<br>

# Optimizations
**Constant folding** is the process of recognizing and evaluating constant expressions at compile time rather than computing them at runtime.<br>
i = 320 * 200 * 32;
Most compilers would not actually generate two multiply instructions and a store for this statement. Instead, they substitute the computed values at compile time (in this case, 2,048,000).

<br>

**Constant propagation** is the process of substituting the values of known constants in expressions at compile time.
Consider the following pseudocode:

```shell
int x = 14;
int y = 7 - x / 2;
return y * (28 / x + 2);
```

Propagating $`x`$ yields:
```shell
int x = 14;
int y = 7 - 14 / 2;
return y * (28 / 14 + 2);
```

Then, propagating $`y`$ yields:
```shell
int x = 14;
int y = 0;
return 0;
```

<br>

**Dead-code elimination** is a compiler optimization to remove **dead code** (code that does not affect the program results).<br>
Removing such code has several benefits:
- it shrinks program size;
- it reduces CPU and RAM usage;
