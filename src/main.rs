use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Please input your guess.");

    loop {
        let mut guess_input = String::new();

        io::stdin()
        .read_line(&mut guess_input)
        .expect("Failed to read line");

        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}