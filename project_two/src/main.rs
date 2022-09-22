extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your Guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // eprintln!("That's not a number");
                continue;
            },
            };
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too Low"),
            std::cmp::Ordering::Equal => { 
                println!("You Win");
                break;
            },
            std::cmp::Ordering::Greater => println!("Too High"),
        }
        println!("You guessed: {}", guess);
    }
}
