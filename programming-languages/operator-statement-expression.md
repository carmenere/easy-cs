# Operator
**Operator** is a **symbolic designation** of some action performed on **operands**.<br>
**Operand** is an operator's argument.<br>

<br>

## Operator properties
1. **Associativity**:
   - **left**-associative (meaning the operations are grouped from the left): `x << y << z == (x << y) << z`;
   - **right**-associative (meaning the operations are grouped from the right), example: `x ** y ** z == x ** (y ** z)`;
2. **Precedence**.
3. **Notation**:
   - **prefix** (aka **polish notation**): **operator** goes **before** its **operands**;
   - **infix**: human readable;
   - **postfix** (aka **reverse polish notation**): **operator** goes **after** its **operands**;
4. **Arity** (number of operands)

<br>

### Associativity
**Associativity** determines the **order** in which operators of **the same priority** are applied to operands in the absence of explicit indications of the order, i.e. in the **absence of brackets**.<br>
Many programming language manuals provide a **table of operator** *precedence* and *associativity*.<br>

<br>

### Notations
|Infix Notation|Prefix Notation|Postfix Notation|
|:-------------|:--------------|:---------------|
|`a + b`|`+ a b`|`a b +`|
|`(a + b) * c`|`* + a b c`|`a b + c *`|
|`a / b + c / d`|`+ / a b / c d`|`a b / c d / +`|

<br>

### Function as operator
In most languages, **functions** may be seen as a special form of **prefix operator** with fixed **precedence** and **associativity**, often with compulsory **parentheses**.<br>

<br>

## Operator overloading
**Operator overloading** (**ad hoc polymorphism**) is mechanism of binding operators with methods and different operators have different implementations depending on their arguments.<br>
**Operator overloading** does not change the expressive power of a language (with functions), as it can be emulated using function calls. 

<br>

# Statement
**Statement** is a **syntactic unit** of an imperative programming language that expresses some action.<br>

Statement can be:
- **simple statement** *doesn't* contain another statement;
- **compound statement** includes *more* than one statement;

<br>

## Properties
- statement **doesn't** return **value**;
- statement **can** include **expression**;

<br>

# Expression
**Expressions** always evaluate to a **value** of some type. **Expression** can be used as part of a **statement**.
