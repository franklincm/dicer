### Grammar

1. digit → 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
2. **nzero** → 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
3. **num** →  **nzero** **num'**
4. **num'** → digit | ε
5. **Δ** → d | D
6. **die** -> **num** **Δ** **num**
7. **extrema** → MIN | MAX
8. **expression** → **simple_expression** **expression'**
9. **expression'** → **addop** **extrema** | **addop** **extrema** | ε
10. **simple_expression** → **term** **simple_expression'**
11. **simple_expression'** → **addop** **term** **simple_expression'** | ε
12. **term** → **factor** **term'**
13. **term'** → **mulop** **factor** **term'** | ε
14. **factor** → **num** **factor'** | **min** | **max** | **count**
15. **factor'** → **Δ** **num** | ε
16. **min** → min( **simple_expression** , **simple_expression** )
17. **max** → max( **simple_expression** , **simple_expression** )
18. **count** → count( **simple_expression**, **condition_list** )
19. **condition_list** → **condtion** **condtion_list'**
20. **condition_list'** → , **condition** **condition_list'** | ε
21. **condition** → **relop** **num**
22. **relop** → > | < | >= | <= | =
23. **mulop** → * | /
24. **addop** → + | -
---

### example valid strings

```
max(10, 1d20 + 4) # evaluate 2 simple_expressions, return max value
min(2d20, 15)

4d6 - MIN # roll 4 six-sided dice, subtract the lowest value
4d6 - MAX # roll 4 six-sided dice, subtract the highest value

count(4d6, >4)
count(10d6, >=5, =1, =6, <=4)
```
---

---

### tokens
1. `digit`
2. `extrema`
3. `(`
4. `)`
5. `+`
6. `-`
7. `*`
8. `/`
9. `=`
10. `<`
11. `>`
12. `min`
13. `max`
14. `count`

### machines
