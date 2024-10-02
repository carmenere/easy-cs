# Token. Lexeme. Pattern
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
|**Identifier**||[A-Za-z0-9]+ A sequence of characters and numbers **initiated** by a character.|
|**Comparison**|||

<br>

# Lexical analyzer. Parser. Semantic analyzer
The **lexical analyzer** *reads* a **stream of characters**, identifies the **lexemes** in the stream, and **categorizes** them into **tokens**. This is called **tokenizing**. If the lexer finds an **invalid** token, it will report an **error**.<br>

Then **parser** (aka **syntax analyzer**, **syntactic analyzer**) reads tokens and checks that they all satisfy **rules** of a formal **grammar**.<br>

The **semantic analyzer** performs **additional checks** *after parser* which are impossible to describe in the EBNF and thus not easily detected during parsing.<br>