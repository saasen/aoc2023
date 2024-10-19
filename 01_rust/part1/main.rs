use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let lines = contents.lines();
    let mut numbers = Vec::new();
    for line in lines {
        let mut digits = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                digits.push(c);
            }
        }
        let mut s = digits.into_iter();
        let first_digit = s.next().unwrap();
        let last_digit = match s.last() {
            Some(inner) => inner,
            None => first_digit
        };
        let mut acc = String::new();
        acc.push(first_digit);
        acc.push(last_digit);
        numbers.push(acc.parse::<i32>().unwrap());
    }

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}