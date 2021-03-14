use rand::Rng;
use std::io;

// A simple guessing game. The program fixes a number between 0 and 100. The player
// will start guessing and the program answers with "Too small!" or "Too big!" until the player
// guesses correctly and the program finishes.
// Implementation based on https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
fn main() {
    let mut rng = rand::thread_rng();
    let number_to_guess = rng.gen_range(0..=100);

    println!("Hello dear friend. Guess my secret number!");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input cannot be parsed as a positive integer, try again!");
                continue;
            }
        };

        match guess.cmp(&number_to_guess) {
            std::cmp::Ordering::Equal => {
                println!("You guessed correctly, the number is {}!", guess);
                break;
            }
            std::cmp::Ordering::Greater => println!("The number {} is too big, try again!", guess),
            std::cmp::Ordering::Less => println!("The number {} is too small, try again!", guess),
        };
    }
}
