use dicer::eval;

fn main() {
    // simple expression
    let test = String::from("1d20 + 4 + min([2d4-MAX], 3) {3}");
    println!("parse: \"{}\"\n", test);

    let result = eval(&test).expect("uh oh");
    println!("{}", result.value);
    println!("{}", result.str);
}
