use std::io;

fn main() {
    println!("Celcius to Fahrenheit Converter\n");
    
    let celcius = loop {
        println!("Input Celcius degree:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please insert number input!");
                continue;
            }
        };
    };
    let fahrenheit = celcius_to_fahrenheit(celcius);
    println!("{celcius} degree Celcius equals to {fahrenheit} degree Fahrenheit");
}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * (9.0/5.0) + 32.0
}