# Contents
<!-- TOC -->
* [Contents](#contents)
* [UML](#uml)
* [UML Class diagram](#uml-class-diagram)
  * [UML Class notation](#uml-class-notation)
    * [The graphical representation of the UML Class diagram](#the-graphical-representation-of-the-uml-class-diagram)
    * [Class `without` signature](#class-without-signature)
    * [Class `with` signature](#class-with-signature)
    * [Visibility of class attributes and operations](#visibility-of-class-attributes-and-operations)
  * [Perspectives of the UML Class diagram](#perspectives-of-the-uml-class-diagram)
    * [Conceptual](#conceptual)
    * [Specification](#specification)
    * [Implementation](#implementation)
* [Types of relationship](#types-of-relationship)
  * [Dependency](#dependency)
  * [Association](#association)
  * [Aggregation](#aggregation)
    * [Example: relationship between `Car` and `Wheels` is Aggregation](#example-relationship-between-car-and-wheels-is-aggregation)
  * [Composition](#composition)
    * [Example: relationship between `Company` and its `Departments` is Composition](#example-relationship-between-company-and-its-departments-is-composition)
  * [Dependency](#dependency-1)
  * [Inheritance (aka Generalization)](#inheritance-aka-generalization)
  * [Realization](#realization)
<!-- TOC -->

<br>

# UML
The **Unified Modeling Language** (**UML**) is a general-purpose **visual modeling language** that is intended to provide a **standard way to visualize the design of a system**.<br>

**UML provides a standard notation** for many types of diagrams which are divided into **2 main groups**:
- **structure diagrams** (represent the **static** aspect of the system):
  - **class diagram**;
  - component diagram;
  - etc
- **behavior diagrams** (represent the **dynamic** aspects of the system):
  - interaction diagrams:
    - **sequence diagrams** (aka **sequence flow diagrams**) emphasize the **flow of control**;
  - etc

<br>

**Control flow** (or **flow of control**) is the **order** of processing of something.<br>

<br>

# UML Class diagram
The **UML Class diagram** describes the structure of a system by showing the system's:
- **classes**;
- their **attributes**;
- **operations** (aka methods);
- **relationships** between classes;

<br>

## UML Class notation
A **class** encapsulates **state** (*attributes*) and **behavior** (*operations*).

- Each **attribute** has a **type**.
- Each **operation** has a **signature**.

<br>

The **UML Class notation** consists of 3 sections:
1. **Class name**.
2. **Class attributes**.
3. **Class methods**.

<br>

The **class name** is the only **mandatory** information.

<br>

### The graphical representation of the UML Class diagram
![UML Class Layout1](/img/uml_class_notation.jpeg)

<br>

### Class `without` signature
![UML Class Layout2](/img/uml_class_notation_1.jpeg)

<br>

### Class `with` signature
![UML Class Layout2](/img/uml_class_notation_2.jpeg)

<br>

### Visibility of class attributes and operations
The `+`, `-` and `#` symbols **before** an **attribute** and **operation** name in a class **denote the visibility** of the attribute and operation:
- `+` denotes **public** attributes or operations;
- `-` denotes **private** attributes or operations;
- `#` denotes **protected** attributes or operations;

<br>

## Perspectives of the UML Class diagram
There are some **levels of detail** (**perspectives**) of UML Class diagram:
1. **Conceptual**: describes only **names** os classes.
2. **Specification**: describes only **interfaces** of classes.
3. **Implementation**: describes **how classes implement their interfaces**.

<br>

### Conceptual
![Conceptual](/img/uml_class_concept.jpeg)

<br>

### Specification
![Specification](/img/uml_class_spec.jpeg)

<br>

### Implementation
![Implementation](/img/uml_class_impl.jpeg)

<br>

# Types of relationship
**Types of relationship** between entities:
1. **Dependency**.
2. **Association**.
3. **Composition**.
4. **Aggregation**.
5. **Inheritance** (aka **Generalization**).
6. **Realization** (aka **Implementation**).

<br>

![UML Class Relationship](/img/uml_class_relations.jpeg)

<br>

## Dependency
A **dependency** is a very general relationship. A **dependency** exists between two entities if changes in one entity may cause changes to the other.<br>
The _dependency_ is represented by a **dashed line with an open arrow** **from** _dependent_ **to** its _dependency_.<br>
**Cardinality** and **modality** normally **doesn’t** make sense on a _dependency_.<br>

A **dependency** implies that an object somehow **uses** another object:
- if type **implements** _interface_ it **depends** on interface;
- if type **has** or somehow **uses** _another class_ it **depends** on it;
- if function/method **receives** or **returns** object of some type it **depends** on it;
- if type **inherits** _another type_ it **depends** on it;

<br>

## Association
**Association** describes the relationships between some entities.<br>
An **association** implies that **two entities** are **connected** but concrete **semantics** is **not specified**.<br>
The **association** is represented by a **solid line** between objects, it my have directions.<br>

<br>

For instance, an **association** can be used to model following cases in programming:
- one instance of some type holds a **reference** to value of another type in one of its **attribute**/**field**;
- one instance of some type holds **value** of another type in one of its **attribute**/**field**;
- one type has **method**, that accepts **argument** of some other type;

<br>

There are two forms of **association** with additional semantics: **composition** and **aggregation**.<br>
Both _aggregation_ and _composition_ imply **has-a** or **part-of** relationship.<br>

<br>

An **association** may have **cardinality** and **modality**.<br>

**Cardinality** and **modality** at the **opposite side** depict number of instances:
- `1..*` one or more;
- `*` zero or more (also `0..*`);
- `4` exactly **4** (also `4..4`);
- `0..1` zero or one;

<br>

![UML Class Cardinality](/img/uml_composition.jpeg)

Here `1..*` means that instance of **Company** can have **one or more** instances of **Department**.<br>

<br>

## Aggregation
**Aggregation** is a special type of *association* used to model a **has-a** or **part-of** relationship.<br>
Instance that **contains** another instance is called **parent** entity (**whole**, **container**).<br>
Instance that **is contained in** another instance is called **child** entity (**part of**, **nested**).<br>
The **aggregation** is displayed by **solid line** from the **parent** to the **child** which has **unfilled diamond** on the side of **parent**.<br>

<br>

In **aggregation**, both _parents_ and _children_ have **separate lifetimes**, in other words, they are **independent**.<br>
- **parent** can *continue to exist* after the **child** is *destroyed*;
- **child** can *continue to exist* after the **parent** is *destroyed*;
- **child can** be *created* before its **parent** is *created*;

<br>

### Example: relationship between `Car` and `Wheels` is Aggregation
![UML Class Cardinality](/img/uml_aggregation.jpeg)

<br>

The **Car** is a **whole entity** and car's **Wheel** is a **part of** the overall **Car**.<br>
The lifecycle of the instance of **Wheel** is **independent** of the instance of **Car**.<br>

<br>

## Composition
**Composition** is a special type of *association* used to model a **has-a** or **part-of** relationship.<br>
Instance that **contains** another instance is called **parent** entity (**whole**, **container**).<br>
Instance that **is contained in** another instance is called **child** entity (**part of**, **nested**).<br>
The **composition** is displayed by **solid line** from the **parent** to the **child** which has **filled diamond** on the side of **parent**.<br>

In **composition**, lifecycle of **children** is **connected** to lifecycle of **parent**:
- if **parent** is _destroyed_ then **all** its **children** are _destroyed_ too;
- **child can't** be *created* before its **parent** is *created*;

<br>

### Example: relationship between `Company` and its `Departments` is Composition
![UML Class Cardinality](/img/uml_composition.jpeg)

<br>

The **Company** is a **parent** and company's **Department** is a **child**.<br>
The **Department** **cannot** exist before a **Company** exists.<br>
When the **Company** instance is destroyed, the **Department** instance is **automatically destroyed** as well.<br>

<br>

## Dependency

<br>

## Inheritance (aka Generalization)
**Inheritance** (aka **Generalization**) is used to model a **parent-childs** relationship:
- **from** _child_ **to** _parent_ - **generalization**;
- **from** _parent_ **to** _child_ - **specialization**;

<br>

The **inheritance** is represented by a **solid line with a hollow arrowhead** that points *from* the **child** *to* the **parent**.<br>

<br>

## Realization
**Realization** is used to model a **interface implementations** relationship.<br>
When type **implements** some **interface** there is **realization** relations between **type** and **interface**.<br>
The **realization** is displayed by a **dashed line with a hollow arrowhead** that points *from* the **type** *to* the **interface**.<br>
