use rand::distributions::{Standard, Uniform};
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let range = Uniform::from(0..20);
    let mut numbers: Vec<i32> = rand::thread_rng().sample_iter(range).take(20).collect();
    numbers.sort_unstable();

    let median = numbers[numbers.len() / 2];
    println!("median is {median}");

    let mut occurrences = HashMap::new();

    for number in numbers {
        let occurrence = occurrences.entry(number).or_insert(0);
        *occurrence += 1;
    }

    println!("occurrences: {occurrences:?}");
}
