use std::{cmp::Ordering, io, io::Error, io::ErrorKind};
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, Error> {
        if value < 1 || value > 100 {
            return Err(Error::new(ErrorKind::Other, "Guess value must be between 1 and 100."));
        }

        Ok(Guess { value })
    }

    // We use a getter so that value can remain private and 
    // therefor can not be set explicitly. Setting it explicitly
    // without using the ::new function would bypass the validation.Guess
    // which we don't want to allow.
    pub fn value(&self) -> i32 {
        self.value
    }

}

fn main() {
    println!("Guess the number between 1 an 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
        // this is a comment

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(value) => value,
                Err(err) =>  {
                    println!("{}", err);
                    continue;
                }
            },
            Err(_) => {
              println!("Please enter a valid number.");
              continue;
            }

        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
