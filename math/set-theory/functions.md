# Table of contents
- [Table of contents](#table-of-contents)
- [Function (aka mapping)](#function-aka-mapping)
  - [Self-map (aka transformation)](#self-map-aka-transformation)
    - [Identity map](#identity-map)
  - [Domain. Image. Codomain](#domain-image-codomain)
    - [Example](#example)
- [Composition](#composition)
- [Left inverse. Right inverse. Invertible](#left-inverse-right-inverse-invertible)
- [Surjection](#surjection)
- [Injection](#injection)
- [Bijection](#bijection)
- [Surjection/Injection/Bijection on finite sets](#surjectioninjectionbijection-on-finite-sets)

<br>

# Function (aka mapping)
The terms **function** and **mapping** are **synonymous**.<br>

**Defenitions**:
1. **Informal definition**: when we write $`f: X \longrightarrow Y`$ or say $`f`$ is a **function from** $`X`$ **to** $`Y`$ we mean that $`f`$ is a **definite rule** which associates to **each** element $`x`$ in $`X`$ an **exactly one** element $`f(x)`$ in $`Y`$.
2. **Formal set-theoretic definition of function**: a **function** $`f`$ **from** a set $`X`$ **to** a set $`Y`$ is a **subset** $`f ⊆ X \times Y`$ with the property that for each $`∀x ∈X, ∃!y∈Y: ⟨x,y⟩ ∈ f`$. The *notation* $`∃!y∈Y:`$ means "**exists exactly one** $`y∈Y`$ **such that ...**".
3. **Defenition of function** in terms of **input** and **output**: when we give **function** $`f`$ an **input** $`x∈X`$, it gives an **output** $`f(x)`$. The $`x`$ in $`f(x)`$ is called the **argument** of $`f`$, and $`f(x)`$ is called the **value of function**.

<br>

So, **function** $`f`$ **maps** $`x ∈ X`$ **to** $`y=f(x) ∈ Y`$.<br>

<br>

**Synonyms**:
|Input|Relationship|Output|
|:----|:-----------|:-----|
|**Pre-image**|*Maps to*|**Image**|
|**Argument**|*Maps to*|**Value of function**|
|**Independent variable**|*Maps to*|**Dependent variable**|

<br>

Notations:<br>
1. $`f: X \longrightarrow Y`$, $`f`$ is the **function**, $`X`$ its **domain**, $`Y`$ its **codomain**.
2. $`y = f(x)`$, $`y∈Y`$, $`x∈X`$.
3. $`x ↦ y`$ **instead** of $`y=f(x)`$, this allows the definition of a function **without naming** it.

<br>

## Self-map (aka transformation)
A **map** (aka **function**) is a **mapping** of a **set** $`X`$ to **any** set, i.e. $`f: X \longrightarrow Y`$.<br>
A **self-map** (aka **transformation**, **transform**) is a **mapping** of a **set** $`X`$ to **itself**, i.e. $`f: X \longrightarrow X`$.<br>

<br>

### Identity map

One important **self-map** is the **identity map** (aka **identity function**, **identity relation**, **identity transformation**).<br>

An **identity function** is a **self-map** such that $`f(x) = x`$ for **all** elements $`x`$ in $`X`$.
In other words, the output value $`f(x)`$ in the *codomain* $`X`$ is always the same as the **input** element $`x`$ in the *domain* $`X`$.

The **identity function** $`f`$ on set $`X`$ is often denoted by $`\text{id}_{X}`$.<br>

<br>

## Domain. Image. Codomain
Consider some function $`f: X \longrightarrow Y`$:
- The set $`X`$ is called the **domain** of $`f`$, written $`dom \space f`$, it is the **set of values** that go **into** a function.
- The set $`Y`$ is called the **codomain** of $`f`$, it is the **set of values** that could **possibly come out** of a function.
- The $`im \space f`$ of $`f`$, written $`im \space f`$, it is the **set of values** that **actually comes out** of a function, more formally: $`im \space f = {f(x): x ∈ X \space and \space f(x) ∈ Y}`$.

So, the **image** $`im \space f`$ is a **subset** of the *codomain* $`Y`$: $`im \space f ⊆ Y`$.<br>
The **image** $`im \space f`$ has **pre-images**. But **complement** of **image** $`Y \ im \space f`$ **hasn't pre-images**.<br>

Why both? Well, sometimes we don't know the exact $`im \space f`$, but we know the set it **lies in**. So we define the **codomain** and continue on.<br>

<br>

### Example
Consider function $`f = x^2`$ and $`dom \space f = {1,2,3}`$. Here $`im \space f`$ = $`{1,4,9}`$ and **codomain** can be = $`{1,2,3,4,5,6,7,8,9}`$.

<br>

# Composition
Let $`f: X \longrightarrow Y`$ and $`g: Y \longrightarrow Z`$ be functions.<br>
The **composition** of $`g`$ and $`f`$, written $`g∘f`$ is the function $`g∘f: X→Z`$ such that $`(g∘f)(x) = g(f(x))`$. Thus $`g∘f`$ means apply $`f`$ to $`x`$, then apply $`g`$ to $`f(x)`$.<br>

Notice that composition only makes sense when the **codomain** of $`f`$ is the same as the **domain** of $`g`$.

<br>

# Left inverse. Right inverse. Invertible
Consider functions $`f: X \longrightarrow Y`$ and $`g: Y \longrightarrow Z`$:

$`g: Y \longrightarrow X`$ is a **left inverse** of $`f: X \longrightarrow Y`$ if $`g(f(x)) = x`$ for all $`x ∈ X`$.<br>
If you follow the function **from** the **domain** **to** the **codomain**, the **left inverse** tells you **exactly** how to go back to where you started.<br>

$`h: Y \longrightarrow X`$ is a **right inverse** of $`f: X \longrightarrow Y`$ if $`f(h(y)) = y`$ for all $`y ∈ Y`$.<br>
In other words, **right inverse** tells you where you might have come from domain, for any possible destination.<br>

In other words, consider **concrete value** $`y_{1}`$ in the **codomain** $`Y`$. Then answer the question what **exact** value of argument was used to get this **concrete value** $`y_{1}`$?
- $`f(x_{1}) \longrightarrow y_{1} ?`$
- $`f(x_{2}) \longrightarrow y_{1} ?`$
- $`f(x_{3}) \longrightarrow y_{1} ?`$

So, **right inverse** **doesn't** tell the **original** value of argument, only **possible** value of argument.

<br>

Also,
- If $`g∘f = idₓ`$, then $`g`$ is a **left inverse** to $`f`$;
- If $`f∘h = idᵧ`$, then $`h`$ is a **right inverse** to $`f`$;

<br>

If $`f`$ has a **left inverse** and a **right inverse**, it is **invertible**.<br>
If $`f`$ has a **left inverse** $`g`$ and a **right inverse** $`h`$ they are both equal: $`h = g`$, so we write them as $`f⁻¹`$.<br>
If $`f`$ has a **left inverse** and a **right inverse**, it is **invertible** and it has *one and only one* **inverse function**, written $`f⁻¹`$.<br>

<br>

# Surjection
Formal defenition: $`f: X \longrightarrow Y`$ is **surjective** if $`∀y∈Y, ∃x∈X : y = f(x)`$ for every $`y ∈ Y`$ there is some $`x ∈ X`$ such that $`f(x) = y`$.<br>
In other words, the function is **surjective** if **every element** in the **codomain** has **at least one pre-image**.<br>

A **surjective function** is also called a **surjection**.<br>

So, for **surjection**:
- the **codomain** is **equal** to the **image** $`Y = im \space f`$;
- element in the **codomain** can have **more then one pre-image**;

<br>

# Injection
Formal defenition: $`f: X \longrightarrow Y`$ is **injective** if $`∀ x, x' ∈ X, f(x) = f(x') => x = x'`$.<br>
In other words, the function is **injective** (aka **one-to-one**) if:
- **every** element in the **domain** has **exactly one image** in the **codomain**;
- **every** element in the **image** has **exactly one pre-image** in **domain**;
- **elements** in **image complement** $`Y \ im \space f`$ **don't** have **pre-images** in **domain**;

<br>

An **injective function** is also called an **injection**.<br>

<br>

# Bijection
The function is **bijective** if it is both **injective** and **surjective**.<br>
Formal defenition: $`f: X \longrightarrow Y`$ is **surjective** if $`∀y∈Y, ∃!x∈X : y = f(x)`$ and $`∀x∈X, ∃!y∈Y : y = f(x)`$.<br>
In other words, the function is **bijective** if each element in its **codomain** is mapped to **exactly one** element in the **domain**.<br>

<br>

A **bijective function** is also called a **bijection**.<br>

<br>

# Surjection/Injection/Bijection on finite sets
- If $`f: X \longrightarrow Y`$ is **injective** then $`|X| ≤ |Y|`$;
- If $`f: X \longrightarrow Y`$ is **surjective** then $`|X| ≥ |Y|`$;
- If $`f: X \longrightarrow Y`$ is **bijective** then $`|X| = |Y|`$;