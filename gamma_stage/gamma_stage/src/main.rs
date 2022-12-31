use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guessing numbers");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("Answer: {}", secret);

    loop {
        println!("Guessing: ");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
                .expect("Unreadable string");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
                
    
        println!("Your number: {}", guess);
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Lower"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Good!");
                break;
            },
        }
    }
}