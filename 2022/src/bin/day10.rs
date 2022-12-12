
use std::str::FromStr;

use anyhow::{Result, Ok};
use itertools::Itertools;

/// Adds a character to the screen based on the clock and current regeister value
#[macro_export]
macro_rules! update_screen {
    ( $clock:ident, $pos:ident, $row:ident ) => {
   
        if (($clock % 40) as isize >= $pos - 1) && (($clock % 40)as isize <= $pos + 1) {
                $row[$clock % 40] = '#';
        }  else {
            $row[$clock % 40] = '.';
        }
    };
}

#[macro_export]
macro_rules! print_screen {
    ( $height:literal, $width:literal, $screen:ident ) => {
        for x in 0..$height {
            let mut row = String::new();
            for y in 0..$width {
                row +=  format!(" {} ", $screen[x][y].to_string()).as_str();
            }
            println!("{}", row);
        } 
   
        // if (($clock % 40) as isize >= $pos - 1) && (($clock % 40)as isize <= $pos + 1) {
        //         $row[$clock % 40] = '#';
        // }  else {
        //     $row[$clock % 40] = '.';
        // }
    };
}

/// Save the current row to the screen and creates a new row on every
/// 40th clock tick
#[macro_export]
macro_rules! new_row {
    ($clock:ident, $screen:ident, $row:ident) => {
        // checks for the clock tick to see if a new row should be added 
        if $clock % 40 == 0 {
            $screen.push($row.clone());
            $row = vec!['.'; 40];
        }
        
    };
}

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
                anyhow::bail!("Invalid input");
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
    let mut x_reg = 1;
    let mut x_values = vec![];

    // Start instruction
    x_values.push(0);

    for command in input.iter()  {
        match command {
            Command::AddX(value) => {
                // load 
                x_values.push(x_reg);
                // execute
                x_values.push(x_reg);

                x_reg += value;
            },
            Command::NOOP => {
                // load and exe
                x_values.push(x_reg);
            }
        }
        
    }

    x_values
    .iter()
    .enumerate()
    .filter(|(idx, _)| idx % 20 == 0 && idx % 40 != 0 )
    .map(|(idx, value)| value * idx as isize)
    .sum::<isize>()

    // x_values
    // .iter()
    // .enumerate()
    // .fold(0, |acc, (idx, value)| {
    //     if idx % 20 == 0 && idx % 40 != 0 {
    //         acc + (idx as isize * value)
    //     } else {
    //         acc 
    //     }
    // })

}




fn part_2_function(input: &Vec<Command>) -> Vec<Vec<char>> {
    let mut clock: usize = 0;
    let mut x_reg: isize = 1;
    let mut row = vec!['.'; 40];
    let mut screen = vec![];

    // Start instruction
    for command in input.iter()  {
        match command {
            Command::AddX(value) => {

                update_screen!(clock, x_reg, row);
                // Load
                clock += 1;
                new_row!(clock, screen, row);
                update_screen!(clock, x_reg, row);

                // execute
                clock += 1;
                x_reg += value;
                new_row!(clock, screen, row);
                update_screen!(clock, x_reg, row);
            },
            Command::NOOP => {
                // load and exe
                clock += 1;
                new_row!(clock, screen, row);
                update_screen!(clock, x_reg, row);
            }
        }
        
    }
    print_screen!(6, 40, screen);
    // for x in 0..6 {
    //     let mut row = String::new();
    //     for y in 0..40 {
    //         row +=  format!(" {} ", screen[x][y].to_string()).as_str();
    //     }
    //     println!("{}", row);
    // } 

    screen
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
    fn part_1() {

    let items = INPUT
                                .lines()
                                .map(|x| x.parse::<Command>().unwrap())
                                .collect_vec();
        assert_eq!(part_1_function(&items), 13140);
    }

    #[test]
    fn part_2() {

        let items = INPUT
                                .lines()
                                .map(|x| x.parse::<Command>().unwrap())
                                .collect_vec();
        let mut test_screen = vec![];
        let row_1 = "##..##..##..##..##..##..##..##..##..##..".chars().collect_vec();
        let row_2 = "###...###...###...###...###...###...###.".chars().collect_vec();
        let row_3 = "####....####....####....####....####....".chars().collect_vec();
        let row_4 = "#####.....#####.....#####.....#####.....".chars().collect_vec();
        let row_5 = "######......######......######......####".chars().collect_vec();
        let row_6 = "#######.......#######.......#######.....".chars().collect_vec();
        test_screen.push(row_1);
        test_screen.push(row_2);
        test_screen.push(row_3);
        test_screen.push(row_4);
        test_screen.push(row_5);
        test_screen.push(row_6);
        assert_eq!(part_2_function(&items), test_screen);
    }
}








