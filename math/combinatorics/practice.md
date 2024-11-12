# Table of contents
- [Table of contents](#table-of-contents)
- [Exercises](#exercises)
  - [Exercise 1](#exercise-1)
    - [Solution 1](#solution-1)
      - [Answer](#answer)
    - [Solution 2](#solution-2)
      - [Answer](#answer-1)
  - [Exercise 2](#exercise-2)
    - [Solution 1](#solution-1-1)
      - [Answer](#answer-2)
    - [Solution 2](#solution-2-1)
      - [Answer](#answer-3)
  - [Exercise 3](#exercise-3)
    - [Solution 1](#solution-1-2)
      - [Answer](#answer-4)
    - [Solution 2](#solution-2-2)
      - [Answer](#answer-5)
  - [Exercise 4](#exercise-4)
    - [Solution 1](#solution-1-3)
      - [Answer](#answer-6)
    - [Solution 2](#solution-2-3)
      - [Answer](#answer-7)
    - [Solution 3](#solution-3)
      - [Answer](#answer-8)
  - [Exercise 5](#exercise-5)
    - [Solution 1](#solution-1-4)
      - [Answer](#answer-9)
    - [Solution 2](#solution-2-4)
      - [Answer](#answer-10)
  - [Exercise 6](#exercise-6)
    - [Solution 1](#solution-1-5)
      - [Answer](#answer-11)
  - [Exercise 7](#exercise-7)
    - [Solution 1](#solution-1-6)
      - [Answer](#answer-12)
  - [Example 1](#example-1)
  - [Example 2](#example-2)
  - [Example 3](#example-3)
  - [Example 4](#example-4)
  - [Example 5](#example-5)

<br>

# Exercises
## Exercise 1
Дана квадратная таблица **2x2** клеток. Каждую клетку такой таблицы можно покрасить в **белый** либо в **черный** цвет.
**Сколько** существует различных раскрасок этой таблицы?

### Solution 1
Полностью переберем **все** варианты. Все варианты распадаются на *4 категории*:
1. Есть **2** одноцветные раскраски:<br>
![square_1](/img/square_1.png)
2. Есть **4** раскраски, в которых ровно **1** клетка *чёрная*:<br>
![square_2](/img/square_2.png)
3. Есть **4** раскраски, в которых ровно **1** клетка *белая*:<br>
![square_3](/img/square_3.png)
4. Есть **6** раскрасок, в которых **2** клетки белые, а **2** клетки чёрные:<br>
- **по диагонали**:<br>
![square_5](/img/square_5.png)
- **смежные**:<br>
![square_4](/img/square_4.png)

#### Answer
Т.о. получаем: **2** + **4** + **4** + **6** = **16** вариантов.<br>

### Solution 2
Если **последовательно** двигаться по клеткам, то на каждом шаге мы можем сделать **выбор** из **2** *вариантов* (т.к. всего **2** *цвета*: *чёрный* и *белый*).<br>
Всего таких шагов **4**, а значит нам придется делать **4** выбора.<br>

#### Answer
Т.о. получаем: $`2\cdot 2\cdot 2\cdot 2 = \textbf{2}^{\textbf{4}} = \textbf{16}`$ вариантов.<br>

<br>

## Exercise 2
Дано **7** букв: `A`, `B`, `C`, `D`, `E`, `F`, `O`.<br>
Нужно придумать слова из **2-х** букв, но в каждом слове на **1-м** месте должна стоять **согласная**, а на втором - **гласная**.

### Solution 1
Переберем все варианты, для этого выпишем их в виде таблицы:<br>
||`A`|`E`|`O`|
|:-|:---|:--|:--|
|**`B`**|`BA`|`BE`|`BO`|
|**`C`**|`CA`|`CE`|`CO`|
|**`D`**|`DA`|`DE`|`DO`|
|**`F`**|`FA`|`FE`|`FO`|

#### Answer
Т.о. получаем: **12** вариантов.<br>

### Solution 2
При формировании слова из **2-х** букв нам нужно сделать **4** выбора, но количество **вариантов** на **первом** шаге и **последующих** **отличаются**:
- в **первый** раз мы можем выбирать из **4-х** вариантов (из 4 согласных);
- **после** первого выбора мы можем выбрать только из **3-х** вариантов (из 3 гласных);

#### Answer
Т.о. получаем: $`4\cdot 3 = \textbf{12}`$ вариантов.<br>

<br>

## Exercise 3
Сколько мы можем сформировать чисел длиной **3** (3-х разрядных чисел), если в числе могут использоваться **только** цифры: `1`, `2`, `3`, `4`, `5`?<br>

### Solution 1
Переберем все варианты. Будем выписывать все числа в порядке возрастания, начнем с тех, у которых в первом разряде 1:
||||
|:-|:-|:-|
|**1**23|**1**24|**1**25|
|**1**32|**1**34|**1**35|
|**1**42|**1**43|**1**45|
|**1**52|**1**53|**1**54|

Итого: **12** вариантов.

Сформируем еще **4** такие таблицы для оставшихся 4-х цифр: далее будут те, в которых в первом разряде **2**, и т.д.<br>

В каждой таблице будет по **12** вариантов. А **всего** таких таблиц: **5**.<br>

#### Answer
Т.о. получаем: $`12\cdot 5 = \textbf{60}`$ вариантов.<br>

### Solution 2
Чтобы сформировать число длиной 3 без повторений из алфавита длины 5 нам нужно сделать 3 выбора, причем, каждый выбор будет уменьшать варианты для следующего выбора:
- на **первом** шаге нам доступны **все** **5** вариантов;
- на **втором** шаге нам доступно на 1 вариант меньше: только **4** варианта;
- на **третьем** шаге нам доступно на 2 варианта меньше: только **3** варианта;
- и т.д.

#### Answer
Т.о. получаем: $`5\cdot (5-1) \cdot (5-2) = 5 \cdot 4 \cdot 3 = \textbf{60}`$ вариантов.<br>

<br>

## Exercise 4
В турнире принимают участие **8** человек. Каждый участник сыграет с каждым ровно **1** раз.<br>
**Сколько** всего игр будет сыграно?<br>

### Solution 1
Нарисуем **full-mesh** граф, в котором **вершны** - это **участники**, а **ребра** - это **игры**.
**Количество ребер** в **full-mesh** графе: т.к. из каждой вершины исходит **n-1** ребер, а всего вершин **n**, и каждое ребро считает вершину **дважды**: **A->B** и **B->A**, то всего имеем
$`{\frac {n\cdot (n-1)}{2}}`$ ребер.<br>

#### Answer
Т.о. получаем: $`{\frac {8\cdot (8-1)}{2}} = 4 \cdot 7 = \textbf{28}`$ вариантов.<br>


### Solution 2
Мы можем посчитать количество уникальных пар $`(x,y)`$.
При этом, пары $`(x,y)`$ и $`(y,x)`$ будут различными. Но обе такие пары дают одну игру. Поэтому нужно разделить количество уникальных пар на **2**.<br>

Посчитаем количество уникальных пар так:
- на **первом** шаге у нас **8** вариантов для выбора;
- на **втором** шаге у нас уже **7** варианта для выбора;

#### Answer
Т.о. получаем: $`{\frac {8 \cdot 7}{2}} = {\frac {56}{2}} = \textbf{28}`$ вариантов.<br>


### Solution 3
Найдем число сочетаний $`C_{n}^{2}`$, где **n** - это общее количество участников в турнире.<br>


#### Answer
Т.о. получаем: $`C_{8}^{2} = {\frac {8!}{2!(8-2)!}} = {\frac {8\cdot 7}{2!}} = 4 \cdot 7 = \textbf{28}`$ вариантов.<br>

<br>

## Exercise 5
У Ивана 5 друзей. Он хочет позвать в гости только 3 из них. Сколько существует способов сделать это?<br>

### Solution 1
МЫ можем подсчитать количество **уникальных** комбинаций из **3х** друзей, а затем разделить на **3!**, т,к, порядок в каждой комбинации **не** имеет значения и все комбинации из **3** друзей можно трактовать как одну.<br>

Посчитаем количество уникальных комбинаций так:
- на **первом** шаге у нас **5** вариантов для выбора;
- на **втором** шаге у нас уже **4** варианта для выбора;
- на **втором** шаге у нас уже **3** варианта для выбора;

#### Answer
Т.о. получаем: $`{\frac {5 \cdot 4 \cdot 3}{3!}} = {\frac {60}{6}} = \textbf{10}`$.<br>

### Solution 2
Найдем число сочетаний $`C_{5}^{3}`$.<br>

#### Answer
Т.о. получаем: $`C_{5}^{3} = {\frac {5!}{3!2!}} = {\frac {5\cdot 4 \cdot 3!}{3!2!}} = {\frac {20}{2}} = \textbf{10}`$

<br>

## Exercise 6
К девочке пришла подруга. У девочки есть **7** печений, **5** пироженных и **10** конфет.<br>
Она решила дать подруге только **2** какие-нибудь сладости.<br>
**Сколько** разных вариантов угощения она может составить?<br>

### Solution 1
Существует **3 независимых** пары сладостей:
1. печенье + пироженное
2. печенье + конфета
3. пироженное + конфета

Для каждой пары число вариантов будет разным:
1. печенье + пироженное: даёт $`7\cdot 5 = 35`$
2. печенье + конфета: даёт $`7\cdot 10 = 70`$
3. пироженное + конфета: даёт $`5\cdot 10 = 50`$

#### Answer
Т.о. получаем: $`35 + 70 + 50 = \textbf{155}`$ вариантов.<br>

<br>

## Exercise 7
В США дату принято записывать так: сначала **месяц**, затем **день**, потом **год**.
В Европе дату принято записывать так: сначала **день**, затем **месяц**, потом **год**.

**Сколько** есть дней в году, дату которых **нельзя однозначно понять**, не зная, каким способом она записана?

### Solution 1
В году есть **12** месяцев, поэтому числа от **1** до **12** **нельзя** однозначно интерпретировать, не зная формата.

Чтобы найти все такие неоднозначные даты, поступим так
- на **первом** шаге выберем любое число от **1** до **12**;
- на **втором** шаге выберем любое число от **1** до **12**;

Всего получим 144 даты.
Но из 144 дат есть ровно 12 дат, в которых и день и месяц одинаковыНапример, 03.03. Такие даты интерпретируются однозначно.

#### Answer
Т.о. получаем: $`144 - 12 = \textbf{132}`$ даты.










## Example 1
A restaurant offers 2 salads and 4 entrees.
1. How many different items can you choose if you get to choose 1 item?
2. How many different meals if you can choose 1 salad and 1 entree?

Solution<br>
To choose 1 item you can choose 1 salad **OR** 1 entree. There is no overlap. This means **AP**.<br>
So, $2+4=6$ items.<br>

You are choosing 1 salad **AND** 1 entree. You choose the second event **after** the first event occurs. This means **MP**.<br>
So, $2×4=8$ meals.<br>

<br>

## Example 2
A lock will open with the correct choice of 3 numbers from 0 to 99 inclusive.<br>
1. How many different sets of 3 numbers are there if the numbers **can** repeat?
2. How many different sets of 3 numbers are there if the numbers **cannot** repeat?

Solution<br>
All three numbers are needed. You need the first number **AND** the second number AND the third number. This is **MP**. There are 100 choices for each number.<br>
so, $100·100·100=1,000,000$.<br>

**Without** repeating means that after the first number is chosen, there is one less number available to choose. After the first two numbers are chosen, there are two less numbers to choose from.<br>
So, $100·99·98=970,200$.<br>

<br>

## Example 3
Find the **number of all solutions** of following inequality $`x^2 + y^2 <= 5`$.<br>
We can divide it into 6 disjoint cases: $`x^2 + y^2 = i`$, where $0 <= i <= 5$.<br>
Let $Si$ is set of all solutions for case $i$. Let denote **solution** as an **ordered** pair $(x,y)$.<br>
Then,<br>
- $i=0$: $|S0| = 1$, $`S_0 = \{(0,0)\}`$;
- $i=1$: $|S1| = 4$, $`S_1 = \{(1,0), (-1,0), (0,1), (0,-1)\}`$;
- $i=2$: $|S2| = 4$;
- $i=3$: $|S3| = 0$, $`S_3 = \{\} = ∅`$;
- $i=4$: $|S4| = 4$;
- $i=5$: $|S5| = 8$;

<br>

So, the **number of all solutions** = $`|S_0| + |S_1| + |S_2| + |S_3| + |S_4| + |S_5| = 1 + 4 + 4 + 0 + 4 + 8 = 21`$.

<br>

## Example 4
Consider sequence of cities: $A \rightarrow B \rightarrow C \rightarrow D$ and consider that there are **2 ways** from $A$ to $B$, **3 ways** from $B$ to $C$ and **4 ways** from $C$ to $D$.<br>
Let denote $S \rightarrow D$ set of all ways between **adjacent cities** $S$ and $D$.<br>
Obvious that all such sets in path $A \rightarrow B \rightarrow C \rightarrow D$ ($A \rightarrow B$, $B \rightarrow C$, $C \rightarrow D$) are **pairwise disjoint**.<br>
So, there are $2 * 3 * 4 = 24$ ways to choose **one** way $(a,b,c,d) \  \text{where} \  a∈A, b∈B, c∈C, d∈D$ from $A$ to $D$ or **equaly** the **total number** of ways from $A$ to $D$ is $2 * 3 * 4 = 24$.<br>

<br>

## Example 5
Let $`X = \{1,2, ..., 100\}`$ and let $`S = \{(a,b,c): a,b,c ∈ X, a \lt b \ \text{and} \  a \lt c\}`$. **Find** $|S|$.<br>

We can divide it into **99 disjoint cases**: $a=1, a=2, ..., a=99$.<br>
For every such case there are $(100 - a)$ choices for $b$ and $c$.
So, we use **AP** to sum results of **all** cases and **MP** to count variants for **concrete** case:
- the number of **ordered** triples $(a,b,c)$ for concrete value of $a$ is $(100-a) \times (100-a)$;
- the number of **ordered** triples $(a,b,c)$ for **all** values of $a$ is the sum of $`(100-i)^2$ for $i$ in $[1,99]`$;

So, $`|S| = 99^2 + 98^2 + ... + 1^2`$.