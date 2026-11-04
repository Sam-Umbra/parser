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
    
    fn init(s: &str) {
    let input = s;
    let pos = 0;
    let peek = input.chars().nth(pos);
    }

    fn slit(op: &str) -> i32 {
        op.parse::<i32>().expect("Should be an op symbol here")
    }

}
