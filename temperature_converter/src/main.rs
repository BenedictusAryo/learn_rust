use std::io;

static TITLE: &str = "
Temperature Converter
---------------------
Choose option:
1) Celcius to Fahrenheit
2) Fahrenheit to Celcius
";

enum OptionChoice {
    One,
    Two
}

fn main() {
    println!("{TITLE}");

    // Choose Option
    let option: OptionChoice = loop {
        println!("Input option:");
        let mut choosen = String::new();
        io::stdin()
            .read_line(&mut choosen)
            .expect("Failed to read input");

        match choosen.trim().parse::<u32>() {
            Ok(1) => {
                println!("You choose Option 1\n");
                break OptionChoice::One;
            },
            Ok(2) => {
                println!("You choose Option 2\n");
                break OptionChoice::Two;
            },
            Ok(_) => println!("Please insert number of given option!\n"),
            Err(_) => {
                println!("Please insert a valid number!\n");
            }
        };
    };
    
    // Input degree
    let input_degree = loop {
        println!("Input degree:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please insert number input!\n");
                continue;
            }
        };
    };

    // Convert the input using the correct converter function
    // based on the user's choice
    let result = match option {
        OptionChoice::One => celcius_to_fahrenheit(input_degree),
        OptionChoice::Two => fahrenheit_to_celcius(input_degree),
    };

    // Print the result
    match option {
        OptionChoice::One => println!("\nResult:\n{input_degree} degree Celcius equals to {result}, degree Fahrenheit"),
        OptionChoice::Two => println!("\nResult:\n{input_degree} degree Fahrenheit equals to {result}, degree Celcius"),
    }

}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * (9.0/5.0) + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0/9.0)
}