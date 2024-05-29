# Future vs. Promise
What is **computation**? It is a proccess of **transformig** some **input** into some **result** where *result* can be **success** or **failure**.<br>
What is **async** *computation*? It is a **promise** to provide the **result** of computation in the **future**.<br>

`Future` (in programming language) is an object that stores result of async operation and which can be accessed later. `Future` is created and returned to caller **before** async operation.<br>

`Promise` (in programming language) is a function that writes result to `Future`.<br>

`Future` and `Promise` are two sides of **caller implementor pattern**.<br>

For example, **non-blocking socket** is a `Future`, and it will hold data.
