use rand::{Rng, RngCore};
use std::io;

// A simple rock paper scissors game. The program prompts the user for input shape (rock / paper / scissors)
// and plays something. In the first iteration, the program will choose its shape randomly/
// In the future, we may add strategies and certain players to improve the blackbox experience.
// As, so far, the computer draws randomly, we require no multi-game state, so the entire "game" struct consists of only one round.

struct RockPaperScissorsGame<W: std::io::Write> {
  computer_shape: Shapes,
  writer: W
}

#[derive(Copy, Clone)]
enum Shapes {
  Rock,
  Paper,
  Scissors
}

impl<W: std::io::Write> RockPaperScissorsGame<W>{
  fn new(rng: impl RngCore, writer: W) -> Self {
    let mut rng = rand::thread_rng();
    RockPaperScissorsGame {
      computer_shape: [Shapes::Rock, Shapes::Paper, Shapes::Scissors][rng.gen_range(0..2)], // todo: implement in a cleaner way
      writer
    }
  }
}

// A simple guessing game. The program fixes a number between 0 and 100. The player
// will start guessing and the program answers with "Too small!" or "Too big!" until the player
// guesses correctly and the program finishes.
// Implementation based on https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
struct GuessingGame<W: std::io::Write> {
    number_of_guesses: Guesses,
    number_to_guess: u32,
    writer: W,
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

impl<W: std::io::Write> GuessingGame<W> {
    fn new(max: u32, mut rng: impl RngCore, writer: W) -> Self {
        GuessingGame {
            number_of_guesses: Guesses(0),
            number_to_guess: rng.gen_range(0..=max),
            writer,
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

    fn user_round(&mut self) -> Result<bool, std::io::Error> {
        match self.evaluate_user_input() {
            GuessResult::ParseError => {
                writeln!(
                    self.writer,
                    "Input cannot be parsed as a positive integer, try again!"
                )?;
                Ok(true)
            }
            GuessResult::Equal(guess) => {
                writeln!(
                    self.writer,
                    "You guessed correctly, the number is {}! It took you {}.",
                    guess, self.number_of_guesses,
                )?;
                Ok(false)
            }
            GuessResult::Greater(guess) => {
                writeln!(self.writer, "The number {} is too big, try again!", guess)?;
                Ok(true)
            }
            GuessResult::Less(guess) => {
                writeln!(self.writer, "The number {} is too small, try again!", guess)?;
                Ok(true)
            }
        }
    }

    fn start(&mut self) -> Result<(), std::io::Error> {
        writeln!(self.writer, "Hello dear friend. Guess my secret number!")?;
        self.number_of_guesses = Guesses(0);

        while self.user_round()? {}
        Ok(())
    }
}

fn main() -> Result<(), std::io::Error> {
    GuessingGame::new(100, rand::thread_rng(), std::io::stdout()).start()
}
