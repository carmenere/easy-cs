# Morphism
In *mathematics*, particularly in *category theory*, a **morphism** is a **structure-preserving map** between two **mathematical** structures of **the same type**.<br>

Examples:
- in **set theory**, morphisms are **functions**;
- in **group theory**, morphisms are **homomorphisms**;

<br>

# Homomorphisms
## In algebra
In algebra, a **homomorphism** is a **structure-preserving map** between two **algebraic** structures of the same type (for example, **two groups**, **two rings**, **two vector spaces**).<br>

Consider 2 groups: <G,*> and <H,·>.
A **group homomorphism** is a **function** `f: G → H` such that `∀ x,y ∈ G, f(x * y) = f(x)·f(y)`.<br>

![Group homomorphism](/img/group-homomorphism.png)

The **kernel** of a group homomorphism `f: G → H` consists of all elements in the `G` that are mapped to the **neutral element** in the  `H`. The **kernel** **forms** a **subgroup** of `G`.<br>

A **group isomorphism** is a *group homomorphism* which is a **bijection**, in other words it **can be reversed** by an **inverse mapping**.<br>

<br>

## In category theory
A **functor** is a **homomorphism of categories**.<br>

A functor `F` from a category `C` to a category `D` is a **map** sending each object `x ∈ C` to an object `F(x) ∈ D` and each **morphism** `f:x→y` in `C` to morphism `F(f): F(x) → F(y)` in `D` such that `F` **preserves** **composition** and `F` **preserves** **identity morphisms**.

![Functor](/img/functor.jpg)

<br>

# Special homomorphisms
1. **Epimorphism** is a **surjective** *homomorphism*.
2. **Monomorphism** is an **injective** *homomorphism*.
3. **Isomorphism** is a **bijective** *homomorphism*.
4. **Endomorphism** is a *homomorphism* from **some structure** to **itself**.
5. **Automorphism** is a **bijective endomorphism**, in other words it's **isomorphism** from **some structure** to **itself**.

<br>

So,
1. **Homomorphism** is generalization of **isomorphism**.
2. **Isomorphism** is generalization of **equality**. In mathematical jargon, one says that two **objects are the same up to an isomorphism**.

<br>

Two mathematical structures are called **isomorphic** if an **isomorphism** **exists** between them.
