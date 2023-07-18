# Flynn's taxonomy
**Flynn's taxonomy** is a classification of computer architectures, proposed by Michael J. Flynn.

<br>

Flynn defined 4 architectures:
- **SISD** (Single Instructions, Single Data)
- **SIMD** (Single Instructions, Multiple Data)
- **MISD** (Multiple Instructions, Single Data)
- **MIMD** (Multiple Instructions, Multiple Data)

<br>

## SISD
In **SISD** architecture a **single** CPU executes a **single instruction** over some data at every moment of time, i.e., the machine with **one** CPU and with **one** core.<br>
**SISD** machines can run only *concurrent* apps, **not** *parallel* apps.

<br>

## SIMD
In **SIMD** architecture a **single** CPU executes a **single instruction** over **vector** of data at every moment of time.<br>
Example: **MMX**, **SSE**, **SSE2**, **SSE3**, **SSE4**, **AVX**.

<br>

## MISD
In a **MISD** architecture, **multiple** CPUs execute **different instructions** on **the same data**.<br>
This type of architecture is **not** commonly used in practice.

<br>

## MIMD
In a **MIMD** architecture, **multiple** CPUs execute **different instructions** on **different data**.<br>
**MIMD** machines can run *concurrent* apps and/or *parallel* apps.

<br>

# Memory models
There are 2 memory models:
- **shared memory**: all CPUs/cores inside machine **share** a common, **central memory**. In the simplest form, all processors are connected to a bus which connects them to memory.
- **distributed memory**: every machine has its **own RAM** which isn't seen to other machines. For data to be shared, it must be passed from one machine to another as a **message**.

<br>

- **SM-MIMD** (aka **multiprocessor**) is a *MIMD* that uses **shared memory** model.
- **DM-MIMD** is a *MIMD* that uses **distributed memory** model (aka **NORMA**).

<br>

**Multiprocessor** can refer to machine with
- **single** but **multicore** CPU;
- **multiple** CPUs but every with **single** core;
- **multiple multicore** CPUs.

<br>
