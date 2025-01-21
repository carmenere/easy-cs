# Recursion
**Recursion** is situation when function **calls itself**.<br>
**Recursive function** is any function that **calls itself**.<br>

**Recursion** spawns a **chain of function calls** on the stack **until** the **stopping condition** (aka **base condition**) is met. Evaluation of the **chain of function calls** is delayed until the stopping condition is met. If the recursion is **too deep** it will cause to **stackoverflow**.<br>

**Base condition** is a condition upon which the function **stops** calling itself and **returns**.<br>

<br>

# Tail recursion
**Tail recursion** is situation when function calls itself and such **call** is the **final action** in a function.<br>
**Tail call** is a situation when function returns result **by calling another function** instead of simply passing a variable holding the return value.<br>
**Tail call** means that such **call** is a **final action** in function.<br>

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
**Tail call elimination** (aka **tail call optimization**) is replacing recursion by **loop** (internally by compiler).<br>

However, **not** all functions can be tail-optimized. Only function with **linear iterative process** can be tail-optimized.<br>
If we have to process the recursive call’s return value in any way, then tail-call optimization is **impossible**.<br>
