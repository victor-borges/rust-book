use std::io::{stdin, stdout, Write};
use std::str::FromStr;

fn main() {
    print!("Enter temperature in degrees Celsius (°C): ");
    let mut input = String::new();

    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    let temperature_celsius = f64::from_str(&input.trim()).expect("Invalid number");
    let temperature_fahrenheit = (temperature_celsius * 9_f64/5_f64) + 32_f64;

    println!("{temperature_celsius} °C = {temperature_fahrenheit} °F");
}
