# BFS
**Best-case and worst-case time complexity** is $O(|V|)$, where $|V|$ is the number of nodes. You need to traverse all nodes.<br>
**Worst-case space complexity** is $O(|V|)$ since at **worst** case you need to hold all vertices in the **queue**.<br>

<br>

# DFS
**Best-case and worst-case time complexity** is $O(|V|)$, where $|V|$ is the number of nodes. You need to traverse all nodes.<br>

**Space complexity** - depends on the implementation:
- **recursive implementation** has **worst-case space complexity** $O(h)$, where $h$ is the **maximal depth** of **tree**. With a balanced tree, this would be $O(log \kern3pt |V|)$;
- **stack based implementation** of DFS has **worst-case space complexity** $O(|V|)$;
