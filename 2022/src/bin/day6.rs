
use std::{collections::{HashMap, HashSet}, slice::Windows};
use anyhow::{Result, Ok};
use itertools::Itertools;


fn main() -> Result<()> {
    let input_file = std::fs::read_to_string("src/bin/input/input_6.txt")?;
    let items = input_file.chars().collect_vec();

    let part_1 = part_1_function(&items);

    let part_2 = part_2_function(&items);


    println!("part_1 {}", part_1);
    println!("part_2 {}", part_2);
    Ok(())
}


fn part_1_function(input: &Vec<char>) -> u32 {
    let mut length = 0;
    for (index, window) in input.windows(4).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == 4 {
            length = index + 4;
            break;
        }
    }
    length as u32
}



fn part_2_function(input: &Vec<char>) -> u32 {
    let mut length = 0;
    for (index, window) in input.windows(14).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == 14 {
            length = index + 14;
            break;
        }
    }
    length as u32
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    #[test]
    fn part_1() {

        assert_eq!(part_1_function(&INPUT.chars().collect::<Vec<char>>()), 5);
    }
    #[test]
    fn part_2() {

        assert_eq!(part_2_function(&INPUT.chars().collect::<Vec<char>>()), 23);
    }
}








