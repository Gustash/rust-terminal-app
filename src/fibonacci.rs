use std::io;

const GOLDEN_RATIO: f64 = 1.618034;

pub fn fibonacci_program() {
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

pub fn golden_ratio_fibonacci() {
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
