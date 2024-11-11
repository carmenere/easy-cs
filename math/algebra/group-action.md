# Table of contents
- [Table of contents](#table-of-contents)
- [Group action](#group-action)
  - [Left group action](#left-group-action)
  - [Right group action](#right-group-action)
- [The orbit-stabilizer theorem](#the-orbit-stabilizer-theorem)
  - [Fixed points](#fixed-points)
  - [Stabilizer](#stabilizer)
  - [Orbit](#orbit)
- [Examples](#examples)
  - [The orbit-stabilizer theorem](#the-orbit-stabilizer-theorem-1)
- [Burnside's lemma](#burnsides-lemma)
- [Rus](#rus)
  - [Неподвижные точки](#неподвижные-точки)
  - [Cтабилизатор](#cтабилизатор)
  - [Орбита](#орбита)
  - [Лемма бернсайда для группы перестановок](#лемма-бернсайда-для-группы-перестановок)

<br>

# Group action
There are 2 group actions: **left** and **right**.<br>

<br>

## Left group action
A **left group action** of a group $`G`$ **on** a **set** $`X`$ is a **function** $`\alpha: G \times X \leftarrow X`$ satisfying the following properties:
1. $`\alpha(e,x)=x`$ where $`e`$ is the **identity element** of $`G`$;
2. $`\alpha(g,\alpha(h,x))=\alpha(g\cdot h,x)`$ for all $`g,h`$ in $`G`$ for **all** $`g`$ and $`h`$ in $`G`$ and **all** $`x`$ in $`X`$.

The **group** $`G`$ is then said to **act on** $`X`$ (from the **left**).<br>

<br>

## Right group action
A **right group action** of a group $`G`$ **on** a **set** $`X`$ is a **function** $`\alpha: X \times G \leftarrow X`$ satisfying the following properties:
1. $`\alpha(x,e)=x`$ where $`e`$ is the **identity element** of $`G`$;
2. $`\alpha(\alpha(x,g),h)=\alpha(x, g\cdot h)`$ for all $`g,h`$ in $`G`$ for **all** $`g`$ and $`h`$ in $`G`$ and **all** $`x`$ in $`X`$.

The **group** $`G`$ is then said to **act on** $`X`$ (from the **right**).<br>

<br>

# The orbit-stabilizer theorem
Consider a group $`G`$ **acting on** a set $`X`$ and $`x \in X`$.<br>

<br>

## Fixed points
Some elements $`g`$ of a group $`G`$ **acting on** a set $`X`$ may **fix** a point $`x \in X`$.<br>

<br>

A **fixed point of** an element $`g \in G`$ is an **element** $`x∈X`$ **such** that $`g \cdot x=x`$. It is also said $`g`$ **fixes** $`x`$.<br>

**All** such **fixed points** form a **subset** of $`X`$ called the **group's set of fixed points**:
- $`Fix(g) = \{x \mid g \cdot x = x \space \space \forall g \in G\}`$, where $`Fix(g) \subseteq X`$

<br>

## Stabilizer
**All** elements $`g \in G`$ that **fix** $`x \in X`$ form a **subset** called the **stabilizer**.<br>
The **stabilizer** $`G_{x}`$ **of** a point $`x \in X`$ is the **set** of elements $`g \in G`$ **such** that $`x`$ is a **fixed point** of $`g`$:
- $`G_{x} = \{g \in G: g \cdot x = x\}`$

<br>

The **stabilizer** of $`x`$, in turn, forms a **subgroup** called the **stabilizer subgroup** of $`x`$ (aka **isotropy group**).<br>

It means that $`|G_{x}| \le |G|`$.<br>

<br>

## Orbit
The **orbit of** an element $`x \in X`$ **under** $`G`$ is the **set** of elements $`j \in X`$ such that $`g⋅x=j`$ for some $`g \in G`$:
- $`Orb_{G}(x) = G\cdot x = \{g \cdot x : g\in G\}`$

<br>

The **orbit** of an element $`x`$ in $`X`$ is the **set** of elements in $`X`$ to which $`x`$ can be moved by the elements of $`G`$.

<br>

# Examples
Let $`G`$ be a **group** of **permutations** $`S_{n}`$ on the **set** $`X = {1,2,\dots,n}`$.<br>

Consider elements of $`G = S_{3}`$:
- $`g_{1} = e = (1)`$;
- $`g_{2} = (124)`$;
- $`g_{3} = (142)`$;
- $`g_{4} = (35)`$;
- $`g_{5} = (124)(35)`$;
- $`g_{6} = (142)(35)`$;

Let $`G_{i}`$ be the **set of permutations** $`g`$ in $`G`$ that **fix** $`x`$, i.e. $`G_{x} = \{g \in G \mid g(x) = x\}`$.<br>

<br>

**Stabilizers**:
- $`G_{1} = \{e, g_{4}\}`$
- $`G_{3} = \{e, g_{2}, g_{3}\}`$

<br>

For each $`x`$ in $`X`$ the **orbit** of $`x`$ under $`G`$ is $`Orb_{G}(x) = \{g_{x} \mid g \in G\}`$.

**Orbits**:
- $`Orb_{G}(1) = \{1,2,4\}`$
- $`Orb_{G}(3) = \{3,5\}`$

<br>

## The orbit-stabilizer theorem
The **orbit-stabilizer theorem** (together with *Lagrange's theorem*): the **order** of the group $`G`$ is the **length** of the **orbit** of $`x`$ times the **order** of its **stabilizer**, also the **orbit length** is a **divisor** of the **group order**:
- $`|G| = |Orb_{G}(x)| \cdot |G_{x}|`$

<br>

**Proof**. There is one-to-one mapping between $`Orb_{G}(x)`$ and **cosets** of $`G_{x}`$, it means that the **index** of **stabilizer** $`[G : G_{x}]`$ is **equal** to $`|Orb_{G}(x)|`$.<br>

Then by *Lagrange's theorem*: $`|G| = [G : G_{x}] \cdot |G_{x}| \implies |G| = |Orb_{G}(x)| \cdot |G_{x}|`$.<br>

<br>

# Burnside's lemma
The **Burnside's lemma**:
- $`|X/G| = {\frac {1}{|G|}} \sum_{g\in G} |Fix(g)|`$, where $`Fix(g) \subseteq X`$ is the **subset** of points **fixed** by $`g`$.<br>

<br>

# Rus
Ясно, что **любые** две орибты либо **совпадают**, либо не **пересекаются**.<br>

Отсюда следует, что множество $`X`$ **под действием группы** $`G`$ **распадается** на **непереcекающиеся** подмножества - **орбиты группы** $`G`$.<br>

В частности, может случиться так, что **единственной орбитой** группы $`G`$ будет само множество $`X`$. В таком случае говорят, что группа $`G`$ **транзитивна**.<br>

Например, **группа перестановок** $`G`$ на множестве $`X`$ **транзитивна**, если **любой** элемент $`w \in X`$ может быть получен **из любого другого** элемента $`x \in X`$ **под действием** подходящим способом выбранной **перестановки** $`\phi \in G \mid \phi(x) = w`$. Все другие группы перестановок называются **интранизитивными**.<br>

<br>

В связи с разбиением множества $`X`$ на **орбиты группы** $`G`$ возникают 2 вопроса:
1. **Сколько** *орбит* имеет группа $`G`$ на множестве $`X`$?
2. Какова **мощность** каждой из этих *орбит*?

Ответ на **второй** вопрос дает **the orbit-stabilizer theorem**.<br>

<br>

## Неподвижные точки
**Неподвижная точка** или **неподвижный элемент** - такой элемент $`x \in X`$, который переходит сам в себя **под действием** элемента $`g`$ группы $`G`$:
$`Fix(g) = \{x \mid g \cdot x = x \space \space \forall g \in G\}`$.<br>

<br>

## Cтабилизатор
Для **любого** элемента $`x \in X`$ можно рассмотреть множество $`G_{x}`$ всех **перестановок** из группы $`G`$, для которых точка $`x \in X`$ является **неподвижной**. Это множество является **группой**, которая и называется **стабилизатором** точки $`x`$.<br>

<br>

**Пример**.<br>
Пусть $`X= {1,2,3}`$.<br>
Рассмотрим **симметрическую группу** перестановок $`S_{3}`$ и назовем её $`G`$.<br>

$`S_{3} = \{\phi_{1} = e = (1)(2)(3),\phi_{2} = (123),\phi_{3} = (132),\phi_{4} = (1)(23),\phi_{5} = (13)(2),\phi_{6} = (12)(3)\}`$.<br>

Найдем **стабилизатор** $`G_{1}`$ элемента $`1 \in X`$ в группе $`S_{3}`$:<br>

$`G_{1} = \{\phi_{1} = (1)(2)(3),\phi_{4}(1) = (1)(23)\}`$.

<br>

## Орбита
Пусть $`G = \{\phi_{1} = e,\space \phi_{2},\space \dots,\space \phi_{k}\}`$ – **симметрическая группа перестановок** $`S_{n}`$ на множестве $`X = \{1,2,\dots,n\}`$.<br>

Определим **бинарное отношение** $`R_{G}`$ на множестве $`X`$ следующим образом: если
$`a,b \in X`$, **то** $`aR_{G} b \iff ∃\phi \in G : \phi(a) = b`$.<br>

Если для элементов $`a,b \in X`$ верно $`aR_{G}b`$, то говорят, что
элементы $`a`$ и $`b`$ **эквивалентны по группе** $`G`$ (или
**относительно группы** $`G`$).<br>

Отношение $`R_{G}`$ является **отношением эквивалентности** на множестве $`X`$.<br>

**Орбитой** элемента $`a \in X`$ в группе $`G`$ называется
**порожденный им класс эквивалентности** по отношению $`R_{G}`$:
- $`Orb_{G}(a) = \{b \in N |∃ \phi \in G : \phi(a) = b\}`$.<br>

<br>

**Пример**.<br>
Пусть $`X = \{1,2,3\}`$.<br>
Пусть $`G = \{\phi_{1} = e = (1)(2)(3),\phi_{2} = (123),\phi_{3} = (132)\}`$ – **группа вращений** правильного треугольника в плоскости.<br>

Найдем **орбиту** $`Orb_{G}(1)`$ элемента $`1 \in N`$ в группе $`G`$: $`Orb_{G}(1) = \{\phi_{1}(1),\phi_{2}(1),\phi_{3}(1)\} = {1,2,3}`$.<br>

Другими словами, **перестановками** группы $`G`$ элемент $`1`$ может быть переведен в любой другой элемент множества $`X`$. Т.е. **вращениями** правильного треугольника в плоскости **вершина** $`1`$ может перейти в **любую другую** его **вершину**.<br>


## Лемма бернсайда для группы перестановок
Пусть
- $`Fix(\phi)`$ - **число неподвижных точек** перестановки $`\phi`$;
- $|X/G|$ - **число орбит** **группы** перестановок $`G = \{\phi_{1} = e, \space \phi_{2}, \space \dots, \space \phi_{n}\}`$, действующей на множестве $`X={1,2,\dots,n}`$;

<br>

**Лемма Бернсайда**: для **любой** группы перестановок имеет место равенство:
$`|X/G| = {\frac {1}{|G|}} \sum_{\phi \in G} |Fix(\phi)|`$

<br>

Пример. 

**Пример**.<br>
Пусть $`X = \{1,2,3\}`$.<br>
Пусть $`G = \{\phi_{1} = e = (1)(2)(3),\phi_{2} = (123),\phi_{3} = (132)\}`$ – **группа вращений** правильного треугольника в плоскости.<br>
Найдем **число орбит** элементов множества $`X = {1,2,3}`$ по группе $`G`$ вращений правильного треугольника в плоскости.<br>

Тогда
- $`Fix(\phi_{1}) = 3`$;
- $`Fix(\phi_{2}) = Fix(\phi_{3}) = 0`$;

Значит:
$`|X/G| = {\frac {1}{|G|}} \sum \limits_{i=1}^{3}|Fix(\phi_{i})| = {\frac {1}{3}}(3 + 0 + 0) = 1`$.