# Grammar

1. _expression_ → _term_ _expression'_ | **count** **(** **die** **,** _condition_list_ **)**
2. _expression'_ → **addop** _term_ _expression'_ | ε
3. _term_ → _factor_ _term'_
4. _term'_ → **mulop** _factor_ _term'_ | ε
5. _factor_ → **num** | **die** _factor'_ | **(** _expression_ **)** |_min_ | _max_
6. _factor'_ → **addop** **extrema** | ε
7. _min_ → **min** **(** _expression_ **,** _expression_ **)**
8. _max_ → **max** **(** _expression_ **,** _expression_ **)**
9. _condition_list_ → _condition_ _condition_list'_
10. _condition_list'_ → **,** _condition_ _condition_list'_ | ε
11. _condition_ → **relop** **num**
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

max(10, max(1d10, 1d20))

4d6 - MIN # roll 4 six-sided dice, subtract the lowest value
4d6 - MAX # roll 4 six-sided dice, subtract the highest value

count(4d6, >4)
count(10d6, >=5, =1, =6, <=4)
```

