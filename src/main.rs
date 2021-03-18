use rand::{Rng, RngCore};
use std::io;

// A simple guessing game. The program fixes a number between 0 and 100. The player
// will start guessing and the program answers with "Too small!" or "Too big!" until the player
// guesses correctly and the program finishes.
// Implementation based on https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
struct GuessingGame {
    number_of_guesses: Guesses,
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

enum GuessResult {
    ParseError,
    Less(u32),
    Greater(u32),
    Equal(u32),
}

struct Guesses(usize);

impl std::fmt::Display for Guesses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.0,
            if self.0 == 1 { "guess" } else { "guesses" }
        )
    }
}

impl GuessingGame {
    fn new(max: u32, mut rng: impl RngCore) -> Self {
        GuessingGame {
            number_of_guesses: Guesses(0),
            number_to_guess: rng.gen_range(0..=max),
            finished: true,
        }
    }

    fn make_guess(&mut self, guess: u32) -> GuessResult {
        self.number_of_guesses = Guesses(self.number_of_guesses.0 + 1);
        match guess.cmp(&self.number_to_guess) {
            std::cmp::Ordering::Equal => GuessResult::Equal(guess),
            std::cmp::Ordering::Greater => GuessResult::Greater(guess),
            std::cmp::Ordering::Less => GuessResult::Less(guess),
        }
    }

    fn evaluate_user_input(&mut self) -> GuessResult {
        match read_and_parse_u32() {
            Ok(num) => self.make_guess(num),
            Err(_) => GuessResult::ParseError,
        }
    }

    fn user_round<W: std::io::Write>(&mut self, mut writer: W) -> Result<W, std::io::Error> {
        match self.evaluate_user_input() {
            GuessResult::ParseError => {
                writeln!(
                    writer,
                    "Input cannot be parsed as a positive integer, try again!"
                )?;
            }
            GuessResult::Equal(guess) => {
                writeln!(
                    writer,
                    "You guessed correctly, the number is {}! It took you {}.",
                    guess, self.number_of_guesses,
                )?;
                self.finished = true;
            }
            GuessResult::Greater(guess) => {
                writeln!(writer, "The number {} is too big, try again!", guess)?;
            }
            GuessResult::Less(guess) => {
                writeln!(writer, "The number {} is too small, try again!", guess)?;
            }
        }
        Ok(writer)
    }

    fn start(&mut self, mut writer: impl std::io::Write) -> Result<(), std::io::Error> {
        writeln!(writer, "Hello dear friend. Guess my secret number!")?;
        self.finished = false;
        self.number_of_guesses = Guesses(0);

        while !self.finished {
            writer = self.user_round(writer)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    GuessingGame::new(100, rand::thread_rng()).start(std::io::stdout())
}
