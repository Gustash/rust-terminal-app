extern crate rand;

use rand::Rng;
use std::{io, collections::HashMap};

const VOWELS: [char;5] = ['a', 'e', 'i', 'o', 'u'];

enum EmployeeDirOptions {
    Add { department: String, name: String },
    List(String),
}

pub fn integer_list_program() {
    println!("Integer List Math");

    let mut rng = rand::thread_rng();
    let mut integer_list = Vec::new();

    for _ in 0..10 {
        let n: i32 = rng.gen_range(-50, 50);
        integer_list.push(n);
    }
    integer_list.sort();

    println!("The list is: {:?}", integer_list);

    let median_index = integer_list.len() / 2;
    let median: f64 = if integer_list.len() % 2 == 0 {
        (integer_list[median_index] + integer_list[median_index - 1]) as f64 / 2f64
    } else {
        integer_list[median_index] as f64
    };

    let mut avg: i32 = 0;
    let mut mode_map = HashMap::new();

    for &i in integer_list.iter() {
        avg += i;
        let entry = mode_map.entry(i).or_insert(0);
        *entry += 1;
    }
    avg /= integer_list.len() as i32;
    let mut mode: i32 = 0;
    let mut count: i32 = 0;
    for (&v, &c) in mode_map.iter() {
        if c > count {
            mode = v;
            count = c;
        }
    };

    println!("The mean (average) is: {}", avg);
    println!("The median is: {:.1}", median);
    println!("The mode is: {}", mode);
}

pub fn pig_latin_program() -> Result<(), io::Error> {
    println!("Pig Latin");
    println!("\nWhat sentence do you want to convert to pig latin?");

    let mut sentence = String::new();

    io::stdin().read_line(&mut sentence)?;

    let sentence = sentence.split_whitespace();
    let mut new_sentence = Vec::new();

    for word in sentence {
        let mut new_word = String::new();
        let first_char = word.chars().nth(0)
            .expect("Error getting first char");
        let is_vowel = VOWELS.contains(&first_char);
        let appendix = if is_vowel {
            String::from("-hay")
        } else {
            format!("-{}ay", first_char)
        };
        let skip = if is_vowel { 0 } else { 1 };

        for c in word.chars().skip(skip) {
            new_word.push(c);
        }
        new_word.push_str(appendix.as_str());

        new_sentence.push(new_word);
    }

    let new_sentence: String = new_sentence.join(" ");

    println!("{}", new_sentence);

    Ok(())
}

pub fn employee_program() -> Result<(), io::Error> {
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Employee Directory");
        println!("\nWhat do you want to do?");
        println!("1. Add Employee");
        println!("2. List Department");
        println!("0. Exit");


        let mut option = String::new();

        io::stdin().read_line(&mut option)?;

        let option = match option.trim().parse() {
            Ok(1) => {
                let mut department = String::new();
                let mut name = String::new();

                println!("What department?");
                io::stdin().read_line(&mut department)?;
                let department = department.trim().to_owned();

                println!("What's their name?");
                io::stdin().read_line(&mut name)?;
                let name = name.trim().to_owned();

                EmployeeDirOptions::Add { department, name }
            },
            Ok(2) => {
                let mut department = String::new();

                println!("What department?");
                io::stdin().read_line(&mut department)?;
                let department = department.trim().to_owned();

                EmployeeDirOptions::List(department)
            },
            Ok(0) => break,
            _ => {
                println!("Unknown option.");
                break;
            }
        };

        match option {
            EmployeeDirOptions::Add { department, name } => {
                let department = directory.entry(department).or_insert(Vec::new());

                department.push(name);
                department.sort();
            },
            EmployeeDirOptions::List(department) => {
                println!("{} employees:", department);

                let department = directory.entry(department).or_insert(Vec::new());

                for employee in department {
                    println!("- {}", employee);
                }
            }
        }
    }

    Ok(())
}
