# Binary search
The `binary_search` procedure returns **index** of element if it **was** *found* or **-1** if it **wasn't** *found*.<br>

<br>

## Code
```rust
binary_search(A,val,p,r):
    if not(p < r)  // not(p < r) equal to (p >= r) and it means that element NOT found
        return -1
    
    q = ⌊(p+r)/2⌋

    if val > A[q]
        return binary_search(A,val,q+1,r)
    else if val < A[q]
        return binary_search(A,val,p,q)
    else   // it means val = A[q]
        return q
```

<br>

### Example on python3
```python
def binary_search(A,val,p,r):
    if not(p < r):
        return -1
    q = (p+r) // 2
    if val > A[q]:
        return binary_search(A,val,q+1,r)
    elif val < A[q]:
        return binary_search(A,val,p,q)
    else:
        return q

l1 = []
l2 = [1]
l3 = [1, 2]
l4 = [1, 2, 3]
l5 = [1, 3, 5, 6]
l6 = [1, 3, 5, 6, 7]


assert binary_search(l1, 8, 0, len(l1)) == -1
assert binary_search(l2, 0, 0, len(l2)) == -1
assert binary_search(l2, 1, 0, len(l2)) == 0
assert binary_search(l2, 7, 0, len(l2)) == -1
assert binary_search(l3, 0, 0, len(l3)) == -1
assert binary_search(l3, 1, 0, len(l3)) == 0
assert binary_search(l3, 2, 0, len(l3)) == 1
assert binary_search(l3, 3, 0, len(l3)) == -1
assert binary_search(l4, -1, 0, len(l4)) == -1
assert binary_search(l4, 0, 0, len(l4)) == -1
assert binary_search(l4, 1, 0, len(l4)) == 0
assert binary_search(l4, 2, 0, len(l4)) == 1
assert binary_search(l4, 3, 0, len(l4)) == 2
assert binary_search(l4, 4, 0, len(l4)) == -1
assert binary_search(l4, 5, 0, len(l4)) == -1
assert binary_search(l5, -1, 0, len(l5)) == -1
assert binary_search(l5, 0, 0, len(l5)) == -1
assert binary_search(l5, 1, 0, len(l5)) == 0
assert binary_search(l5, 2, 0, len(l5)) == -1
assert binary_search(l5, 3, 0, len(l5)) == 1
assert binary_search(l5, 4, 0, len(l5)) == -1
assert binary_search(l5, 5, 0, len(l5)) == 2
assert binary_search(l5, 6, 0, len(l5)) == 3
assert binary_search(l5, 7, 0, len(l5)) == -1
assert binary_search(l5, 8, 0, len(l5)) == -1
assert binary_search(l6, -1, 0, len(l6)) == -1
assert binary_search(l6, 0, 0, len(l6)) == -1
assert binary_search(l6, 1, 0, len(l6)) == 0
assert binary_search(l6, 2, 0, len(l6)) == -1
assert binary_search(l6, 3, 0, len(l6)) == 1
assert binary_search(l6, 4, 0, len(l6)) == -1
assert binary_search(l6, 5, 0, len(l6)) == 2
assert binary_search(l6, 6, 0, len(l6)) == 3
assert binary_search(l6, 7, 0, len(l6)) == 4
assert binary_search(l6, 8, 0, len(l6)) == -1
assert binary_search(l6, 9, 0, len(l6)) == -1
```