# Function
**Informally**, when we write `f: X → Y` or say `f` is a **function** **from** `X` **to** `Y` we mean that `f` is a **definite rule** which associates to **each** element `x` in `X` an **exactly one** element `f(x)` in `Y`. In other words **function** `f` **maps** `x ∈ X` **to** `y=f(x) ∈ Y`.<br>

**Formal set-theoretic definition of function**:<br>
a function `f` **from** a set `X` **to** a set `Y` is a **subset** `f ⊆ X×Y` with the property that for each `∀x∈X, ∃!y∈Y: ⟨x,y⟩ ∈ f`, here `∃!y∈Y:` means `... exists exactly one y∈Y such that ... `.<br>

**Defenition of function** in terms of **input** and **output**: when we give **function** `f` an **input** `x∈X`, it gives an **output** `f(x)`. The `x` in `f(x)` is called the **argument** of `f`, and `f(x)` is called the **value of function**.<br>

<br>

**Synonyms**:
|Input|Relationship|Output|
|:----|:-----------|:-----|
|Pre-image|*Maps to*|Image|
|Argument|*Maps to*|Value of function|
|Independent variable|*Maps to*|Dependent variable|

<br>

Notations:<br>
1. `f: X → Y`, `f` is the **function**, `X` its **domain**, `Y` its **codomain**.
2. `y = f(x)`, `y∈Y`, `x∈X`.
3. `x ↦ y` **instead** of `y=f(x)`, this allows the definition of a function **without naming** it.

<br>

## Domain. Image. Codomain
Consider some function `f: X → Y`:
- The set `X` is called the **domain** of `f`, written `dom f`, it is the **set of values** that go **into** a function.
- The set `Y` is called the **codomain** of `f`, it is the **set of values** that could **possibly come out** of a function.
- The `im f` of `f`, written `im f`, it is the **set of values** that **actually comes out** of a function, more formally: `im f = {f(x): x ∈ X and f(x) ∈ Y}`.

So, the *image* `im f` is a **subset** of the *codomain* `Y`: `im f ⊆ Y`.<br>
*Image* `im f` has **pre-images**. But **complement** of *image* `Y \ im f` **hasn't pre-images**.<br>

Why both? Well, sometimes we don't know the exact `im f`, but we know the set it **lies in**. So we define the **codomain** and continue on.<br>

<br>

### Example
Consider function `f = x^2` and `dom f = {1,2,3}`. Here `im f` = `{1,4,9}` and **codomain** can be = `{1,2,3,4,5,6,7,8,9}`.

<br>

# Composition
Let `f: X → Y` and `g: Y → Z` be functions.
The **composition** of `g` and `f`, written `g∘f` is the function `g∘f: X→Z` such that `(g∘f)(x) = g(f(x))`. Thus `g∘f` means apply `f` to `x`, then apply `g` to `f(x)`.
Notice that composition only makes sense when the **codomain** of `f` is the same as the **domain** of `g`.

<br>

# Identity function
One important function from a set `X` to **itself** is the **identity function**, written `idₓ`.

<br>

# Left inverse. Right inverse. Invertible
Consider functions `f: X → Y` and `g: Y → Z`:

`g: Y → X` is a **left inverse** of `f: X → Y` if `g(f(x)) = x` for all `x ∈ X`.
If you follow the function **from** the **domain** **to** the **codomain**, the **left inverse** tells you **exactly** how to go back to where you started.

`h: Y → X` is a **right inverse** of `f: X → Y` if `f(h(y)) = y` for all `y ∈ Y`.
In other words, **right inverse** tells you where you might have come from domain, for any possible destination.
In other words, consider **concrete value** `y1` in the **codomain** `Y`. Then answer the question what **exact** value of argument was used to get this **concrete value** `y1`?
- `f(x1) -> y1 ?`
- `f(x2) -> y1 ?`
- `f(x3) -> y1 ?`

So, **right inverse** **doesn't** tell the **original** value of argument, only **possible** value of argument.

<br>

Also,
- If `g∘f = idₓ`, then `g` is a **left inverse** to `f`;
- If `f∘h = idᵧ`, then `h` is a **right inverse** to `f`;

<br>

If `f` has a **left inverse** and a **right inverse**, it is **invertible**.<br>
If `f` has a **left inverse** `g` and a **right inverse** `h` they are both equal: `h = g`, so we write them as `f⁻¹`.<br>
If `f` has a **left inverse** and a **right inverse**, it is **invertible** and it has *one and only one* **inverse function**, written `f⁻¹`.<br>

<br>

# Surjection
Formal defenition: `f: X → Y` is **surjective** if `∀y∈Y, ∃x∈X : y = f(x)` for every `y ∈ Y` there is some `x ∈ X` such that `f(x) = y`.<br>
In other words, the function is **surjective** if **every element** in the **codomain** has **at least one pre-image**.<br>
A **surjective function** is also called a **surjection**.<br>

So, for **surjection**:
- the **codomain** is **equal** to the **image** `Y = im f`;
- element in the **codomain** can have **more then one pre-image**;

<br>

# Injection
Formal defenition: `f: X → Y` is **injective** if `∀ x, x' ∈ X, f(x) = f(x') => x = x'`.<br>
In other words, the function is **injective** (aka **one-to-one**) if:
- **every** element in the **domain** has **exactly one image** in the **codomain**;
- **every** element in the **image** has **exactly one pre-image** in **domain**;
- **elements** in **image complement** `Y \ im f` **don't** have **pre-images** in **domain**;

An **injective function** is also called an **injection**.

<br>

# Bijection
The function is **bijective** if it is both **injective** and **surjective**.<br>
Formal defenition: `f: X → Y` is **surjective** if `∀y∈Y, ∃!x∈X : y = f(x)` and `∀x∈X, ∃!y∈Y : y = f(x)`.<br>
In other words, the function is **bijective** if each element in its **codomain** is mapped to **exactly one** element in the **domain**.<br>

A **bijective function** is also called a **bijection**.


<br>

# Surjection/Injection/Bijection on finite sets
If `f: X → Y` is **injective** then `|X| ≤ |Y|`.
If `f: X → Y` is **surjective** then `|X| ≥ |Y|`.
If `f: X → Y` is **bijective** then `|X| = |Y|`.