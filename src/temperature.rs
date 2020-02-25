use std::io;

enum Temperature {
    Fahrenheit,
    Celsius,
}

pub fn temperature_program() {
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
