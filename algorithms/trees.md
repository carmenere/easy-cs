# Table of contents
- [Table of contents](#table-of-contents)
- [Trees](#trees)
  - [Tree terminology](#tree-terminology)
  - [Perfect tree](#perfect-tree)
  - [Tree traversal](#tree-traversal)
    - [Preorder](#preorder)
    - [Inorder](#inorder)
    - [Postorder](#postorder)
    - [Breadth First Search (BSF)](#breadth-first-search-bsf)
  - [Binary tree](#binary-tree)
  - [Binary search tree](#binary-search-tree)
  - [Operations](#operations)
    - [Inorder tree walk](#inorder-tree-walk)
    - [Search](#search)
    - [Min and Max](#min-and-max)
    - [Insertion](#insertion)
    - [Inorder successor](#inorder-successor)
    - [Deletion](#deletion)
- [Self-balancing BST](#self-balancing-bst)
  - [AVL vs. RB](#avl-vs-rb)
- [Trie](#trie)
  - [Trie](#trie-1)
  - [Radix tree](#radix-tree)

<br>

# Trees
A **tree** is a *linked data structure* that consists of **nodes** connected by branches (aka **links**). There are parent/child relationships between nodes in the tree.
A **rooted tree** is a tree in which a special (labeled) node is singled out. This node is called the **root of the tree** or just **root node**.<br>

<br>

**Properties**:
- each node in tree has exactly **one** parent node;
- **root node** has **no** parent;
- nodes that have the **same parent** are sometimes called **siblings**;

<br>

## Tree terminology
|Term|Meaning|
|:---|:------|
|**Ancestors** /ænsestəʳz/|Node’s parent, node’s parent’s parent and so on up to the **root**.|
|**Binary tree**|A tree with **degree 2**.|
|**Branch**|Element of tree that connects nodes.|
|**Child**|Node that is connected to another upper node, called its parent.|
|**Degree of node**|The number of children the node has.|
|**Degree of tree**|The **maximum degree** of any of tree’s node.|
|**Depth**|== Level|
|**Descendants** /dɪsendənts/|Node’s children, node’s children’s children and so on down to the leaves.|
|**External node**|== Leaf|
|**First common ancestor**|First common ancestor of any two nodes is the closest ancestor of both nodes.|
|**Height** (for **node**)|Length of the longest path from the node downward through the tree to a leaf node.|
|**Height** (for **tree**)|The same as the **root**’s height.|
|**Internal node**|Node that has at least one child.|
|**Leaf**|Node with **no** children.|
|**Level**|It is **distance** between **node** and the **root**.|
|**Node**|Element of tree that holds some data.|
|**Ordered tree**|Tree in which order of children is important.|
|**Parent**|Node that is connected to another bottom node, called its child.|
|**Predecessor**|The **predecessor** of a node `x` is the **previous** node in **tree traversal**. So there **3 types** of predecessor: **preorder predecessor**, **inorder predecessor** and **postorder predecessor**.|
|**Root**|Node that has **no** parent.|
|**Siblings**|All nodes that have the **same parent**.|
|**Subtree**|Node and all its descendants.|
|**Successor**|The **successor** of a node `x` is the **next** node in **tree traversal**. So there **3 types** of successor: **preorder successor**, **inorder successor** and **postorder successor**.|
|**Unordered tree**|Tree in which order of children doesn’t matter.|

<br>

## Perfect tree
**Full tree** is a tree in which every node has either zero children or as many children as the tree’s degree.
**Complete tree** is a tree in which **every level is completely full**, except possibly the bottom level.
**Perfect tree** is a **full tree** in which **all leaves are at the same level**.

<br>

**Properties** of **perfect binary trees**:
- The **total number** of nodes (**N**) in a **perfect binary tree** of *height* **H** is **N = 2<sup>H+1</sup>  – 1**;
- If a **perfect binary tree** contains **N nodes**, it has a *height* **H = log2(N + 1) – 1**;
- The number of leaf nodes (**L**) in a **perfect binary tree** of *height* **H** is **L = 2<sup>H</sup>**;
- The number of internal nodes (**I**) is **I = N – L**;
- There are **half** of the nodes are **leaves** and **half** of the nodes are **internal nodes** in a **perfect binary tree**;

<br>

## Tree traversal
**Tree traversal** is a visiting all the nodes in the tree in some order and performing operations on them.

There are 4 kinds of **tree traversal**: 
- **Preorder**;
- **Inorder**;
- **Postorder**;
- **Breadth First Search** (**BFS**);

**Preorder**, **Inorder** and **Postorder** are variants of **Depth First Search** (**DFS**).

<br>

|Preorder|Inorder|
|:-------|:------|
|![preorder](/img/preorder.png)|![inorder](/img/inorder.png)|
|**Postorder**|**BSF**|
![postorder](/img/postorder.png)|![bsf](/img/bsf.png)|

<br>

### Preorder
Node visiting order:
1. Node.
2. Nodes from **left** subtree.
3. Nodes from **right** subtree.

<br>

### Inorder
Node visiting order:
1. Nodes from **left** subtree.
2. Node.
3. Nodes from **right** subtree.

<br>

### Postorder
Node visiting order:
1. Nodes from **left** subtree.
2. Nodes from **right** subtree.
3. Node.

<br>

### Breadth First Search (BSF)
**BFS** algorithm visits all the nodes **level by level** from root to bottom **from left to right** at each level.<br>
**BFS** can use **queue**.<br>

<br>

## Binary tree
**Binary tree** is a **tree** in which **each node** has **at most** *two children*.<br>

Each **node** `n` of a **binary tree** `T` contains **at least** 4 attributes:
- `parent` stores pointer to **parent node**;
- `left` stores pointer to **left child**;
- `right` stores pointer to **right child**;
- `key` stores some value;


There are 2 kinds of binary tree:
- **binary search tree**;
- **binary heap** (**heap**);

<br>

## Binary search tree
**Binary search tree** (**BST**) is a **rooted binary tree** that **satisfies** following **binary search tree property**.<br>

The **binary search tree property**:
- let `x` is a **node** in a **binary search tree** `T`, then:
  - **all nodes** in **left subtree** must have `key` that **less than or equal to** `x.key`, i.e. if `y` is a node in the **left subtree** of `x`, then `y.key ≤ x.key`;
  - **all nodes** in **right subtree** must have `key` that **greater than or equal to** `x.key`, i.e. if `y` is a node in the **right subtree** of `x`, then `y.key ≥ x.key`;

<br>

## Operations
### Inorder tree walk
```rust
inorder(x)
  if x != NIL
    inorder(x.left)
    print x.key
    inorder(x.right)
```

<br>

### Search
The `search(x, k)` procedure returns a **pointer to a node** or  **NIL** if such node **doesn't** exist.<br>

To **search** for a node with a given key `key` in the entire **BST**, call the `search(T.root, key)` procedure, where `T.root` is a **pointer** to the **root of a subtree**.<br>

**Recursive** variant:
```rust
search(x, k)
  if x == NIL or k == x.key
    return x
  if k < x.key
    return search(x.left, k)
  else
    return search(x.right, k)
```

<br>

**Iterative** variant:
```rust
search(x, k)
  while x != NIL and k != x.key
    if k < x.key
      x = x.left
    else
      x = x.right
  return x
```

<br>

### Min and Max
The `min(x)` procedure returns a **pointer to the node** containing **minimum** key in the **subtree** with root at `x`.
The `max(x)` procedure returns a **pointer to the node** containing **maximum** key in the **subtree** with root at `x`.

<br>

```rust
min(x)
  while x.left != NIL
    x = x.left
  return x
```

<br>

```rust
max(x)
  while x.right != NIL
    x = x.right
  return x
```

<br>

### Insertion
The `insert(T, z)` procedure inserts a **new** node into a BST.<br>

<br>

```rust
z.left = NIL
z.right = NIL
z.p = NIL

insert(T, z)
  x = T.root      // node being compared with z
  y = NIL         // y will be parent of z
  while x != NIL  // descend until reaching a leaf
    y = x
    if z.key < x.key
      x = x.left
    else
      x = x.right
  z.p = y         // assign parent to z
  if y == NIL
    T.root = z    // tree T was empty
  else if z.key < y.key
    y.left = z
  else
    y.right = z
```

<br>

### Inorder successor
The `inorder_successor(x)` procedure returns the **inorder successor** of a node `x` in a BST if it exists, or `NIL` if `x` is the **last node** that would be visited during **inorder walk**.<br>

If the **right** subtree of node `x` is **nonempty**, then the **inorder successor** of `x` is just the `min` value in `x`’s **right** subtree.<br>
If the **right** subtree of node is **empty** and `x` has a **inorder successor** `s`, then `s` is the **lowest ancestor** of `x` whose **left child** is also an **ancestor** of `x`.<br>

<br>

```rust
inorder_successor(x)
  if x.right != NIL
    return min(x.right)
  else
    s = x.p
    while s != NIL and x == s.right
      x = s
      s = s.p
    return s
```

<br>

### Deletion
During deleting a node the subtrees need to move around within the BST.<br>
The subroutine `transplant(T, u, v)` replaces the subtree rooted at node `u` with the subtree rooted at node `v`.<br>

<br>

```rust
transplant(T, u, v)
  if u.p == NIL  // case when u is root of BST
    T.root = v
  else if u == u.p.left
    u.p.left = v
  else
    u.p.right = v
  if v != NIL  // because v may be NIL update v.p only if v is non-NIL
    v.p = u.p
```

<br>

```rust
delete(T, z)
  if z.left == NIL              // handle the case in which z has only one right children
    transplant(T, z, z.right)   // replace z by its right child
  else if z.right == NIL        // handle the case in which z has only one left children
    transplant(T, z, z.left)    // replace z by its left child
  else // deal with the remaining two cases, in which z has two children
    y = min(z.right)            // y is z’s inorder successor
    if y != z.right
      transplant(T, y, y.right) // replace y by its right child
      y.right = z.right         // z's right child becomes y's right child
      y.right.p = y
    transplant(T, z, y)         // replace z by its inorder successor y
    y.left = z.left             // and give z’s left child to y
    y.left.p = y
```


<br>

# Self-balancing BST
**Self-balancing BST** is any BST that **automatically keeps** its **height** (maximal number of levels below the root) equal to **log<sub>2</sub>N**, where `N` is number of **all nodes in tree**.<br>

**Balanced tree** guarantees that basic operations (`insertion`, `delete`, `search`, `inorder_successor`, `min`, `max`) all have `O(h)` time on a tree of **height** `h`.<br>

<br>

Examples of **self-balancing BST**:
- **AVL** tree;
- **Red-black** tree;
- **2-3** tree;
- **B** tree;

<br>

## AVL vs. RB
- **AVL** trees are **more rigidly balanced** and hence provide **faster look-up**. 
- For a **look-up intensive** task use an **AVL** tree.
- For an **insert/remove intensiv**e tasks, use a **Red-Black** tree.

<br>

# Trie
## Trie
**Trie** (aka **digital tree**, **prefix tree**) is **tree** where **key** is **not** stored inside node, instead every node store only one letter of some string and the **path from root to node** gives a **whole string** (**key**) that corresponds this that node.<br>

So, the **node's key** is a **path from the root to that node**.<br>

Assume you have the words `hello`, `hat` and `have`. Below is an example ot **trie** that stores all of them:
```
    e - l - l - o
  /
h - a - t
      \
       v - e
```

<br>

**Disadvantages**:
- **trie** stores only string types, so any other types must be converted to strings;
- it can **consume much memory** because on **every letter** it spends one or more **pointers**;

<br>

## Radix tree
**Radix tree** (aka **radix trie**) is **space-optimized trie** in which each node that is the only one child is merged with its parent.

Example:<br>
![radix-tree](/img/radix-tree.png)

<br>

- the **original trie** has **9** nodes and **8** edges;
- the **compressed radix trie** has **6** nodes and **5** edges;
