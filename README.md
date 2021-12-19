# Dicer

version 1.0.2

## About

A simple library for evaluating strings representing dice rolls in rust. Dicer
uses an _LL(1)_ grammar and a recursive descent parser to parse strings and
simulate randomly rolled dice.
See [grammar.md](https://github.com/gnullByte/dicer/blob/main/grammar.md)
for examples of valid strings.

## Example

```rust
use dicer::eval;

fn main() {
    // simple expression
    let test = String::from("1d20 + 4 + min([2d4-MAX], 3) {3}");
    println!("parse: \"{}\"\n", test);

    let result = eval(&test).unwrap();
    for res in result {
        println!("{} = {}", res.str, res.value);
    }
}
```

output:

```sh
parse: "1d20 * (3+2) + min([2d4 - MAX], 3)"

(6) * (3 + 2) + min([ (3 + 1) -3 ], 3) = 31
```
