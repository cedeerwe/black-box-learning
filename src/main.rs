use rand::{Rng, RngCore};
use std::io;

// A simple guessing game. The program fixes a number between 0 and 100. The player
// will start guessing and the program answers with "Too small!" or "Too big!" until the player
// guesses correctly and the program finishes.
// Implementation based on https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
struct GuessingGame {
    number_of_guesses: usize,
    number_to_guess: u32,
    finished: bool,
}

fn read_and_parse_u32() -> Result<u32, std::num::ParseIntError> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess.trim().parse::<u32>()
}

impl GuessingGame {
    fn new(max: u32, mut rng: impl RngCore) -> Self {
        GuessingGame {
            number_of_guesses: 0,
            number_to_guess: rng.gen_range(0..=max),
            finished: true,
        }
    }

    fn make_guess(&mut self, guess: u32) -> std::cmp::Ordering {
        self.number_of_guesses += 1;
        guess.cmp(&self.number_to_guess)
    }

    fn start(&mut self, mut writer: impl std::io::Write) -> Result<(), std::io::Error> {
        writeln!(writer, "Hello dear friend. Guess my secret number!")?;
        self.finished = false;
        self.number_of_guesses = 0;

        while !self.finished {
            let guess = match read_and_parse_u32() {
                Ok(num) => num,
                Err(_) => {
                    writeln!(
                        writer,
                        "Input cannot be parsed as a positive integer, try again!"
                    )?;
                    continue;
                }
            };

            match self.make_guess(guess) {
                std::cmp::Ordering::Equal => {
                    writeln!(
                        writer,
                        "You guessed correctly, the number is {}! It took you {} {}.",
                        guess,
                        self.number_of_guesses,
                        if self.number_of_guesses == 1 {
                            "guess"
                        } else {
                            "guesses"
                        }
                    )?;
                    self.finished = true;
                }
                std::cmp::Ordering::Greater => {
                    writeln!(writer, "The number {} is too big, try again!", guess)?
                }
                std::cmp::Ordering::Less => {
                    writeln!(writer, "The number {} is too small, try again!", guess)?
                }
            };
        }
        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    GuessingGame::new(100, rand::thread_rng()).start(std::io::stdout())
}
