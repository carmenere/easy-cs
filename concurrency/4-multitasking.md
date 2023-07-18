# Multitasking
**Multitasking** is a property of the **execution environment** (OS, app runtime, and so on) to provide **parallel** or **pseudo-parallel** processing of multiple tasks.<br>

In modern OS there are 2 approaches to achive multitasking: 
1.	**Multiprocessing** (aka process level multitasking) – multitasking at OS **processes** level, i.e., os schedules its **processes**.
2.	**Multithreading** (aka thread level multitasking) – multitasking at OS **threads** level, i.e., os schedules its **threads**.

<br>

## Process vs. thread
A **process** is an executable code located in an **isolated address space**. In other words, a process is a running instance of a program, while in the general case, one running program can spawn N processes.<br>
A **thread** is an independent flow of instructions that runs inside the **address space of a process**.<br>

<br>

Thread's properties:
- every process can be single threaded or multi-threaded;
- every process spawns **primary thread**;
- all threads of some process are executed in **the same** address space;
- every thread is scheduled by OS kernel like processes.

<br>

### POSIX threads (Pthreads)
**POSIX Threads** (aka **pthreads**) defines types, functions and constants to control threads.

<br>

# Scheduling
**Scheduling** is the action of assigning resources (workers) to perform tasks.<br>
**Scheduler** is a component of execution environment that dispatches tasks to workers, e.g., OS is execution environment and CPU/cores are workers. <br>

There 2 different types of scheduling:
1.	**Preemptive scheduling**.
2.	**Cooperative scheduling**.

<br>

## Preemptive scheduling
**Preemptive scheduling** involves the use of an **interrupt mechanism** which suspends the currently executing process or thread and invokes a scheduler to determine which process should execute next.<br>
Therefore, all processes/threads will get some amount of CPU time (**quantum**) at any given time.<br>
The **time slice** or **quantum** is the period of time for which a process or thread is allowed to run in a preemptive multitasking system.<br>

All modern OS use **preemptive scheduling**.

<br>

### Advantages
It has fine grained control over tasks in contrast to cooperative scheduler and allows evenly distribute them among workers.

<br>

### Disadvantages
It has **context switch** overhead.

<br>

## Cooperative scheduling
**Cooperative scheduling** is a multitasking in which the OS never initiates a context switch, instead, processes or threads voluntarily **yield control** back to **scheduler**.

<br>

### Advantages
Scheduling is **lightweight** because it doesn't require *context switching* of process/thread, it just **switches coroutines**.

<br>

### Disadvantages
There is risk that **poorly** designed program can consume all of the CPU time for itself.

<br>

# Threading models
There are 2 types of threads from programming point of view:
- **kernel-level threads** are scheduled by OS **kernel scheduler**.
- **user-level threads** (aka **coroutines**/**green threads**) are scheduled by **application scheduler** (aka **event loop**).

<br>

So there can be 3 threading models:
- **Kernel-level threading**: all threads inside process are **kernel threads**.
- **User-level threading**: all threads inside process are **green threads**.
- **Hybrid threading**: some of threads inside process are **kernel threads** and some of threads inside process  **green threads**.
