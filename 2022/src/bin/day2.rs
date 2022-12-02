use std::{str::FromStr, error, char::ParseCharError};

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
struct Play(Move, Outcome);

impl FromStr for Play {
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

        Ok(Play(first, second))
    }
}

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SISSORS_SCORE: i32 = 3;


fn main() {
    let lines = include_str!("./input/input_2.test")
                        .lines()
                        .map(|x| Play::from_str(x).unwrap())
                        .map(|x| {
                            match x.0 {
                                Move::ROCK => {
                                    match x.1 {
                                        Outcome::DRAW => 3 + ROCK_SCORE,
                                        Outcome::WIN => 6 + PAPER_SCORE,
                                        Outcome::LOSE => 0 + SISSORS_SCORE,
                                    }
                                },
                                Move::PAPER => {
                                    match x.1 {
                                        Outcome::LOSE => 0 + ROCK_SCORE,
                                        Outcome::DRAW => 3 + PAPER_SCORE,
                                        Outcome::WIN => 6  + SISSORS_SCORE,
                                    }

                                }
                                Move::SISSORS => {
                                    match x.1 {
                                        Outcome::WIN => 6 + ROCK_SCORE,
                                        Outcome::LOSE => 0 + PAPER_SCORE,
                                        Outcome::DRAW => 3 + SISSORS_SCORE,
                                    }
                                }

                            }
                        }).sum::<i32>();

    println!("{:?}", lines)
}