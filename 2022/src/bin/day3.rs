use itertools::Itertools;

fn main() {
    let items = include_str!("./input/input_3.prod").lines().collect::<Vec<&str>>();
    
    let part_1 = part_1_function(&items);

    let part_2 = part_2_function(&items);


    println!("part_1 {}", part_1);
    println!("part_2 {}", part_2);
}

fn get_priority(character: char) -> u32 {
    if character.is_uppercase() {
        // A ascii value = 65
        // A for priority = 27
        // 27 = 65 - x
        // -x = -38
        // x = 38
        character as u32 - 38
    } else {
        // lower case a ascii value = 97
        // a priority = 1
        // 1 = 97 - x
        // -x = -96 
        // x = 96
        character as u32 - 96
    }
}

fn part_1_function(input: &Vec<&str>) -> u32 {
    input
        .iter()
        .map(|x| {
            let len = x.len();
            let first_half = x.get(0..len/2).unwrap();
            let second_half = x.get((len/2)..).unwrap();

            (first_half, second_half)
        })
        .map(|(first, second)| {
        first
            .chars()
            .filter(|x| second.contains(*x))
            .nth(0)
            .unwrap()
        })
        .map(|x| get_priority(x))
        .sum::<u32>()
}


fn part_2_function(input: &Vec<&str>) -> u32 {
    input
        .iter()
        .chunks(3)
        .into_iter()
        .map(|x| {
            let mut values = x.into_iter();
            let first = values.next().unwrap();
            let second = values.next().unwrap();
            let third = values.next().unwrap();
            first.chars().filter(|x| second.contains(*x) && third.contains(*x)).nth(0).unwrap()

        })
        .map(|x| get_priority(x))
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1() {

        assert_eq!(part_1_function(&INPUT.lines().collect::<Vec<&str>>()), 157);
    }
    #[test]
    fn part_2() {

        assert_eq!(part_2_function(&INPUT.lines().collect::<Vec<&str>>()), 70);
    }
}








