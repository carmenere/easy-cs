# Variance
## Variance in math
Consider following operations:
|Operation|Notation|Example|
|:--------|:-------|:------|
|**Double**|`D(x)`|`x + x`|
|**Negation**|`N(x)`|`-1 * x`|
|**Square**|`S(x)`|`x * x`|

<br>

Will some operation `F(x)` over some integer value `x` **keep order** relation between its input `x` and output `F(x)`, i.e., if `x <= y`, then `F(x) <= F(y)` or `F(x) >= F(y)`?<br>
**Variance** answers this question.<br>
**Variance** reflects how the relationship between elements will change after the operation.<br>

There are **3 kinds** of **variance**:
|Kind|Description|Example of operation|
|:---|:----------|:-------------------|
|**Covariance**|The operation `F` **keeps** order relation between its input and output, i.e., if `x <= y` then `F(x) <= F(y)`|**Double**: `2 < 3` => `4 < 6`|
|**Contravariance**|The operation `F` **reverses** order relation between its input and output, i.e., if `x <= y` then `F(x) >= F(y)`|**Negation**: `2 < 3` => `-2 > -3`|
|**Invariance**|The operation `F` **reverses** order relation for one values and **keeps** order relation for another values.|**Square**: `2 < 3` => `4 < 9`; `-3 < -2` => `9 > 4`|

<br>

## Variance in programming languages
If `S` is **subtype** (**more specific**) of `T` then `T` is **supertype** (**more general**) of type `S`.<br>
The fact that `S` is a **subtype** of `T` (or `T` is a **supertype** of type `S`) is denoted as `S <: T` or `S => T`.<br>

**Subtyping** is the idea that one type (called **subtype**) can be used in place of another type.<br>

Questions:
- can we **safely** assign variable of **more specific** type `S` to a variable of **more general** type `T`?
- can we **safely** assign variable of **more general** type `T` to a variable of **more specific** type `S`?

<br>

**Variance** answers these questions.<br>
**Variance** defines **rules** for **casting** between types. Rules for casting in turn define **assignment compatibility** (aka **substitutability**) between types.<br>

<br>

There are 2 types of casting:
- **Upcasting** is casting **subtype** to its **supertype**. **Upcasting** is also known as **Liskov substitution**.
- **Downcasting** is casting **supertype** to its **subtype**. **Downcasting** is **danger** and forbid in general.

<br>

Consider following relations between types `S => T`:
- **Covariance** means we can use `S` instead `T`, i.e., it is possible to use `S` where an `T` is expected. **Covariance** means **upcasting**.
- **Contravariance** means we can use `T` instead `S`, i.e., it is possible to use `T` where an `S` is expected. **Contravariance** means **downcasting**.
- **Invariance** means we can **not** use `T` instead `S` and we can **not** use `S` instead `T`.

<br>

Examples of **covariance**: 
- **Inheritance** in OOP languages is **covariant**. **Inheritance** defines an relationship between types (aka **type hierarchy**): **parent** type is **supertype**, **child** type is **subtype**.
  - For example, if `Cat => Animal` then it is possible to upcast `Cat` to `Animal`, i.e., it is possible to use `Cat` where an `Animal` is expected. 
- Variable of type `i32` can be safely assigned to variable of type `i64`, but **not vice versa**, variable of type `i64` cannot be safely assigned to variable of type `i32`. `S: i32 => T: i64`.

<br>

# Bypassing type control system
Consider following code:
```rust
fn replace_with_dog(a: &mut Animal) {
    let d: Dog = ...;
    *a = d;
}

fn main() {
    let mut cat: Cat = ...;
    replace_with_dog(&mut cat);  // after f() cat will be replaced with dog!
    cat.meow();                  // meow() on dog!
}
```

<br>

In general, the **covariant** rule is **not safe** for **mutable types** because it allows **bypass type control system**.<br>

So, there are some rules for type constructors:
- **Read-only** types must be **covariant**;
- **Write-only** types must be **contravariant**.
- **Mutable** types must be **invariant**.

<br>

# Variance of collections
Let the `Cat` and `Dog` types inherit from the `Animal` type.<br>
Let create **collection of animals**: `let animals = Vec<Animal>`.<br>
Let create **collection of cats**: `let cats = Vec<Cat>`.<br>

<br>

### Collection is covariant
Consider `Vec<T>` **covariant** collection: if `Cat => Animal` then `Vec<Cat> => Vec<Animal>`, i.e., it is possible to use `Vec<Cat>` where `Vec<Animal>` is expected.<br>
We can pass `Vec<Cat>` to function `unsafe_animals()` that accepts `Vec<Animal>`. Thus, in such function `Vec<Cat>` is treated as `Vec<Animal>` and it is possible to add object of type `Dog` to `Vec<Cat>`:
```rust
fn unsafe_animals(p: Vec<Animal>) {
    p.add(Dog::new());
}
```

<br>

In general, the **covariant** rule is **not safe** for **mutable collections** because it allows **bypass type control system**.<br>

> **Arrays in Java** are always covariant: if `Cat => Pet`, then `Cat[] => Pet[]`.<br>
There is **no** compile time checks for arrays in Java: compiler allows to pass `Cat[]` to `Pet[]` and then treat `Cat[]` as `Pet[]`, for instance, insert `Dog` to `Cat[]`.<br>
An `ArrayStoreException` is thrown **at runtime** instead.


<br>

### Collection is contravariant
Consider `Vec<T>` **contravariant** collection: if `Cat => Animal` then `Vec<Animal> => Vec<Cat>`. , i.e., it is possible to use `Vec<Animal>` where `Vec<Cat>` is expected.<br>
We can pass `Vec<Animal>` to function `unsafe_cats()` that accepts `Vec<Cat>`. Thus, in such function `Vec<Animal>` is treated as `Vec<Cat>` and it is possible to get object of type `Dog` from `Vec<Animal>`:
```rust
fn unsafe_cats(p: Vec<Cat>) {
    let cat = p.get();  // can return Dog, but out variable cat expect object of Cat!
}
```

<br>

In general, the **contravariant** rule is **not safe** for **immutable  collections** because it allows **bypass type control system**.

<br>

### Collection is invariant
Consider `Vec<T>` **invariant** collection: `Cat => Animal`. It is **not** possible to use `Vec<Cat>` where `Vec<Animal>` is expected and it is not possible to use `Vec<Animal>` where `Vec<Cat>` is expected.<br>

The **invariant** rule is **safe** for **any** **collections**.

<br>

# Variance of generics
In many languages, generics are **invariant** by default.

<br>

# Variance of function type
Some definitions:
- **Argument** is a **value** that is passed to some function, e.g., variable `a` of some type `A`.
- **Receiving variable** is a **variable** that accepts a returned from some function value, e.g., variable `rv` of some type `RV`.
- **Parameter** is a **variable** where function put accepted **argument**, e.g., parameter `p` of type `P`.
- **Returned value** is a **value** that function returns, e.g., value of type `R`.
- **Function type** of some function `foo` with signate `foo(p: P) -> r: R` is denoted as: `P -> R`, where `P` is type of **parameter**, `R` is type of **returned value**.

<br>

```rust
let a: A = 10;       // argument a of type A
...
foo: P -> R;         // function foo has parameter p of type P and returns value of type R
...
let rv: RV = foo(a); // here argument a is assigned to paraeter p; then returned value of type R is assigned to variable rv
```

<br>

**Function type** is `contravariant` in the **parameter type** `P` and `covariant` in the **return type** `R`.<br>
**Substitutability rule** for **function type**: it is **safe** to substitute a function `f` with a function `g` if `f` accepts a **more general** type and returns a **more specific** type than `g`.<br>
In other words,
- it is allowed to **extend** type of **parameter** when replacing function;
- it is allowed to **narrow** type of **returned value** when replacing function.

<br>

Explanation:
- replacing function **doesn’t** change type of **argument**, e.g., variable `a` of some type `A`;
- replacing function **doesn’t** change type of **receiving variable**, e.g., variable `rv` of some type `RV`;
- replacing function **changes only** *function type*, i.e., replacing function changes **type** of **paramater** `P` and **type** of **returning value** `R`.

<br>

If `P2 <: P1` and `R1 <: R2`, then we can replace `P2 -> R2` with `P1 -> R1`, i.e., *function type* `P1 -> R1` is **subtype** of `P2 -> R2`.<br>

<br>

### Example
`Cat -> Animal` can be replaced with:
- `Animal -> Cat`
- `Cat -> Cat`
- `Animal -> Animal`

<br>

## Explanation contravariant in the parameter type
Function `Cat -> Animal` has 1 parameter of type `Cat`. So, the argument that passed to this function is of type `Cat`.<br>
It is possible, to **upcast** variable of type `Cat` to variable of type `Animal`, because inheritance is **covariant**.<br>
So, we can replace function with parameter of type `Cat` on another function with parameter of type `Animal` and then **upcast** argument of type `Cat` to `Animal`.<br>

<br>

## Explanation covariant in the return type
Function `Cat -> Animal` returns type `Animal`. So, the variable to which the value of this function is assigned is of type `Animal`.<br>
It is possible, to **upcast** returning value of type `Cat` to variable of type `Animal`, because inheritance is **covariant**.<br>
So, we can change function with return type `Animal` on another function with return type `Cat` and then **upcast** returned value of type `Cat` to `Animal`.<br>

<br>




