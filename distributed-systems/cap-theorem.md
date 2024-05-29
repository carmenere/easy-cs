# CAP theorem
### Terms
**Consistency** means that **all** clients see the **same** data at the **same** time, no matter which node of **distributed system** they connect to.<br>
**Availability** means that **any** client making a request for data gets a **successful response**, even if one or more nodes of **distributed system** are down.<br>

A **partition** is a **lost** connection or **temporarily delayed** connection between nodes within **distributed system**.<br>
**Partition tolerance** means that the **distributed system must** continue to **operate** despite **any** number of interrupted communication between its nodes.<br>

<br>

### CAP theorem
**CAP theorem** (aka **Brewer's theorem**) imposes **restrictions** on the **distributed systems**.<br>

**CAP theorem**: it is **impossible** to build **distributed systems** that **simultaneously** meets **all** 3 following requirements:
1. **C**onsistency.
2. **A**vailability.
3. **P**artition tolerance.

<br>

# CP, CA, AP
So, **no one** of DBs can provide all CAP properties: **C**, **A**, **P**, but **only 2** of them.<br>

DBs are classified based on the **two** CAP characteristics they support::
- **CP**;
- **CA**;
- **AP**;

<br>

### CP
A **CP** databases provide **C** and **P**. **CP** means that **in the case** of **P**, the system **refuse A** in favor of **CP**.<br>
When a **partition tolerance** occurs between any two nodes, the system has to **shut down** the non-consistent node (make it unavailable) until the partition is resolved.<br>

Examples of **CP**: **Redis**, **Memcached**, **HBase**, **MongoDB** (**Non Replicated**).

<br>

### AP
An **AP** databases provide **A** and **P**. **AP** means that **in the case** of **P**, the system **refuse C** in favor of **AP**.<br>
When the **partition tolerance** occurs, **all nodes remain available** but some of nodes can return **not** actual data.<br>
When the **partition tolerance** is resolved, the **AP** databases typically **resync** the nodes to **repair all inconsistencies** in the system.<br>

<br>

Examples of **AP**: **Cassandra**, **Dynamo**, **CouchDB**, **Riak**, **Elastic Search**, **MongoDB** (**Replicated**).

<br>

### CA
An **CA** databases provide **C** and **A**. **CA** means that **in the case** of **P**, the system **becomes inoperable**.<br>

Examples of **CA**: **MySQL**, **Postgres**.
