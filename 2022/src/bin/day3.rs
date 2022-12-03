use itertools::Itertools;

fn main() {
    let items = include_str!("./input/input_3.prod").lines().collect::<Vec<&str>>();
    
    let part_1 = items
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
                    .sum::<u32>();

    let part_2 = items
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
                    .sum::<u32>();


    println!("part_1 {:?}", part_1);
    println!("part_2 {:?}", part_2);
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
