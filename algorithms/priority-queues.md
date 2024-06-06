# Priority queuee
One of the most popular applications of a *heap*: as an efficient **priority queue**.<br>
As with heaps, priority queues come in two forms: **max priority queues** and **min priority queues**.<br>

A **priority queue** is a data structure for maintaining a **set** `S` of elements, each with an associated value called a **key**.<br>
A **max priority queue** supports the following operations:
|Operation|Complexity|Description|
|:--------|:---------|:----------|
|`insert`|**O(log<sub>2</sub>N)**|**inserts** the element `x` with key `k` into the set `S`|
|`maximum`|**O(log<sub>2</sub>N)**|**returns** the element of `S` with the **largest** key|
|`extract-max`|**O(log<sub>2</sub>N)**|**removes** and **returns** the element of `S` with the **largest** key.|
|`increase-key`|**O(log<sub>2</sub>N)**|**increases** the value of element `x`’s key to the **new** value `k`, **new** value must be **greater than or equal to old**|

<br>

A **min priority queue** supports the following operations:
|Operation|Complexity|Description|
|:--------|:---------|:----------|
|`insert`|**O(log<sub>2</sub>N)**|**inserts** the element `x` with key `k` into the set `S`|
|`minimum`|**O(log<sub>2</sub>N)**|**returns** the element of `S` with the **smallest** key|
|`extract-min`|**O(log<sub>2</sub>N)**|**removes** and **returns** the element of `S` with the **smallest** key.|
|`decrease-key`|**O(log<sub>2</sub>N)**|**decreases** the value of element `x`’s key to the **new** value `k`, **new** value must be **less than or equal to old**|

<br>

The max priority queues can be used to **schedule** jobs. The max priority queue keeps track of the jobs to be performed and their relative priorities. When a job is finished or interrupted, the **scheduler** selects the **highest-priority job** by calling `extract-max`. The **scheduler** can **add** a **new job** to the queue at any time by calling `insert`.<br>
