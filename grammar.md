# Grammar

1. _expression_ → _simple_expression_ _expression'_ | **count** **(** **die** **,** _condition_list_ **)**
2. _expression'_ → **addop** **extrema** | ε
3. _simple_expression_ → _term_ _simple_expression'_
4. _simple_expression'_ → **addop** _term_ _simple_expression'_ | ε
5. term → _factor_ _term'_
6. term' → **mulop** _factor_ _term'_ | ε
7. factor → **num** | **die** | _min_ | _max_
8. _min_ → **min** **(** _simple_expression_ **,** _simple_expression_ **)**
9. _max_ → **max** **(** _simple_expression_ **,** _simple_expression_ **)**
10. _condition_list_ → _condtion_ _condtion_list'_
11. _condition_list'_ → **,** _condition_ _condition_list'_ | ε
12. _condition_ → **relop** **num**
---

### tokens
1. **num** → \[1-9\]\[0-9\]*
2. **die** → **num** [d|D] **num**
2. **addop** → + | -
3. **mulop** → * | /
4. **relop** → > | < | >= | <= | =
5. **extrema** → MIN | MAX
6. **min** → min
7. **max** → max
8. **count** → count
---

### machines
- whitespace
- die
- num
- relop
- addop
- mulop
- extrema
- min
- max
- count
- catchall
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

