# Dependency
Whenever *component* `A` **uses** (**invoke**) another *component* `B`, then it is said that `A` **depends on** `B`.<br>
*Component* `A` is called **dependent** and *component* `B` is called **dependency**.

<br>

# Coupling
**Coupling** refers to the **degree** of **interdependence** between *сomponents*.<br>
**Coupling** can be **low** (aka **loose**, **weak**) or **high** (aka **tight**, **strong**).<br>
**Decoupling** means decreasing interdependence between software components.<br>

**High coupling** means that *components* are **closely connected** and **changes** in one *component* **affects** other *components*.<br>
**Low coupling** means that **changes** in one *component* **have impact** on other *components*.<br>

<br>

Consider you have two **coupled** types: `A` and `B`:
- if `A` and `B` are **tightly coupled** then we **have to** rewrite the `A` if we want to replace `B` with another one;
- if `A` and `B` are **loosely coupled** then we **can** replace `B` with another one **without** having to change `A`;

<br>

**Advantages** of **loose coupling**:
1. Greater **flexibility**.
2. Greater **extensibility**.
3. Greater **scalability**.

<br>

**Disadvantages** of **tight coupling**:
1. A change in one module usually forces a **ripple effect** of changes in other modules.
2. It has the **lack of** *flexibility* and *scalability*.

<br>

# Inversion of Control
A **library** is a **set of functions** that **are called** by **caller** (aka **client**), then each such **function does** some **work** and **returns control** to **caller**.<br>
When a *library* is used then **developer-written** code **calls** reusable **libraries' functions** and **controls** the **flow of control** of a **program**.<br>

<br>

**But** **frameworks invert** the **flow of control** of a **program**.<br>
When a **framework** is used then **developer-written** code **is called** *from within* a **framework itself**, rather than from an **app**. Developper just calls framework's "main function", which will handle all the execution and call the programmer's code when needed. As a result, there is a **loss of control over code execution**.<br>

Such **invertion** of the **flow of control** of a program is called **Inversion of Control** (aka **IoC**).<br>
**IoC** is a **key part** of what **makes** a *framework* **different** to a *library*.<br>
So, with **IoC**, it is the **framework calls developer-written** code and **controls** the **flow of control** of a **program**.<br>

For example, **event-driven programming** is often implemented using **IoC** so that the **custom code** need **only** be concerned with the **handling of events**, while the **event loop** and **dispatch of events**/**messages** is **handled by the framework**. In web server application frameworks, **dispatch** is usually called **routing**, and **handlers** may be called **endpoints**.

**IoC** is a **design principle** and it is **too generic a term**.<br>
**IoC container** is a something that implements **IoC**, for instance, some **framework**.<br>

So, the question is: **What aspect of control does IoC invert**?

If we focus on component’s **dependency lifecycle** management, then there **two types of IoC**:
1. **Dependency Injection** (aka **DI**).
2. **Service Locator**.

<br>


# Dependency Injection (aka DI)
**Dependency Injection** (aka **DI**) is a **design pattern** and one of implementation **IoC** which is a **design principle**.<br>

**DI** allows **decouple** components by **injecting dependencies** into a **dependent component** from the **outside**, instead of creating **dependencies** directly inside **dependent component**. Thus the **control** of object creation is **inverted**, i.e. it is **delegated** to another component called **DI container**.<br>

<br>

**DI** can be implemented through **constructors**, **setters** or **fields**.<br>

So, there are 3 types of DI:
1. **Constructor injection**. *Dependency* is passed throgh parameter of **constructor method**.
2. **Field injection**/**Property injection**. *Dependency* is assigned to appropriate attribute of **initialized instance** directly. But this way of injection also comes with **drawback**: **mutability**.
3. **Method injection**/**Setter Injection**.  *Dependency* is passed throgh parameter of special **setter** method.

<br>

# Service locator
**Service locator** is a **design pattern** and one of implementation **IoC** which is a **design principle**.<br>
**Service locator** is like **registry** that **returns dependencies by request**.
