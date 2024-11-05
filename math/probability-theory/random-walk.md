# Random process
**Stochastic process** (or **random process**) can be defined as a **collection of random variables** that is **indexed by some mathematical set**.<br>
Each random variable of the stochastic process is uniquely associated with an element in the set.<br>
The set used to index the random variables is called the **index set**.<br>
Often **index set** the interpretation of **time**.<br>

<br>

# Random walk
**Random walk** is a **random process** that describes a path that consists of a succession of **random steps** on some mathematical space.<br>

<br>

## One-dimensional random walk
**One-dimensional random walk** is the **random walk** on the integer number line `Z` which starts at `0`, and at each step **particle** moves `+1` or `−1` with **equal probability** (`1/2`).<br>

### Example
![CDF of arcsine distribution](/img/rand_walk.jpeg)

<br>

How long will the **particle** be **above** or **below** the x-axis during its walk?<br>

Answer `1/2` is **wrong**! The most time **particle** will spend a **significant part** of its time in any one half-plane. This is known as **Paul Levy arcsine law**.<br>

<br>

## Paul Levy arcsine law
Let `τ` is the **relative time** spent by a particle on the positive semi-axis during some interval of time `[0,t]`. The ratio `x = τ/t` will then have the **arcsine distribution** and its **CDF** involves the **arcsine**: `𝖯{τ/t < x} = F(x)= 2/π arcsin √x for x ∈ [0,1]; t>0`.