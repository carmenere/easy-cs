# Aspects of type system
1. **Type checking**: `static` typing/`dynamic` typing.
2. **Type conversions**: `strong` typing/`weak` typing.
3. **Declarations**: `explicit` typing/`implicit` typing.

<br>

# Static typing vs. Dynamic typing
**Type checking** is the process of verifying and enforcing the constraints of types.<br>
**Type checking** may occur **at compile time** (aka **static check**) or **at runtime** (aka **dynamic check**):
- a language is **statically typed** if the type of a variable is known **at compile time**, type of variable **cannot** be changed later.
- a language is **dynamically typed** if the type of a variable is not known **at compile time**, but known **at runtime**, type of variable **can** be changed later.

<br>

# Strong typing vs. Weak typing
**Type conversion** (type **casting**) can be **implicit** or **explicit**.<br>
In most languages, the word **coercion** is used to denote an **implicit** *type conversion*.<br>

**Strong typing** means **coercions** are forbidden in language.<br>
**Weak typing** means **coercions** are allowed in language.<br>

<br>

# Explicit typing vs. Implicit typing
**Explicit typing** means variables must be **explicitly declared** before use.<br>
**Implicit typing** means compiler/interpreter can **infer** type for variables.<br>

<br>

# Examples
|Lang|Type checking|Type conversion|Declarations|
|:---|:------------|:--------------|:---------------|
|JavaScript|Dynamic|Weak|Implicit|
|Python|Dynamic|Strong|Implicit|
|C|Static|Weak|Explicit|
|Java|Static|Strong|Explicit|
