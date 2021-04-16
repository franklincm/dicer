# Grammar

1. _expression_ → _simple_expression_ | **fcount** **(** **die** **,** _condition_list_ **)**
2. _simple_expression_ → _term_ _simple_expression'_
3. _simple_expression'_ → **addop** _term_ _simple_expression'_ | ε
4. _term_ → _factor_ _term'_
5. _term'_ → **mulop** _factor_ _term'_ | ε
6. _factor_ → **num** _factor'_ | **[** **num** **d** **num** **addop** **extrema** **]** |**(** _simple_expression_ **)** | _min_ | _max_
7. _factor'_ → **d** **num** | ε
8. _min_ → **fmin** **(** _simple_expression_ **,** _simple_expression_ **)**
9. _max_ → **fmax** **(** _simple_expression_ **,** _simple_expression_ **)**
10. _condition_list_ → _condition_ _condition_list'_
11. _condition_list'_ → **,** _condition_ _condition_list'_ | ε
12. _condition_ → **relop** **num**
---

### tokens
1. **num** → \[1-9\]\[0-9\]*
2. **d** → d|D
3. **addop** → + | -
4. **mulop** → * | /
5. **relop** → > | < | >= | <= | =
6. **extrema** → MIN | MAX
7. **min** → min
8. **max** → max
9. **count** → count
---

### machines
1. whitespace
2. num
3. d
4. relop
5. extrema
6. min
7. max
8. count
9. catchall
---
### example valid strings

```
1d20 + 4 + min([2d4-MAX], 3)

max(10, 1d20 + 4) # evaluate 2 simple_expressions, return max value
min(2d20, 15)

max(10, max(1d10, 1d20))

[4d6 - MIN] # roll 4 six-sided dice, subtract the lowest value
[4d6 - MAX] # roll 4 six-sided dice, subtract the highest value

count(4d6, >4)
count(10d6, >=5, =1, =6, <=4)
```

