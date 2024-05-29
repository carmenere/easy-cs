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
|**Root**|Node that has **no** parent.|
|**Siblings**|All nodes that have the **same parent**.|
|**Subtree**|Node and all its descendants.|
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
2. Node’s **left** child.
3. Node’s **right** child.

<br>

### Inorder
Node visiting order:
1. Node’s **left** child.
2. Node.
3. Node’s **right** child.

<br>

### Postorder
Node visiting order:
1. Node’s **left** child.
2. Node’s **right** child.
3. Node.

<br>

### Breadth First Search (BSF)
**BFS** algorithm visits all the nodes **level by level** from root to bottom **from left to right** at each level.

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

# Self-balancing BST
**Self-balancing BST** is any BST that **automatically keeps** its **height** (maximal number of levels below the root) equal to **log<sub>2</sub>N**, where `N` is number of **all nodes in tree**.<br>


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
