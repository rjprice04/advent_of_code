
use std::{collections::{HashMap, HashSet}, slice::Windows, str::FromStr, f32::consts::E, vec};
use anyhow::{Result, Ok};
use itertools::Itertools;

#[derive(Debug)]
enum Command {
    AddX(isize),
    NOOP
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            return Ok(Command::NOOP);
        } else {
            if let Some((_, num)) = s.split_once(" ") {

                return Ok(Command::AddX(num.parse::<isize>().unwrap()));
            } else {
                panic!()
            }
        }
    }
}


fn main() -> Result<()> {
    let input_file = std::fs::read_to_string("src/bin/input/input_10.txt")?;
    let items = input_file
                                .lines()
                                .map(|line| line.parse::<Command>().unwrap())
                                .collect_vec();

    let part_1 = part_1_function(&items);
    let _part_2 = part_2_function(&items);

    // let part_2 = part_2_function(&items);


    println!("part_1 {}", part_1);
    // println!("part_2 {}", part_2);
    Ok(())
}


fn part_1_function(input: &Vec<Command>) -> isize {
    let mut clock = 0;
    let mut x_reg = 1;
    let mut x_values = vec![];

    // Start instruction
    x_values.push(-1);

    for command in input.iter()  {
        match command {
            Command::AddX(value) => {
                // load 
                clock += 1;
                x_values.push(x_reg);
                // execute
                clock += 1;
                x_values.push(x_reg);

                x_reg += value;
            },
            Command::NOOP => {
                // load and exe
                clock += 1;
                x_values.push(x_reg);
            }
        }
        
    }
    let mut signal = 0;
    for (idx, value) in x_values.iter().enumerate() {
        if idx == 20 || idx == 60 || idx == 100 || idx == 140 || idx == 180 || idx == 220 {
            let signal_val = idx as isize * value;
            signal += signal_val
        }
    }

    signal
}




fn part_2_function(input: &Vec<Command>) -> isize {
    let mut clock: usize = 0;
    let mut x_reg: isize = 1;
    let mut row = vec!['.'; 40];
    let mut screen = vec![];

    // Start instruction
    for command in input.iter()  {
        match command {
            Command::AddX(value) => {
                if ((clock % 40) as isize == x_reg - 1) || ((clock % 40) as isize == x_reg)  || ((clock % 40)as isize == x_reg + 1) {
                    row[clock % 40] = '#';
                }  else {
                    row[clock % 40] = '.';
                }
                // load 
                clock += 1;
                if clock % 40 == 0 {
                    screen.push(row.clone());
                    row = vec!['.'; 40];
                }
                if ((clock % 40) as isize == x_reg - 1) || ((clock % 40) as isize == x_reg)  || ((clock % 40)as isize == x_reg + 1) {
                    row[clock % 40] = '#';
                }  else {
                    row[clock % 40] = '.';
                }

                // execute
                clock += 1;
                x_reg += value;
                if clock % 40 == 0 {
                    screen.push(row.clone());
                    row = vec!['.'; 40];
                }
                if ((clock % 40) as isize == x_reg - 1) || ((clock % 40) as isize == x_reg)  || ((clock % 40)as isize == x_reg + 1) {
                    row[clock % 40] = '#';
                }  else {
                    row[clock % 40] = '.';
                }

            },
            Command::NOOP => {
                // load and exe
                clock += 1;
                if clock % 40 == 0 {
                    screen.push(row.clone());
                    row = vec!['.'; 40];
                }
                if ((clock % 40) as isize == x_reg - 1) || ((clock % 40) as isize == x_reg)  || ((clock % 40)as isize == x_reg + 1) {
                    row[clock % 40] = '#';
                }  else {
                    row[clock % 40] = '.';
                }
            }
        }
        
    }
    for x in 0..6 {
        let mut row = String::new();
        for y in 0..40 {
            row +=  format!(" {} ", screen[x][y].to_string()).as_str();
        }
        println!("{}", row);
    } 

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    #[ignore = "done"]
    fn part_1() {

    let items = INPUT
                                .lines()
                                .map(|x| x.parse::<Command>().unwrap())
                                .collect_vec();
        assert_eq!(part_1_function(&items), 420);
    }

    #[test]
    fn part_2() {

        let items = INPUT
                                .lines()
                                .map(|x| x.parse::<Command>().unwrap())
                                .collect_vec();
        assert_eq!(part_2_function(&items), 0);
    }
}







