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

        match input_number.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please insert number input!\n");
                continue;
            }
        }
    };
    println!("Requested number of sequence: {input_number}")
}
