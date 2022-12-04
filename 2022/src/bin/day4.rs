
use std::{str::FromStr};

#[derive(Debug)]
struct Task {
    group_1: Range,
    group_2: Range,
}

impl Task {
    fn fully_contains(&self) -> bool {
        self.group_1.contains(&self.group_2) || self.group_2.contains(&self.group_1)
   }

    fn partially_contains(&self) -> bool {
        let g1 = &self.group_1;
        let g2 = &self.group_2;
        let range_1 = g1.start..=g1.end;
        let range_2 = g2.start..=g2.end;

        range_1.contains(&g2.start) || range_1.contains(&g2.end) ||
        range_2.contains(&g1.start) || range_2.contains(&g1.end)
   }
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
   fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
   } 
}

impl FromStr for Range {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 2-4,6-8
        let (start, end) = s.split_once("-").expect("Shoule be like this");


        Ok(Range{start: start.parse().unwrap(), end: end.parse().unwrap()})

    }
}

fn main() {
    let items = include_str!("./input/input_4.txt").lines().collect::<Vec<&str>>();
    
    let part_1 = part_1_function(&items);

    let part_2 = part_2_function(&items);


    println!("part_1 {}", part_1);
    println!("part_2 {}", part_2);
}


fn part_1_function(input: &Vec<&str>) -> u32 {
    input
        .iter()
        .map(|&x| {
            let (left, right) = x.split_once(',').expect("Should be like this");
            Task{group_1: left.parse::<Range>().unwrap(), group_2: right.parse::<Range>().unwrap()}
        })
        .filter(|x| x.fully_contains() )
        .count() as u32
}



fn part_2_function(input: &Vec<&str>) -> u32 {
    input
        .iter()
        .map(|&x| {
            let (left, right) = x.split_once(',').expect("Should be like this");
            Task{group_1: left.parse::<Range>().unwrap(), group_2: right.parse::<Range>().unwrap()}
        })
        .filter(|x| x.partially_contains() )
        .count() as u32
    
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1() {

        assert_eq!(part_1_function(&INPUT.lines().collect::<Vec<&str>>()), 2);
    }
    #[test]
    fn part_2() {

        assert_eq!(part_2_function(&INPUT.lines().collect::<Vec<&str>>()), 4);
    }
}








