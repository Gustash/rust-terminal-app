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

pub fn carol_program() {
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
