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

# Fault tolerance
**Fault tolerance** is the **property** that enables a system to **continue operating** properly in the case of the **failure** of one or more its components.

<br>

## Fault tolerance design
Еhe following approaches are used to achieve availability and fault tolerance:
1. Eliminate single point of failure, i.e., adding **redundant** components.
2. **Replication**.
3. **Failure detection**.
4. **Failover** or **statefull failover**.

<br>

**Failover** is switching to a **redundant** or **standby** component of FT system **upon the failure**.
