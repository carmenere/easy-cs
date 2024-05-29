# Recursion
**Recursion** is situation when function calls itself directly or indirectly.<br>
**Recursive function** is any function that **calls** an **instance of itself**.<br>
The **recursive function** internally utilizes a stack. If the recursion is **too deep**, it will eventually run **out of the stack** space.<br>

The key point here is that the evaluation of the function is delayed until the stopping condition insdie it is met. It creates a chain of deferred operations.<br>

**Base condition** is needed to stop the recursion otherwise infinite loop will occur.<br>
**Base condition** is a terminating scenario that does not use recursion to produce an answer.

<br>

# Tail recursion
**Tail call** is a subroutine call that is performed as the **final action** of a *procedure*, i.e., **tail call** is a specific situation within code in which a function returns an expected value **by calling another function** instead of simply passing a variable holding the return value.<br>
The name itself denotes that the function *called* to calculate the value to be returned is **at the end** (**tail**) of the function *calling* it.<br>

**Tail recursion** is situation when function calls itself and such **recursive call** is the **final action** in a function.

<br>

# Tail recursion vs. Non-tail recursion
## Non-tail recursive `sum()`
```python
def sum(vals):
  if vals:
    # recursive call
    return vals[-1] + sum(vals[:-1])
  else:
    # the base case
    return 0
```

<br>

Here, call `return vals[-1] + sum(vals[:-1])` is **not tail call** beacuse *final action* is `+` 2 values: `vals[-1]` and result of `sum(vals[:-1])`.<br>

Steps:
```c
sum([1,2,3])
  3 + sum([1,2])
  3 + 2 + sum([1])
  3 + 2 + 1 + (sum[])
  3 + 2 + 1 + 0
6
```

<br>

## Tail recursive `sum()`
```python
def sum(vals, s):
  if vals:
    # tail call
    return sum(vals[:-1], s + vals[-1])
  else:
    # the base case
    return s
```

<br>

Here, call `sum(vals[:-1], s + vals[-1])` is **tail call** beacuse it is *final action* and expression `s + vals[-1]` is evaluated before this call.<br>

Steps:
```c
sum ([1,2,3], 0)
  sum ([1,2,3], 0 + 3)
  sum ([1], 3 + 2)
  sum ([], 5 + 1)
6
```

<br>

# Tail call elimination
**Tail call** can be implemented without adding a new stack frame to the call stack.<br>
**Tail call elimination** (aka **tail call optimization**) is replacing recursion by **loop** (internally by compiler).<br>

However, not all functions can be tail-optimized. Only function with **linear iterative process** can be tail-optimized.<br>
If we have to process the recursive call’s return value in any way, then tail-call optimization is **impossible**.<br>
