# Table of contents
- [Table of contents](#table-of-contents)
- [Elementary data structures](#elementary-data-structures)
- [Linked list](#linked-list)
    - [Operations](#operations)
  - [Doubly linked list](#doubly-linked-list)
    - [Searching](#searching)
    - [Insertion](#insertion)
    - [Deletion](#deletion)
- [Stack](#stack)
  - [Operations](#operations-1)
  - [Array based stack](#array-based-stack)
    - [Empty](#empty)
    - [Push](#push)
    - [Pop](#pop)
- [Queue](#queue)
  - [Operations](#operations-2)
  - [Array based queue](#array-based-queue)
    - [Enqueue](#enqueue)
    - [Dequeue](#dequeue)
- [Set](#set)

<br>

# Elementary data structures
- **Linked list**;
- **Stack**;
- **Queu**;
- **Set**;
- **Hash table**;
- **Binary search tree**;
- **Prefix tree**;
- **Heap**;

<br>

# Linked list
Kinds of **linked list**:
- **singly** *linked list*;
- **doubly** *linked list*;
- **circular** /ˈsɜːkjələr/ *linked list*;

<br>

There is **no random access** to elements in any *linked list*.<br>
Unlike an **array**, where the data in memory is **arranged strictly sequentially**, in a *linked list*, on the contrary, the data is **arranged chaotically** and the nodes of the list are **connected through links**. Due to this feature, you can add an **arbitrary number** of elements to a *linked list*, however, **access** will only be carried out **sequentially**. 

<br>

### Operations
|Ooperation|Compexity|
|:---------|:--------|
|**Search**|**O(n)**|
|**Insertion**|**O(1)**|
|**Deletion**|**O(n)**|

**Insertion** and **deletion** are **faster** operations on **doubly linked lists** than on **arrays**.<br>
In the worst case **insertion** and **deletion** take **O(n)** time in an **array**, compared with **O(1)** time for a **doubly linked list**, because elements needs to be moved by one position in **array**.<br>

<br>

## Doubly linked list
Each element `e` of a **doubly linked list** `L` contains **at least** 3 attributes: `key`, `next` and `prev`.<br>

**Properties**:
- **initially** `L.head = NIL`;
- if `L.head = NIL`, the list is **empty**;
- an attribute `L.head` points to the **first** element of the list;
- `e.next` points to its **successor** /səkˈsesər/ in the linked list;
- `e.prev` points to its **predecessor** /ˈpriːdɪˌsesər/;
- if `e.prev = NIL`, the element has **no predecessor** and is therefore the **first** element, or **head**, of the list `L`;
- if `e.next = NIL`, the element has **no successor** and is therefore the **last** element, or **tail**, of the list `L`;

<br>

![double-linked-list](/img/double-linked-list.png)

<br>

### Searching
The procedure `search(L, k)` finds the **first** element with `key = k` in list `L`:
```
search(L, k)
    e = L.head
    while e != NIL and e.key != k
        e = e.next
    return e
```

<br>

### Insertion
The procedure `insert-head(L, e)` inserts an element `e` immediately following **head**, i.e. the `e` becomes **new head** in `L`:
```rust
insert_head(L, e)
    e.next = L.head
    e.prev = NIL
    if L.head != NIL
        L.head.prev = e
    L.head = e
```

<br>

The procedure `insert-after(p, e)` inserts an element `e` immediately following `p`:
```rust
insert_after(p, e)
    e.next = p.next
    e.prev = p
    if p.next != NIL
        p.next.prev = e
    p.next = e
```

<br>

### Deletion
The procedure `delete(L, e)` deletes an element `e` from list `L`:
```rust
delete(L, e)
    if e.prev != NIL
        e.prev.next = e.next
    else
        L.head = e.next
    if e.next != NIL
        e.next.prev = e.prev
```

<br>

# Stack
**Stack** implements **LIFO** policy (last in, first out). It means that elements are always **inserted** to the **top** of the stack and **deleted** from the **top** of the stack.<br>
In other words `pop` method always returns the **last** inserted element.<br>

There are **2 kinds** of stacks:
- **array** based:
- **linked list** based:
  - the tail of **linked list** is the **top** of the stack;
  - `push` *inserts* **after tail**;
  - `pop` *deletes* **tail** and **retirns** it;

<br>

## Operations
|Operation|Complexity|Description|
|:--------|:---------|:----------|
|`push`|**O(1)**|**adds** an element to the **top** of the stack|
|`pop`|**O(1)**|**removes** an element from the **top** of the stack and **returns** it|
|`peek`|**O(1)**|**reads** the value of the **top** of the stack (**without deleting** it)|
|`empty`|**O(1)**|checks whether stack is **empty** or **not**|
|`size`|**O(1)**|returns **numbers of elements** in stack|
|`clear`|**O(n)**|**deletes all elements** from stack|
|`search`|**O(n)**||

<br>

## Array based stack
**Special atrbiutes**:
  - `top` holds the **index** the **top** of the stack;
  - `max_length` holds the allowed **max size** of a stack (the **max number** of elements);

**Properties**:
- **initially** `S.top = 0`;
- if `S.top = 0` the stack is **empty**;
- if `S.top = S.max_length` the stack is **full** and if we attempt to **push** an element, then the stack **overflows**;

<br>

![array-based-stack](/img/array-based-stack.png)

<br>

### Empty
```rust
empty(S)
    if S.top == 0
        return true
    else
        return false
```

<br>

### Push
```rust
push(S, e)
    if S.top == S.max_length
        return error("overflow")
    else
        S.top = S.top + 1
        S[S.top] = e
```

<br>

### Pop
```rust
pop(S, e)
    if empty(S)
        return error("underflow")
    else
        S.top = S.top - 1
        return S[S.top+1]
```

<br>

# Queue
The **queue** has **head** (aka first element) and **tail** (aka last element).<br>
**Queue** implements **FIFO** policy (first in, first out). It means that when element is **enqueued** it is put **after** the old **tail** and become **new tail**. It means that when element is **dequeued** it is took from **head** and next element becomes **new head**.<br>
In other words `dequeue` method always returns the **first** inserted element.<br>

There are **2 kinds** of queues:
- **array** based;
- **linked list** based:
  - `enqueue` *inserts* to **head**;
  - `dequeue` *deletes* **tail** and **returns** it;

<br>

## Operations
|Operation|Complexity|Description|
|:--------|:---------|:----------|
|`enqueue`|**O(1)**|**adds** new element at **tail**|
|`dequeue`|**O(1)**|**removes** a current **head** and **returns** it|
|`front`|**O(1)**|**returns** a current **head** (**without deleting** it)|
|`size`|**O(1)**|returns **numbers of elements** in queue|
|`clear`|**O(n)**|**deletes all elements** from queue|
|`search`|**O(n)**||

<br>

## Array based queue
**Special atrbiutes**:
  - `head` holds the **index** of the **head** of a queue;
  - `tail` holds the **index** of the **tail** of a queue;
  - `max_length` holds the allowed **max size** of a queue (the **max number** of elements);

**Properties**:
- **initially** `Q.head = Q.tail = 0`;
- if `Q.head = Q.tail` the queue is **empty**;
- if `Q.head = Q.tail + 1` the queue is **full** and if we attempt to **enqueue** an element, then the queue **overflows**;

<br>

![array-based-queue](/img/array-based-queue.png)

<br>

### Enqueue
```rust
enqueue(Q, e)
    if Q.head = Q.tail + 1
        return error("overflow")
    Q[Q.tail] = e
    if Q.tail == Q.max_length
        Q.tail = 0
    else
        Q.tail = Q.tail + 1
```

<br>

### Dequeue
```rust
dequeue(Q)
    if Q.head = Q.tail
        return error("underflow")
    e = Q[Q.head]
    if Q.head == Q.max_length
        Q.head = 1
    else
        Q.head = Q.head + 1
    return e
```

<br>

# Set
**Set** is an implemetation of **math set** with set operations.<br>
Also there can be **multiset**, that can contains dublicates.
