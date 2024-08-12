# Data structures
A **data structure** is a data organization and storage format that is usually chosen for efficient access to data.<br>
**Data structures** serve as the basis for **abstract data types** (ADT). The ADT defines the logical form of the data type. The data structure implements the physical form of the data type.<br>

<br>

# Data model
A **data model** is an abstract model that organizes elements of data and standardizes how they relate to one another and to the properties of realworld entities.<br>
The corresponding professional activity is called generally **data modeling** or, more specifically, **database design**.<br>
In the context of programming languages a **data model** explicitly determines the **structure of data**.<br>

<br>

Kinds of data models:
- **conceptual** data model;
- **logical** data model;
- **physical** data model;

An **entity–relationship diagram** (**ERD**) can be used to represent all kinds of data models.

<br>

# Database model
A **database model** is a specification describing how a database is structured and used. The most popular example of a *database model* is the **relational model**, which uses a table-based format.

<br>

**Common db models** include:
- hierarchical model;
- network model;
- relational model;
- document model

<br>

The significance of this approach is that it allows the **three perspectives** to be relatively **independent** of each other.<br>
**Storage technology** can change **without** affecting either the **logical** or the conceptual model.<br>
The table/column structure can change **without** affecting the **conceptual** model.<br>
In each case, of course, the structures must remain consistent with the other model.<br>

A data model can sometimes be referred to as a data structure, especially in the context of programming languages.<br>

A **database model** is a type of data model that determines the logical structure of a database.<br>
The **database schema** is the structure of a database described in a formal language.<br>

A **database model** is a type of data model that determines the **logical structure of a database**.<br>
It fundamentally **determines** in which manner data can be **stored**, **organized** and **manipulated**. The most popular example of a database model is the **relational model**, which uses a table-based format.<br>

A database model is a specification describing how a database is structured and used.<br>

A **schema** describes the **structure of the data**, primarily for databases and other tools so they can use and store the data effectively.<br>

**Schema management done well**, makes working with data easy and safe for engineers - **without tight coupling between services or teams**.<br>

<br>

## Examples
### Pydantic
1. **Pydantic models** can be used to define table schemas and validate incoming data.
2. **Pydantic models** can emit JSON Schema, allowing for easy integration with other tools.
3. One of the primary ways of defining **schema** in Pydantic is via **models**.

<br>

# Schema evolution and compatibility
When you start modifying schemas, you need to take into account questions like: Who do we upgrade first—consumers or producers? Can new consumers handle the old events that are still stored in Kafka? Do we need to wait before we upgrade consumers? Can old consumers handle events written by new producers?<br>

Schema evolution in Avro, Protocol Buffers and Thrift.<br>

Thrift, Protobuf and Avro all support schema evolution: you can change the schema, you can have producers and consumers with different versions of the schema at the same time, and it all continues to work. That is an extremely valuable feature when you’re dealing with a big production system, because it allows you to update different components of the system independently, at different times, without worrying about compatibility.<br>

<br>

## Types of compatibility
Different **types of compatibility**:
- **Backward compatibility** refers to the **ability** of a system, software, or device to **work with older versions**.
- **Forward compatibility** is the **ability** of a system or software to **work with future versions**.

<br>

# Schemas are not contracts
It is common to hear the phrase "we do **contract first development**", but what they really mean is that they **define a schema** for their API. **Schemas** are useful, but they are **not contracts**.<br>

**Schema** - using **declarative notation**, defines the **data types** for inputs and outputs that a single system supports at a point in time.<br>
**Contract** defines how two systems are able to communicate by agreeing on what interactions can be sent between them and providing concrete examples to test the agreed behaviour.<br>

Schemas may take many shapes - **protobufs**, **GraphQL**, **Avro**, **Open API Specification** (OAS, previously Swagger) the most common in API development, etc.<br>

Here is how JSON Schema defines a schema:<br>
You may have noticed that the **JSON Schema** itself is written in JSON. It is **data itself**, **not** a computer program.<br>
It’s just a **declarative format** for **describing** the **structure** of other **data**.<br>
However, since a JSON Schema **can’t** contain arbitrary **code**, there are **certain constraints** on the relationships between data elements that **can’t be expressed**.<br>
Any **validation tool** for a complex data format will likely have two phases of validation:
- one at the **structural level** and **can** be *validated* by schema;
- one at the **semantic level** and **cannot** be *validated* by schema, it need to be implemented using a more general-purpose programming language;

<br>

**Schemas** are **abstract**, **contracts** are **concrete**.<br>