use std::{io, error};

mod fibonacci;
mod temperature;
mod carol;

enum Options {
    Temperature,
    Fibonacci,
    GoldenFibonacci,
    Carol,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    loop {
        println!("Choose one of the following programs:");
        println!("1. Temperature Conversion");
        println!("2. Fibonacci Sequence");
        println!("3. Golden Ratio Fibonacci Sequence");
        println!("4. The Twelve Days of Christmas");
        println!("0. Exit");

        let mut option = String::new();

        io::stdin().read_line(&mut option)?;

        println!("===========================");

        let option = match option.trim().parse() {
            Ok(0) => break,
            Ok(1) => Options::Temperature,
            Ok(2) => Options::Fibonacci,
            Ok(3) => Options::GoldenFibonacci,
            Ok(4) => Options::Carol,
            _ => continue,
        };

        match option {
            Options::Temperature => temperature::temperature_program(),
            Options::Fibonacci => fibonacci::fibonacci_program(),
            Options::GoldenFibonacci => fibonacci::golden_ratio_fibonacci(),
            Options::Carol => carol::carol_program(),
        }
    }

    Ok(())
}
