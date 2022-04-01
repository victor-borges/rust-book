use std::io::{stdin, stdout, Write};
use std::str::FromStr;

fn main() {
    print!("Enter amount of fibonacci numbers to be generated: ");

    let mut input = String::new();

    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let sequence_size = i32::from_str(&input.trim()).unwrap();

    let mut first_term: i128 = 0;
    let mut second_term: i128 = 1;

    println!("{first_term}");
    println!("{second_term}");

    for _ in 0..sequence_size {
        let next_term = first_term + second_term;
        println!("{next_term}");

        first_term = second_term;
        second_term = next_term;
    }
}
