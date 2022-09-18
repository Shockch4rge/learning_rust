use std::{cmp::Ordering, io};

use rand::Rng;

pub struct Guess {
    answer: i32,
    input: i32,
}

impl Guess {
    pub fn new(input: String) -> Self {
        let input: i32 = input.trim().parse().expect("Value needs to be a number!");

        if input < 1 || input > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", input);
        }

        Self {
            answer: rand::thread_rng().gen_range(1..=100),
            input,
        }
    }

    pub fn validate(&self) -> Ordering {
        self.input.cmp(&self.answer)
    }
}

pub fn guessing_game() {
    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess = Guess::new(input);

        match guess.validate() {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
            Ordering::Greater => println!("Too large!"),
        }
    }
}
