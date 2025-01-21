# Qualified Names
A **qualified name** is an identifier that includes a **namespace name**, the **namespace separator**, e.g. `::`, and a **item name**.<br>
For example, `posix::getpid()`, here, the **namespace** is `posix` and the function name within the namespace (the **item name**) is `getpid()`.<br>

You must use qualified names from one namespace to access variables and functions in another.

<br>

# Name mangling
**Name mangling** is a mechanism used by compilers to encode **non**-unique names into **unique names** so that **linkers** can separate them.<br>
For example, `C++` supports **function overloading**, i.e., there can be more than one function with **the same name** but, **different parameters**. This technique of adding additional information to function names is called **name mangling**.<br>