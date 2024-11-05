# Stochastic process
**Stochastic process** (/stəˈkæstɪk/) or **random process** (`X{t}`) is a **sequence** of **random variables** `X0`, `X1`, ..., typically indexed either by `ℕ` (**discrete-time stochastic process**) or `ℝ` (**continuous-time stochastic process**).

**Time series** is a **series of values indexed** in time order.

The **index set** of a stochastic process is usually interpreted as **time**, i.e., a random process that evolves over time.

Examples: 
- **martingales**;
- **Markov processes** where the **distribution** of `Xi+1` depends only on `Xi` and not on any previous states;

<br>

## Classification
### By stationary
1. **Stationary** stochastic process.
   - **Ergodic** stochastic process.
   - **Non-ergodic** stochastic process.
2. **Non-stationary** stochastic process.

<br>

## Filtrations
- **Filter** on a set `X`.
- **Filter** (aka **order filter**) is a **special subset** of a **partially ordered set**.
- The **generated filtration** or **natural filtration** associated to a stochastic process is a **filtration** associated to the process which records its past behaviour at each time.
- **Filtered probability space**.
- **Process adapted to the filtration**. Any stochastic process `X` is an adapted process with respect to its natural filtration.

In the theory of stochastic processes, **filtrations** are **totally ordered collections** of subsets that are used to model the information that is available at a given point.

<br>

![Examples](/img/stochastic_filtration.jpeg)

<br>

# Stationary process
The statistics of a **stationary process** do **not** change in time.<br>

There are several **forms of stationarity**:
- **strong-sense stationarity**;
- **weak-sense stationarity**;

<br>

**Strict stationary process** (aka **strong stationary process**) is a **stochastic process** whose **probability distribution** (**CDF** and **PDF**/**PMF**) does not change when shifted in time.<br>

For many applications **strict-sense stationarity** is too restrictive.<br>

**Weak stationary process** is a **stochastic process** whose** 1st moment** (i.e. the **mean**) and **autocovariance** does not change when shifted in time and that the **2nd moment** is finite for all times.<br>

<br>

## Ensemble average vs. time average
In the simplest sense,
- the **ensemble average** is analogous to **expected value**.
- the **time average** is analogous to **sample mean** (**sample average**), i.e., it is the **average value of a single outcome of a stochastic process**. 

<br>

So,
- **time-average** is an average for the i-th member of the ensemble;
- **ensemble-average** is an average for all members of the ensemble at a certain times;

<br>

![Examples](/img/time_ensemble_average.jpeg)

<br>

## Ergodic process
An **ergodic process** is a such **stochastic process** for which its **time average** converges to (or even equal to) **ensemble average**.<br>
**Ergodic process** is a **stationary process**, but **stationary process** might not be **ergodic**.

<br>

### Example
1. Each resistor has an associated thermal noise that depends on the temperature.
2. Take **N resistors** (N should be very large) and plot the voltage across those resistors for a long period.
3. For **each resistor** you will have a **plot**. Calculate the average value of that **waveform**; this gives you the **time average**.
4. There are **N plot** as there are **N resistors**.
5. These **N plots** are known as an **ensemble**.
6. Now take a particular instant of time in all those plots and find the average value of the voltage.
7. That gives you the **ensemble average** for each plot.
8. If **ensemble average** and **time average** are the same then it is **ergodic**.

<br>

# Non-ergodic process
**Non-ergodic process** is a process that is **not in ergodic**.

<br>

### Example
An **unbiased random walk** is **non-ergodic**. Its **expectation value** is `0` at all times, whereas its **time average** is a **random variable with divergent variance**.

<br>

# Non-stationary
**Non-stationary process** such **stochastic process** in which **means**, **variances**, and **covariances** **change over time**.<br>
**Non-stationary process** can have **trends**, **cycles** or combinations of the two.<br>

**Non-stationary process** are **unpredictable**. It means that the past observations may not be representative of the future ones.<br>

How to **test for non-stationarity**?<br>

It is important to test whether the **time series** is stationary or not. There are various statistical tests that can help you detect non-stationarity, such as the Augmented Dickey-Fuller (ADF) test, the Phillips-Perron (PP) test, or the Kwiatkowski-Phillips-Schmidt-Shin (KPSS) test.<br>
These tests compare the **null hypothesis** of non-stationarity against the **alternative hypothesis** of stationarity, and provide a **p-value** that indicates the probability of rejecting the null hypothesis.<br>
Generally, a low **p-value** (**less than 0.05**) suggests that the **time series** is **stationary**, while a high **p-value** (more than 0.05) suggests that the **time series** is **non-stationary**.
