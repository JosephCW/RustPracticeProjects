use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);
    println!("Welcome to the guessing game!");

    loop {
        println!("Please enter your guess!");
        // Create a mutatable string variable
        let mut guess = String::new();
        // Read input to the guess var
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        // Type convert from String to u32
        // when okay continue,
        // catch all errors _, repeat the main loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);
        // Compare our guess vs the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
