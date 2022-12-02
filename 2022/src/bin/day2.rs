use std::{str::FromStr, char::ParseCharError};

#[derive(Debug)]
enum Move {
    ROCK,
    PAPER,
    SISSORS
}

#[derive(Debug)]
enum Outcome {
    WIN,
    LOSE,
    DRAW,
}

#[derive(Debug)]
struct GamePart1(Move, Move);

impl FromStr for GamePart1   {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().chars();

        let first = match iter.next().unwrap() {
            'A' => Move::ROCK,
            'B' => Move::PAPER,
            'C' => Move::SISSORS,
            _ => panic!("Undefined"),
        };

        iter.next(); // space between moves

        let second = match iter.next().unwrap() {
            'X' => Move::ROCK,
            'Y' => Move::PAPER,
            'Z' => Move::SISSORS,
            _ => panic!("Undefined"),
        };

        Ok(GamePart1(first, second))
    }
}

impl GamePart1 {
   fn get_score(&self) -> i32 {
        match self.0 {
            Move::ROCK => {
                match self.1 {
                    Move::ROCK => 3 + ROCK_SCORE,
                    Move::PAPER => 6 + PAPER_SCORE,
                    Move::SISSORS => 0 + SISSORS_SCORE,
                }
            },
            Move::PAPER => {
                match self.1 {
                    Move::ROCK => 0 + ROCK_SCORE,
                    Move::PAPER => 3 + PAPER_SCORE,
                    Move::SISSORS => 6  + SISSORS_SCORE,
                }
            }
            Move::SISSORS => {
                match self.1 {
                    Move::ROCK => 6 + ROCK_SCORE,
                    Move::PAPER => 0 + PAPER_SCORE,
                    Move::SISSORS => 3 + SISSORS_SCORE,
                }
            }
        }
   }
}

#[derive(Debug)]
struct GamePart2(Move, Outcome);

impl FromStr for GamePart2 {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().chars();

        let first = match iter.next().unwrap() {
            'A' => Move::ROCK,
            'B' => Move::PAPER,
            'C' => Move::SISSORS,
            _ => panic!("Undefined"),
        };
        iter.next(); // space between moves

        let second = match iter.next().unwrap() {
            'X' => Outcome::LOSE,
            'Y' => Outcome::DRAW,
            'Z' => Outcome::WIN,
            _ => panic!("Undefined"),
        };

        Ok(GamePart2(first, second))
    }
}

impl GamePart2 {
   fn get_score(&self) -> i32 {
        match self.0 {
            Move::ROCK => {
                match self.1 {
                    Outcome::DRAW => 3 + ROCK_SCORE,
                    Outcome::WIN => 6 + PAPER_SCORE,
                    Outcome::LOSE => 0 + SISSORS_SCORE,
                }
            },
            Move::PAPER => {
                match self.1 {
                    Outcome::LOSE => 0 + ROCK_SCORE,
                    Outcome::DRAW => 3 + PAPER_SCORE,
                    Outcome::WIN => 6  + SISSORS_SCORE,
                }
            }
            Move::SISSORS => {
                match self.1 {
                    Outcome::WIN => 6 + ROCK_SCORE,
                    Outcome::LOSE => 0 + PAPER_SCORE,
                    Outcome::DRAW => 3 + SISSORS_SCORE,
                }
            }
        }
   }
}

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SISSORS_SCORE: i32 = 3;


fn main() {
    let  part_1 = include_str!("./input/input_2.prod")
                        .lines()
                        .map(|x| GamePart1::from_str(x).unwrap())
                        .map(|x| x.get_score()).sum::<i32>();

    let part_2 = include_str!("./input/input_2.prod")
                        .lines()
                        .map(|x| GamePart2::from_str(x).unwrap())
                        .map(|x| x.get_score()).sum::<i32>();

    println!("part 1: {:?}", part_1);
    println!("part 2: {:?}", part_2);
}