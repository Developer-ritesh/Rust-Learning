use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let sec_num = rand::thread_rng().gen_range(1..=100);
    println!("Your number {sec_num}");

    loop {
        println!("Please input your guess : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number!");
                continue;
            },
        }; //shadowing

        println!("You guessed {guess}");

        match guess.cmp(&sec_num) {
            Ordering::Equal => {
                println!("You won!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too less!"),
        }
    }
}
