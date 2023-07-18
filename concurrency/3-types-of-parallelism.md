# Types of parallelism
## Bit level parallelism
**Bit-level parallelism** is a form of parallel computing based on **increasing** processor **word size**.<br>
However, the practical use of **bit-level parallelism** has already been exhausted at **64 bits**, further increase in the machine word is practically meaningless.

<br>

## Instruction level parallelism
**Instruction-level parallelism** (*ILP*) is technique of execution more than one CPU's instruction per CPU clock cycle.<br>

Consider following sequence of operations:
```
1: e = a + b
2: f = c + d
3: m = e * f
```

<br>

In the above snippet operation `3` depends on the results of operations `1` and `2`, so it cannot be evaluated until operations `1` and `2` are completed.<br>
However, operations `1` and `2` are independent of other operations, so they can be evaluated at the same time.<br>

<br>

There are two approaches to achieve instruction-level parallelism:
- *hardware* (aka **superscalar execution**);
- *software* (aka **VLIW**, **V**ery **L**ong **I**nstruction **W**ord).

<br>

### Superscalar techniques
- **instruction pipelining** is a technique for implementing ILP in which every instruction is executed in stages, e.g. instruction **fetch**, **decode**, **execute**, **memory access**, **register write**;
- **out of order execution** is a technique for implementing ILP in which instructions are executed in **any order** that **doesn't** violate data dependencies;
- **speculative execution**: is a technique for implementing ILP in which instructions are executed before their results are actually needed.

<br>

## Data level parallelism
**Data Level Parallelism** corresponds **SIMD**, i.e., **SIMD instrustions** of CPU implement **data level parallelism**.

<br>

## Task level parallelism
**Task level parallelism** (aka **task parallelism**) focuses on **distributing** and **execution** tasks on **independent** CPUs or even machines.<br>
In a multiprocessor system, **task parallelism** is achieved when each processor simultaneously executes a different thread or process on the same or different data.<br>
**Task level parallelism** is possible on machines with **MIMD** only and it is not possible on other achitectures.<br>
In contrast, **multitasking** is possible even on **SIMD**, but **multitasking** on **SIMD** is **pseudo-parallelism**.

<br>

### Concurrency vs. Parallelism
- **Parallelism** (aka **parallel computing**) means **simultaneity** at the **physical** level, i.e., **one** task is distributed on sub-tasks then every sub-task is executed on one or many CPUs, followed by combining the results into final result.
- **Concurrency** (aka **concurrency computing**) means **simultaneity** at the **logical** level: **many** *independent* tasks are distributed and executed on one or many CPUs over a certain period of time, tasks can interrupt already started ones before they finish, instead of waiting for them to end.
