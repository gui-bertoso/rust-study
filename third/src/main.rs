use rand::prelude::*;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Hi :)");

    println!("Lets play a game? (Y/N)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get read line");



    let randomized_number = rand::rng().random_range(1..=100);
    println!("Ok, I gonna try think the same number of you, write a number!");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to get read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&randomized_number) {
            Ordering::Less => println!("Too small, try again"),
            Ordering::Greater => println!("Too big, try again"),
            Ordering::Equal => {
                println!("You win !!!!, good bye");
                break
            },
        }
    }
}
