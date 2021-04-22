use dicer::eval;

fn main() {
    // simple expression
    let test = String::from("1d20 + 4 + min([2d4-MAX], 3) {3}");
    println!("parse: \"{}\"\n", test);

    //let result = eval(&test).expect("uh oh");
    let result = eval(&test).unwrap();
    for res in result {
        println!("{} = {}", res.str, res.value);
    }
    //println!("{}", result.value);
    //println!("{}", result.str);
}
