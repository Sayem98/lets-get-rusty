use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    println!("Guessing game.");
    println!("Enter a number");

    let mut guess = String::new();
    
    io::stdin()
    .read_line( &mut guess)
    .expect("Failed te read line");

    let rand_number = rand::thread_rng().gen_range(1, 100);
    let guess: u32 = guess.trim().parse().expect("Please type a number");
    match guess.cmp(&rand_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("Equal"),
        Ordering::Greater => println!("Too high")
    }

    
}
