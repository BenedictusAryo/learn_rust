use std::io;

static TITLE: &str = "
Fibonacci Sequence Generator
----------------------------

";

fn main() {
    println!("{TITLE}");

    // Input number of sequence
    let input_number = loop {
        println!("Input number of sequence:");
        let mut input_number = String::new();
        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed to read input");

        match input_number.trim().parse::<u64>() {
            Ok(0) => {
                println!("Please insert number greater than 0!\n");
                continue;
            },
            Ok(num) => break num,
            Err(_) => {
                println!("Please insert positive number input!\n");
                continue;
            }
        }
    };

    // Run Fibonacci Generator
    println!("\nRequested number of sequence: {input_number}");
    let fibonacci_sequence = generate_fibonacci_sequence(input_number);
    println!("Generated Fibonacci Sequence: {fibonacci_sequence:?}");

}

fn generate_fibonacci_sequence(number: u64) -> Vec<u64> {
    let mut sequence: Vec<u64> = Vec::new();

    let mut num1 = 0;
    let mut num2 = 1;
    let mut num3;

    for _ in 0..number {
        sequence.push(num1);
        num3 = num1 + num2;
        num1 = num2;
        num2 = num3;
    }
    sequence
}