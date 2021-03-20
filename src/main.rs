use rand::{Rng, RngCore};
use std::io;

// A simple rock paper scissors game. The program prompts the user for input shape (rock / paper / scissors)
// and plays something. In the first iteration, the program will choose its shape randomly/
// In the future, we may add strategies and certain players to improve the blackbox experience.
// As, so far, the computer draws randomly, we require no multi-game state, so the entire "game" struct consists of only one round.
struct RockPaperScissorsGame<W: std::io::Write> {
  computer_shape: ShapesInput,
  writer: W
}

// probably dont need to return Result<String, ParseErorr> here, as all input is string
// will later want to use this in read_and_parse_u32
fn read_string() -> String {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  if input.ends_with("\n") { input.pop(); }
  input
}

#[derive(Copy, Clone, Debug)]
enum ShapesInput {
  Rock,
  Paper,
  Scissors,
  Unrecognized,
}

#[derive(Debug, Eq, PartialEq)]
enum RoundState {
  Draw,
  Win,
  Lose
}

impl std::fmt::Display for ShapesInput {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
  }
}

impl std::fmt::Display for RoundState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f, 
      "The round was {:}.",
      if self == &RoundState::Lose { "lost" } else if self == &RoundState::Draw { "a draw" } else { "won" } )
  }
}

impl<W: std::io::Write> RockPaperScissorsGame<W>{
  pub fn new(mut rng: impl RngCore, writer: W) -> Self {
    RockPaperScissorsGame {
      computer_shape: [ShapesInput::Rock, ShapesInput::Paper, ShapesInput::Scissors][rng.gen_range(0..2)], // todo: implement in a cleaner way, extract from constructor
      writer
    }
  }

  fn evaluate_user_input(&mut self) -> ShapesInput {
    match read_string().to_lowercase().as_str() as &str {
      "rock" => ShapesInput::Rock,
      "paper" => ShapesInput::Paper,
      "scissors" => ShapesInput::Scissors,
      _ => ShapesInput::Unrecognized
    }
  }

  fn play_round(&mut self) -> Result<bool, std::io::Error> {
    let input = self.evaluate_user_input();
    match input {
      ShapesInput::Unrecognized => {
        writeln!(
          self.writer,
          "Input cannot be parsed as a a rock / paper / scissors, try again!"
        )?;
        Ok(true)
      },
      shape_played => {
        let result = self.evaluate_game_as_left(shape_played, self.computer_shape);
        writeln!(
          self.writer,
          "You played {}, the computer played {}. {}", 
          shape_played, self.computer_shape, result
        )?;
        Ok(false)
      }
    }
  }

  fn transform_shape_to_int(&mut self, shape: ShapesInput) -> Option<i32>  {
    match shape {
      ShapesInput::Rock => Some(0),
      ShapesInput::Paper => Some(1),
      ShapesInput::Scissors => Some(2),
      _ => None
    }
  }

  fn evaluate_game_as_left(&mut self, left: ShapesInput, right: ShapesInput) -> RoundState {
    let left_int = self.transform_shape_to_int(left).unwrap();
    let right_int = self.transform_shape_to_int(right).unwrap();

    if left_int == right_int {
      RoundState::Draw
    } else if (left_int - right_int) % 3 == 1 {
      RoundState::Win
    } else {
      RoundState::Lose
    }
  }

  fn start(&mut self) -> Result<(), std::io::Error> {
    writeln!(self.writer, "Hello dear friend. Rock/paper/scissors?")?;

    while self.play_round()? {}
    Ok(())
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
    // GuessingGame::new(100, rand::thread_rng(), std::io::stdout()).start()
    RockPaperScissorsGame::new(rand::thread_rng(), std::io::stdout()).start()
}
