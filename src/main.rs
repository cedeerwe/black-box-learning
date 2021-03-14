use rand::{Rng, RngCore};
use std::io;

// A simple guessing game. The program fixes a number between 0 and 100. The player
// will start guessing and the program answers with "Too small!" or "Too big!" until the player
// guesses correctly and the program finishes.
// Implementation based on https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
struct GuessingGame {
    min: u32,
    max: u32,
}

impl GuessingGame {
    fn new(min: u32, max: u32) -> Self {
        GuessingGame { min, max }
    }

    fn start(
        &self,
        mut writer: impl std::io::Write,
        mut rng: impl RngCore,
    ) -> Result<(), std::io::Error> {
        let number_to_guess = rng.gen_range(self.min..=self.max);

        writeln!(writer, "Hello dear friend. Guess my secret number!")?;

        loop {
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    writeln!(
                        writer,
                        "Input cannot be parsed as a positive integer, try again!"
                    )?;
                    continue;
                }
            };

            match guess.cmp(&number_to_guess) {
                std::cmp::Ordering::Equal => {
                    writeln!(writer, "You guessed correctly, the number is {}!", guess)?;
                    return Ok(());
                }
                std::cmp::Ordering::Greater => {
                    writeln!(writer, "The number {} is too big, try again!", guess)?
                }
                std::cmp::Ordering::Less => {
                    writeln!(writer, "The number {} is too small, try again!", guess)?
                }
            };
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    GuessingGame::new(0, 100).start(std::io::stdout(), rand::thread_rng())
}
