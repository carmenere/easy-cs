# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Properties of reliabile systems](#properties-of-reliabile-systems)
  - [Disaster recovery](#disaster-recovery)
  - [Availability. High availability](#availability-high-availability)
  - [Fault tolerance](#fault-tolerance)
- [Fault vs. Error vs. Failure](#fault-vs-error-vs-failure)
- [Fault models](#fault-models)
<!-- TOC -->

<br>

# Properties of reliabile systems
Properties of reliabile system:
- **Disaster recovery** (**DR**);
- **Fault tolerance** (**FT**);
- **High availability** (**HA**);

<br>

## Disaster recovery
**Disaster recovery** (DR) is an ability of organization or system to **recover from catastrophic events**.<br>
The goal of DR is to enable the organization or system to **quickly resume work** as soon as possible **after** a disaster occurs.<br>

**DR** consists of **IT technologies** and best practices designed to **circumvent** or **minimize** data loss and business disruption resulting from catastrophic events.<br>
**DR** is a **crucial** aspect of **business continuity**.<br>

A **disaster recovery plan** (**DRP**) is a documented that describes how an organization can quickly resume work after an unplanned incident.<br>
A **DRP** is an essential part of a **business continuity plan** (**BCP**).<br>

Types of disaster include:
- Cyber attacks such as malware, DDoS and ransomware attacks;
- Sabotage;
- Power outages;
- Equipment failure;
- Epidemics or pandemics, such as COVID-19;
- Terrorist attacks or threats;
- Industrial accidents;
- Hurricanes;
- Tornadoes;
- Earthquakes;
- Floods;
- Fires;

<br>

## Availability. High availability
An **uptime** an amount of time a system is performing its mission.<br>
A **downtime** an amount of time a system is **not** performing its mission.<br>

The **availability** of system is the **probability** that the system **can perform** its **mission** (**is available for service**) when needed.<br>

**Availability** is usually expressed as a **percentage** of **uptime per year**: `Availability = uptime / (uptime + downtime) * 100`<br>

**High availability** (**HA**) means that a system has a **high level** of **availability** and can operate **continuously**, without intervention, for a given time period.<br>

**Availability** is frequently measured by the number of **nines**: **99%** availability is called **2 nines**, **99.9%** availability is called **3 nines** and so on.<br>

|Availability %|Downtime per year|
|:-------------|:----------------|
|90% (**one nine**)|36.53 days|
|99% (**two nines**)|3.65 days|
|99.9% (**three nines**)|8.77 hours|
|99.99% (**four nines**)|52.60 minutes|
|99.999% (**five nines**)|**5.26 minutes**|

<br>

## Fault tolerance
**Fault tolerance** is the **ability of a system** to **continue operating** properly in the case of the **failure** of one or more its components.<br>
**Byzantine fault tolerance** means system **can to resist** the *byzantine failure*.<br>

A **fault-tolerant system** is a system that **can resist failures**, i.e. it **designed** to continue operating without interruption, even if one or more components **fail**.<br>
**BFT-based systems** are designed to resist **byzantine failure**.<br>

If system can resist **any k concurrent failures**, it is said to be **k-fault tolerant** and **k** is called **degree of fault tolerance** or **resiliance**.<br>
To resist the attacks of **k** *byzantine failures* a **BFT-based systems** must have **3k+1** nodes.<br>

<br>

Techniques for achieving *fault tolerance:*
1. **Redundancy**: eliminate single point of failure, i.e., adding **redundant** components.
2. **Replication**.
3. **Failure detection**.
4. **Failover** or **statefull failover**. **Failover** is switching to a **redundant** or **standby** component of FT system **upon the failure**.

<br>

# Fault vs. Error vs. Failure
`Fault` -> `Error` -> `Failure`:
- a **fault** (aka **defect** or **malfunction**), represents the **physical** or **logical** condition that **may lead** to **error**, which in turn **may lead** to **failure**
  - i.e., a **fault** is the **cause of** an **error** or **failure**;
  - a **fault** *may exist* **without** *being seen*;
  - **faults** are typically caused by **hardware defects**, **software bugs**, or **design flaw**;
  - **faults** are almost **non-avoidable**;
- an **error** is the **manifestation of a fault** that results in incorrect behavior or outputs from the system;
  - **errors** **may lead** to a **failure**;
- a **failure** is an **inability** of a **system** or **component** to **perform** required **functions** according to its specification;
  - a **failure** is the **result of** an activated *fault* and it is **visible** to end user;

<br>

# Fault models
A **fault model** tells you which **type of faults** can occur in a system.<br>

A crash failure is a system fault where a component, such as a process, node, or server, suddenly stops functioning and ceases all operations, typically due to hardware failure, software bugs, or resource exhaustion. 

**Fault models**:
- **crash failures** or just **crashes**:
  - **fail-stop failure** (the simplest failure model):
    - **characteristics**:
      - a component can **suddenly stop** working and **does not recover**;
      - the *failed component* **no longer sends** or **receives** any messages;
      - *other components* can **easily detect** this failure via **timeouts**;
  - **fail-recovery failure**:
    - **characteristics**:
      - a component can **suddenly stop** working, but **can recover** and **resume operations**;
      - *other components* can might mistakenly **continue interacting** with the *failed component*;
      - it is **harder to detect**;
  - **detection**:
    - **health checks**;
    - **retry timeouts**;
  - **mitigation**:
    - **redundancy**;
    - **replication**;
    - **failover**;
- **omission failure**:
  - **characteristics**:
    - a component **fails** to **receive** and/or **send** a message;
    - **messages lost** in transit;
    - the system continues operating, but **messages** are **lost** and **not delivered**;
  - **detection**:
    - **health checks**;
    - **retry timeouts**;
    - **monitoring network health**;
  - **mitigation**:
    - **redundancy**: **additional links/connections**;
    - **failover**;
- **temporal failures**:
  - **characteristics**:
    - a component **send** or **receive** messages **too late** or **too early**
    - this **may lead** to **incorrect ordering** or **duplicates** which in turn **may lead** to *data inconsistency*, *race conditions* or *synchronization issues*;
  - **mitigation**:
    - **vector clocks**;
    - **consensus algorithms** (such as **Paxos** and **Raft**);
- **byzantine failure**:
  - **characteristics**:
    - a component behaves **unpredictably** or **arbitrarily**;
    - a component **intentionally mislead** *other components* by sending **contradictory**, **false**, **malicious** or **incorrect** data to *other components*
    - a component **sabotages the whole system**;
  - **mitigation**:
    - **PBFT algorithm**
      - we need **3k+1** nodes to resist the attacks of **k** *byzantine failures*;
