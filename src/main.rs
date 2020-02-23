use std::io;

enum Options {
    Temperature,
    Fibonacci,
    GoldenFibonacci,
    Carol,
}

enum Temperature {
    Fahrenheit,
    Celsius,
}

const GOLDEN_RATIO: f64 = 1.618034;
const CAROL_PARTS: [&str;12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "four calling birds",
    "five gold rings",
    "six geese a laying",
    "seven swans a swimming",
    "eight maids a milking",
    "nine drummers drumming",
    "ten pipers piping",
    "eleven ladies dancing",
    "twelve Lords a leaping",
];

fn main() {
    loop {
        println!("Choose one of the following programs:");
        println!("1. Temperature Conversion");
        println!("2. Fibonacci Sequence");
        println!("3. Golden Ratio Fibonacci Sequence");
        println!("4. The Twelve Days of Christmas");
        println!("0. Exit");

        let mut option = String::new();

        io::stdin().read_line(&mut option)
            .expect("Failed to read option.");

        println!("===========================");

        let option = match option.trim().parse() {
            Ok(0) => break,
            Ok(1) => Options::Temperature,
            Ok(2) => Options::Fibonacci,
            Ok(3) => Options::GoldenFibonacci,
            Ok(4) => Options::Carol,
            Ok(_) => continue,
            Err(_) => continue,
        };

        match option {
            Options::Temperature => temperature_program(),
            Options::Fibonacci => fibonacci_program(),
            Options::GoldenFibonacci => golden_ratio_fibonacci(),
            Options::Carol => carol_program(),
        }
    }
}

fn temperature_program() {
    println!("Temperature Conversion");
    println!("\nDo you wish to convert Celsius or Fahrenheit? (c/f)");

    let mut unit = String::new();

    io::stdin().read_line(&mut unit)
        .expect("Failed to read unit.");

    let unit = match unit.trim()
        .to_lowercase()
        .chars()
        .nth(0)
        .unwrap() {
        'c' => Temperature::Celsius,
        'f' => Temperature::Fahrenheit,
        _ => {
            println!("Wrong unit.");
            return;
        },
    };

    println!("What's the temperature?");

    let mut temperature = String::new();

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read temperature.");

    let temperature: f64 = match temperature.trim().parse::<f64>() {
        Ok(num) => {
            match unit {
                Temperature::Celsius => num * (9. / 5.) + 32.,
                Temperature::Fahrenheit => (num - 32.) * (5. / 9.),
            }
        },
        Err(_) => {
            println!("The provided temperature was not a number!");
            return;
        }
    };

    println!(
        "That is {:.1} {}!",
        temperature,
        match unit {
            // These need to be reversed because the resulting temperature is of the opposite unit.
            Temperature::Celsius => "fahrenheit",
            Temperature::Fahrenheit => "celsius",
        }
    );
}

fn fibonacci_program() {
    println!("Fionacci Sequence");
    println!("\nWhat nth number do you want?");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read n.");

    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("The selected n wasn't a number!");
            return;
        }
    };

    let mut x: u64 = 0;
    let mut y: u64 = 1;

    for _ in 0..n-1 {
        let z: u64 = y;
        y += x;
        x = z;
    }

    println!("The {}th element of fibonacci is: {}", n, y);
}

fn golden_ratio_fibonacci() {
    println!("Golden Ratio Fionacci Sequence");
    println!("Warning: This algorithm goes higher than the regular one,\nbut it's not as accurate.");
    println!("\nWhat nth number do you want?");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Failed to read n.");

    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("The selected n wasn't a number!");
            return;
        }
    };

    let fibonnaci = (GOLDEN_RATIO.powi(n) - (1. - GOLDEN_RATIO).powi(n)) / 5f64.powf(0.5);

    println!("The {}th element of fibonacci is: {:.0}", n, fibonnaci);
}

fn carol_program() {
    for day in 1..13 {
        println!(
            "On the {}{} day of Christmas my true love sent to me",
            day,
            match day {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            },
        );

        for part in (0..day).rev() {
            println!(
                "{}{}{}",
                if part == 0 && day > 1 { "and " } else { "" },
                CAROL_PARTS[part],
                if part > 0 { "," } else { "" },
            );
        }

        println!();
    }
}
