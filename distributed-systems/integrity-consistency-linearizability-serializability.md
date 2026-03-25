# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Integrity vs. Consistency](#integrity-vs-consistency)
- [Coherence vs. Consistency](#coherence-vs-consistency)
- [Consistency models](#consistency-models)
  - [Serializability](#serializability)
  - [Linearizability and its descendants](#linearizability-and-its-descendants)
    - [Linearizability](#linearizability)
      - [Ways to achieve linearizability](#ways-to-achieve-linearizability)
    - [Sequential consistency](#sequential-consistency)
    - [Causal consistency](#causal-consistency)
    - [Eventual consistency](#eventual-consistency)
- [The CAP theorem](#the-cap-theorem)
<!-- TOC -->

<br>

# Integrity vs. Consistency
**Integrity** ensures that the data remains **correct** (**valid**) over its **entire lifecycle**, i.e. **from** *creation* **to** *deletion*.<br>
**Consistency** means that data are **synchronized across all replicas** of a distributed systems. **Consistency** ensures that **all replicas** of a distributed systems have the **same copy** of data, meaning **any read operation** returns the **most recent write**, **regardless** of which node the operation was performed on. In **consistent systems**, every client sees the same, **up-to-date** information, **regardless** of which node/replica responded to the client.<br>

So, data can be:
- **consistent but corrupted**, i.e. **all** replicas have **corrupted data**;
- **correct but inconsistent**, i.e. **one** replica has the **most recent** copy of data against other replicas;

<br>

The relational databases _ensure_ **data consistency** and **data integrity** through **transactions**. **Transaction** is a **group of operations** over **multiple objects**. **Transactions** have the following **properties** (aka **ACID**):
- **C**onsistency: in the context of **ACID** *consistency* means *data integrity*;
  - *consistency* transforms database from one **correct** state to another **correct** state;
  - *consistency* is maintained through **integrity constraints** (e.g. `UNIQUE`, `NOT NULL`, **referential integrity constraints**), **cascades** and **triggers**;
  - **note**, in the context of the **CAP theorem** *consistency* has another sense and means that data are **synchronized across all replicas**;
- **A**tomicity: **all operations** are executed as a **single unit** of work or rolled backed;
- **I**solation: it **doesn't affect other** *transactions*;
- **D**urability: after crash, the system may still contain some changes made by **uncommitted** *transactions* and **the system must be able to restore data consistency after craches**;

<br>

# Coherence vs. Consistency
**Coherence** ensures **all** nodes/processors across the system (distributed or not) see the **most resent version** (i.e. the **most recent write**) of **some data item** (e.g., **variable** `X`).<br>
Coherence typically refers to **cache coherence** and it is often **managed by hardware**. It uses protocols like **MESI** to manage cache line states.<br>

<br>

# Consistency models
A **consistency model** gives **a set of guarantees** that a distributed system **provides** about **results** of concurrent **read**/**write** **operations**. In other words, a **consistency model** is a **contract** between a distributed system and the clients, which specifies when results of concurrent write operations become visible to readers.<br>

The **stricter** the model, the **more guarantees** it provides. One *consistency model* can be considered **stronger** than another if it requires **all constraints** of that *model* and more. In other words, a *model* with **fewer constraints** is considered a **weaker** *consistency model*.<br>

<br>

A **consistency model** is concerned with: **What happens** if a *client* **modifies** some data items and **concurrently** *another client* **reads** or **modifies** the **same data items** *possibly at a* **different replica**?<br>

<br>

[**Consistency models hierarchy**](https://jepsen.io/consistency/models):<br>
![consistency_models](/img/consistency_models.png)

<br>

At the top of hierarchy is the **strong serializability** (aka **strict serializability**) *consistency model*.<br>
**Strong serializability** is the **strongest** *consistency model*, it combines **serializability** and **linearizability**. *Linearizability* is often confused with *serializability* - another consistency model.<br>

<br>

**Transaction** is a **group of operations** over **multiple objects**.<br>

<br>

All *consistency models* **under** *strong serializability* are divided into **2 groups**:
- *consistency models* for *transactions*:
  - **serializability**;
    - this is the the **strongest** *consistency model* for *transactions*;
  - **repeatable read**;
  - **read committed**;
  - **read uncommitted**;
- *consistency models* for *single operations* on *single objects*:
  - **linearizability** (aka **strong consistency**);
    - this is the **strongest single-object** *consistency model*;
  - **sequential consistency**, it is a **weaker** than *linearizability*;
  - **causal consistency**, it is a **weaker** than *sequential consistency*;
  - **eventual consistency**, it is a **weak** *consistency model*;

<br>

## Serializability
**Serializability** is an **isolation property** of transactions and it refers to the **I** or **isolation**, in **ACID**.<br>

*Serializability* **guarantees** that the **final result** of **concurrently** executed transactions is the **same as** if they had executed **serially**, i.e. *sequentially*, *one after another*, **without** *any concurrency*.<br>

Note that this guarantee says **nothing** *about single operations order* (**no real-time constraints** as for *linearizability*). All it assumes is that there **exists some serial execution order** for transactions. So, unlike *linearizability*, *serializability* **allows interleaving of operations** inside transactions.<br>

<br>

Consider **2 concurrent transactions**: `T1` (red) and `T2` (blue):
- `T1` transfers **100$** **from** account `B` **to** account `A`:
  - `A += 100`;
  - `B -= 100`;
- `T2` applies **6%** interest rate to both accounts `A` nad `B`:
  - `A *= 1.06`;
  - `B *= 1.06`;

<br>

*Serial execution order* for transactions `T1` nad `T2`:<br>
![](/img/ser_2.png)

<br>

This execution order **has** a property of *serializability* (i.e. it is **equivalent** to *serial execution*):<br>
![](/img/ser_1.png)

<br>

**But** this execution order of interleaved operations is **not equivalent** to *serial execution*:<br>
![](/img/not_ser_1.png)

<br>

## Linearizability and its descendants
**Consider example**: initially `x=0` and `y=0`, then 2 processes (`P1` and `P2`) perform **read** and **write** *operations* as in the table below:
|Real time clock|`P1`|`P2`|
|:--------------|:-|:-|
|1||`r(y)` **=> ?** |
|2|`r(x)` **=> ?**||
|3|`w(x=5)`||
|4||`r(x)` **=> ?**|
|5|`w(x=9)`||
|6||`w(y=7)`|

<br>

**Logical order** of operations: the **individual sequence of operations per each process**. So, *logical order* is always **per process**:
- *logical order* for `P1`:
  - `r(x)`, `w(x=5)`, `w(x=9)`
- *logical order* for `P2`:
  - `r(y)`, `r(x)`, `w(y=7)`

<br>

### Linearizability
**Linearizability** (aka **strong consistency**, **strict consistency**) provides following **guarantees**:
- **every operation** takes place **atomically**;
- **real-time order guarantee**:
  - **all operations** are ordered *at the global level* according to **real-time ordering**, i.e. **results** of **all** operations are seen to **all** cients according **real-time clock**;
  - i.e., if operation `A` **completes before** operation `B` **starts**, `A` **must appear before** `B` in the **total order**;
- **recency guarantee**:
  - the **result** of **completed** *write* operation must be **immediately visible** to all subsequent operations **regardless of replica** of *distributed system*;
  - i.e., in a *linearizable system* every client always sees the **most recent up-to-date value**;

<br>

**Linearizability** **guarantees** that **all** operations of **all** processes (`P1` and `P2`) appears *at the global level* according to **real-time oredring** (**wall-clock ordering**):<br>
![linearaizable_system_1](/img/linearaizable_system_1.png)

So, in a *linearizable system* **result** of operations is **deterministic**. **Linearizability** makes *distributed system* behave like a **single**, **atomic**, **non-distributed** *system*.<br>

<br>

**Thread-safety** *implies linearizability*.<br>

<br>

The **cost** of *linearizability* is a **high latency** due to quorum-based operations.<br>

<br>

#### Ways to achieve linearizability
Common **approaches** to achieve linearizability:
- **single-leader replication**;
  - **all write operations** go to a single, designated **leader node**;
- **quorum-based linearizability** (e.g., *Paxos*, *Raft*);
  - operations require **agreement from a majority** (**quorum**) of nodes before being considered complete;
    - for a **write**, a **quorum** of nodes must acknowledge the write;
    - for a **read**, a **quorum** of nodes must be queried to ensure the latest data is retrieved;
- **atomic broadcast** (e.g., *Zookeeper’s ZAB*, *Apache Kafka’s Raft*);

<br>

To **ensure** that *read quorum* and *write quorum* **overlap** the inequality **N < R + W** must be satisfied, where
- **N** is a number of **all** nodes in a *distributed system*;
- **R** is a number of nodes in a *read quorum*;
- **W** is a number of nodes in a *write quorum*;

<br>

**Quorum-based linearizability** ensures *linearizability* (*strong consistency*) by requiring that *read quorum* and *write quorum* **must overlap**, because this **intersection** of writes and readers **sees all history** of all operations and this requirement **ensures** that readers see the most resnet write.<br>

<br>

### Sequential consistency
**Sequential consistency** provides following **guarantee**:
- **logical order guarantee**:
  - **all** operations **occur** in a **logical order**;

<br>

**Sequential consistency** **relaxes** *real-time global oredring of all operations* and it **allows interleaving** of operations *at the global level*, but **prohibit reordering** of operations *at the process level*.<br>

**Sequential consistency** **requires** that **all** operations must be executed **exactly** in the **order** at which they were issued by each process, but the **total order** is **not guaranteed**. In other words,
- if process `P1` issues operation `A` **before** operation `B`, then `A` **must be seen before** `B` *at the global level*;
- if process `P1` issues operation `A` **before** process `P2` issues operation `X`, then `A` and `X` *can be seen* **in any order** *at the global level*;

<br>

This means, that in a *sequentially consistent system* **result** of operations is **NOT** *deterministic*.<br>

Indeed, in the above example, `r(x)` of `P2` can return `0` or `9`, depending on interleaving of operations *at global level*:<br>

![seq_cons_1](/img/seq_cons_1.png)
![seq_cons_2](/img/seq_cons_2.png)

<br>

**Not valid** interleaving of operations at global level:<br>
![not_seq_cons](/img/not_seq_cons.png)

<br>

### Causal consistency
**Causal consistency** provides following **guarantee**:
- **causal order guarantee**:
  - **only** operations with a **cause-and-effect relationship** must be seen in the correct order (in an order that respects causality) across all nodes in a distributed system;

<br>

**Causal consistency** is suitable for **collaborative systems** where logical ordering matters but strict synchronization isn’t needed.<br>

<br>

### Eventual consistency
**Eventual consistency** **allows** **temporary inconsistencies** between nodes, with a **guarantee** that **all** replicas will *eventually converge to the same state* (*eventually become consistent*).<br>

In practice, *eventual consistency* means that **updates propagate asynchronously** across replicas, leading to temporary inconsistencies between replicas.<br>

**Eventual consistency** is suitable for **high-availability systems**, such as *web caching*, *social media*, and *CDNs*, where occasional stale reads are acceptable.<br>

<br>

# The CAP theorem
The **CAP theorem** states that a *distributed system* **cannot provide all 3 CAP properties** (**C**/**A**/**P**), but **only any 2 of them**:
- **Consistency** (**C**): every **read** receives the **most recent write** or **an error**;
- **Availability** (**A**): every request receives a (*non-error*) **response**, **without** the **guarantee** that it contains the *most recent write*;
- **Partition tolerance** (**P**): is the **ability** of a *distributed system* to **continue operating** despite arbitrary *network failures* (*network partitions*) between nodes;
  - **P** means that messages are **dropped** or **delayed** by the network between nodes;

<br>

**Network partitions** occur when **communication** between components is **interrupted**, but each component continues to operate independently. Components operate independently, causing state divergence.<br>

<br>

*In practice*, for any a *distributed system*, **P** **is given**: **network failures happens**. This means you **must choose between** **C** and **A** during a **network partition**:
- **CA systems** provide both **C** and **A**:
  - **CA** means that in the case of **P**, the **system becomes inoperable**;
  - **examples**: *MySQL*, *Postgres*;
- **linearizable systems** (aka **CP systems**) **prioritize consistency**:
  - **CP** means that in the case of **P**, the system refuse **A** in favor of **C**;
  - during a *network partition* **some of nodes** *might become* **unavailable** to some clients to *ensure* **all remaining available nodes** have a **consistent** view of the data;
  - **examples**: *etcd*, *ZooKeeper*, a distributed databases using *Paxos/Raft*;
- **eventually consistent systems** (aka **AP systems**) **prioritize availability**:
  - **AP** means that in the case of **P**, the system refuse **C** in favor of **A**;
  - during a *network partition* **all nodes remain available**, but **some of nodes** *might have* **inconsistent** views of the data and *might return* **not actual data**;
  - **consistency** is **eventually achieved** when the network partition is resolved;
  - **examples**: *Cassandra*, *DynamoDB*, *CouchDB*, *Riak*, *Elastic Search*;
