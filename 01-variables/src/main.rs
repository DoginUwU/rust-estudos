use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..=10);
    
    loop {
        println!("Insert your number:");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read... :c");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Number too big"),
            Ordering::Less => println!("Number too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
