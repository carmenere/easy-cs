# I/O bound vs. CPU-bound
There are 2 types of tasks: **I/O bound** and **CPU-bound**.<br>
- **CPU-bound** means the progresses of task is principally limited by the speed of the CPU.
- **I/O bound** means the progresses of task is principally limited by the time spent waiting for **input**/**output** operations to be completed, in other words, more time is spent requesting data than processing it.

<br>

> **Notes**: <br>
> Increasing performance of CPU speeds up **CPU-bound** tasks.<br>
> Increasing performance of CPU **doesn't** speed up **I/O bound** tasks.

<br>

- For **CPU-bound** tasks **parallelism** is better.
- For **I/O bound** tasks **concurrency** is better.