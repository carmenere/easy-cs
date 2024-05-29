# Types of code behavior
1. **Undefined** behaviour (**UB**): it is **syntactically valid**, but is is **not** ok to do. The executing program is **unpredictable**.
2. **Unspecified** behaviour: the **result** may be **different** with **different compilers** and **different machines** and it is **not** documented. *Unspecified behavior* **may** or **may not** result in **UB**.
3. **Implementation-defined** behaviour: the **result** may be **different** with **different compilers** and **different machines** and it is **documented**.

<br>

Sometimes compilers may diagnose **UB**/**US**, however, sometimes they are not designed to diagnose the **UB**.<br>

<br>

# Examples
## Undefined behaviour
### Division By Zero

```c
int val = 5;
return val / 0; // undefined behavior
```

<br>

### Signed integer overflow
```c
int x = INT_MAX;
printf("%d", x + 1); // undefined behavior
```

<br>

### Null pointer dereference
```c
val = 0;
int ptr = *val; // undefined behavior
```

<br>

## Unspecified behavior
The C language doesn't specify the order of evaluation of arguments, **left to right** or **right to left**, so, it is **unspecified behavior**.<br>

```c
void fun(int n, int m);

int fun1() {
    std::cout << "fun1";
    return 1;
}
int fun2() {
    std::cout << "fun2";
    return 2;
}

fun(fun1(), fun2()); // unspecified behavior
```

<br>

On **different machines** we will see **different results**: 
- `fun1` then `fun2`;
- `fun2` then `fun1`.

<br>

# Sequence points in C
In `C` language most of the operators `+`, `-`, `/`, `*`, `&`, `|` don’t have a standard defined order for evaluation for its operands.<br>

Consider example:
```c
#include <stdio.h>
int main()
{
   int i = 8;
   int p = i++ * i++;
   printf("%d\n", p);
}
```

<br>

The output of above program is also **undefined**. It may be **64**, **72**, or maybe something else.<br>
The subexpression `i++` causes a **side effect**, it modifies `i`, which leads to **UB** since `i` is also **referenced** elsewhere **in the same expression**.<br>

Some languages support conception of **sequence points**.<br>
A **sequence point** defines **point in a code** at which **all** side effects of **previous** evaluations are guaranteed to be complete, and **no** side effects from **subsequent** evaluations have yet been performed.<br>

The C standard defines following **sequence points**: `&&`, `||`, `?`, `,`.

<br>