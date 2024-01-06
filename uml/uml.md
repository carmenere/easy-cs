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

## UML relationships between classes
Types of **relationship**:
1. **Association**.
2. **Composition**.
3. **Aggregation**.
4. **Dependency**.
5. **Inheritance** (aka **Generalization**).
6. **Realization**.

<br>

### The graphical representation of the UML relationships between classes
![UML Class Relationship](/img/uml_class_relations.jpeg)

<br>

### Association
**Association** means some class is **somehow related** to other class.<br>
The **association** is represented by a **solid line** between classes.

<br>

**Cardinality** and **modality** at the **opposite side** depict number of instances of opposite class per **class**:
- `1..*` one or more;
- `*` zero or more (aka `0..*`)
- `4` exactly **4**;
- `0..1` zero or one;

<br>

#### Example
![UML Class Cardinality](/img/uml_composition.jpeg)

Here `1..*` at the opposite side of **Department** means that instance of **Company** can have **one or more** instances of **Department**.

<br>

### Aggregation
**Aggregation** is a special type of *association* used to model a **whole to its parts** relationship.<br>
**Properties**:
- one class is a **whole class** (**container class**) and another is a **part class** (**nested class**);
- **part class** is a *part of* **whole class**;
- **instance of part class** can exist in **multiple instances of whole classes**;
- **instance of part class** and **instances of whole class** have **separate lifetimes**:
  - **instance of part class** can *continue to exist* (outlives its *whole class's* instance) after the **instances of whole class** is *deleted*;
  - **instance of part class** can be *created* before the **instances of whole class** is *created*;

<br>

The **aggregation** is displayed by **solid line** from the **whole class** to the **part class** which has **unfilled diamond** on the side of **whole class**.

<br>

#### Example: relationship between `Car` and `Wheels` is Aggregation
![UML Class Cardinality](/img/uml_aggregation.jpeg)

<br>

The **Car** is a **whole entity** and car's **Wheel** is a **part of** the overall **Car**.<br>
The lifecycle of the instance of **Wheel** is independent from the instance of **Car**.<br>

<br>

### Composition
**Composition** just another form of the *aggregation*.<br>
**Properties**:
- one class is a **whole class** (**container class**) and another is a **part class** (**nested class**);
- **part class** is a *part of* **whole class**;
- **instance of part class** can **only** be **related** to **one whole class**;
- lifecycle of a **part class** is dependent on the **whole class's** lifecycle:
  - **instance of part class** is deleted if the **instances of whole class** is *deleted*;
  - **instance of part class** **can't** be *created* before the **instances of whole class** is *created*;

<br>

The **composition** is displayed by **solid line** from the **whole class** to the **part class** which has **filled diamond** on the side of **whole class**.

<br>

#### Example: relationship between `Company` and its `Departments` is Composition
![UML Class Cardinality](/img/uml_composition.jpeg)

<br>

The **Company** is a **whole class** and company's **Department** is a **part of** the overall **Company**.<br>
The **Department** **cannot** exist before a **Company** exists.<br>
When the **Company** instance is destroyed, the **Department** instance is **automatically destroyed** as well.<br>

<br>

### Dependency
**Dependency** is used to model a **class uses another class** relationship.<br>
**Properties**:
- instance of some class uses instance of another class but used object is **not stored** in any of attributes of using object;
- changes in used class may cause changes to the using class; 

<br>

The **dependency** is displayed by a **dashed line with an open arrow** at the **used** entity side.

<br>

### Inheritance (aka Generalization)
**Inheritance** is used to model a **parent-childs** relationship.<br>


<br>

- From child to parent - **Generalization**.
- From Parent to child - **Specialization**.

<br>

The **inheritance** is represented by a **solid line with a hollow arrowhead** that points *from* the **child** *to* the **parent**.<br>

<br>

### Realization
**Realization** is used to model a **interface implementations** relationship.<br>
When class implemets some interface there is realization relations betwee class and imterface.

<br>

The **dependency** is displayed by a **dashed line with a hollow arrowhead** that points *from* the **class** *to* the **interface**.
