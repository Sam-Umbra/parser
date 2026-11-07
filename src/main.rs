use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
};

use parser::expression::Expression;
fn main() {
    // let input = "2*6+4*5";
    // let mut sum = 0;
    // for term in input.split("+") {
    //     let mut prod = 1;
    //     for factor in term.split("*") {
    //         prod *= factor.parse::<i32>().unwrap();
    //         println!("Prod: {prod}")
    //     }
    //     sum += prod;
    //     println!("Sum: {sum}")
    // }
    // let math = 2 * 6 + 4 * 5;
    // println!("{} == {}", sum, math);

    let mut variables: HashMap<char, f64> = HashMap::new();
    loop {
        print!(">> ");
        stdout().flush().unwrap();
        let input = {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            buf
        };
        if input.trim() == "exit" {
            break;
        }
        let expr = Expression::from_str(&input);
        if let Some((var, left_side)) = expr.is_assign() {
            let value = left_side.eval(&variables);
            variables.insert(var, value);
            continue;
        }
        let value = expr.eval(&variables);
        println!("{}", value)
    }
}
