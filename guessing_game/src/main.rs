use rand::Rng;
use std::{cmp::Ordering, fmt::Debug, io};

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        println!("Your guessesd: {guess}");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("error {:?}", err);
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => 
            {
                println!("You win");
                break;
            }
        }
    }
}
