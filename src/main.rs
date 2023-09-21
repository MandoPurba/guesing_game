use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guesing the number!");
    println!("Range number 1 - 100");

    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut count = 1;

    loop {
        println!("Please input your number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed number {}", guess);
        // println!("Secret number {}", secret_number);

        match guess.cmp(&secret_number) {
            // Arm
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
        count += 1;
    }

    println!("Total Guesing:  {}", count)
}
