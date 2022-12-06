
use std::collections::HashMap;
use anyhow::{Result, Ok};
use itertools::Itertools;

#[derive(Debug)]
struct Stack {
    stacks: HashMap<u16, Vec<char>>,
}
impl Stack {
    fn new() -> Self {
        let mut stack_map:HashMap<u16, Vec<char>> = HashMap::new();
        stack_map.insert(1, "mjcbfrlh".chars().collect_vec());
        stack_map.insert(2, "zcd".chars().collect_vec());
        stack_map.insert(3, "hjfcngw".chars().collect_vec());
        stack_map.insert(4, "pjdmtsb".chars().collect_vec());
        stack_map.insert(5, "ncdrj".chars().collect_vec());
        stack_map.insert(6, "wldqpjgz".chars().collect_vec());
        stack_map.insert(7, "pztfrh".chars().collect_vec());
        stack_map.insert(8, "lvmg".chars().collect_vec());
        stack_map.insert(9, "cbgpfqrj".chars().collect_vec());

        for (key, value) in stack_map.iter() {
            println!("Key: {:?} Stack: {:?}", key, value);
        }

        Self { stacks: stack_map }
    }

    fn move_crates(&mut self, from: u16, to: u16, count: u16 ) {
        let mut temp_stack = self.get_temp_stack(from, count);

        self.stacks.get_mut(&to).unwrap().append(&mut temp_stack);

        for (key, value) in self.stacks.iter()  {
           println!("Key: {:?} Stack: {:?}", key, value);
        }
    }

    fn get_temp_stack(&mut self, from: u16, count: u16) -> Vec<char> {
        let stack = self.stacks.get_mut(&from).unwrap();
        let mut temp_stack = Vec::new();
        for _ in 0..count {
            let crate_to_move = stack.pop().unwrap();
            temp_stack.push(crate_to_move);
        }
        // Part 1
        // temp_stack

        // part 2
        temp_stack.reverse();
        temp_stack

    }

    fn show_top(&self) -> String {
        let values = &self.stacks;
        let mut str = String::new();
        for i in 0..values.len(){
            let key = (i + 1) as u16;
            let last = values.get(&key).unwrap().last().unwrap();
            let curr = format!("{}", last);
            str += &curr;
        }

        str
    }

}


fn main() -> Result<()> {
    let input_file = std::fs::read_to_string("src/bin/input/input_5.txt")?;
    let items = input_file.lines().collect_vec();

    let part_1 = part_1_function(&items);

    let part_2 = part_2_function(&items);


    println!("part_1 {}", part_1);
    println!("part_2 {}", part_2);
    Ok(())
}


fn part_1_function(input: &Vec<&str>) -> String {
    let mut stacks = Stack::new();

    for &row in input {
        println!("{}", &row);
        let mut iter = row.split_whitespace();
        iter.next();
        let count = iter.next().unwrap().parse::<u16>().unwrap();
        iter.next();
        let from = iter.next().unwrap().parse::<u16>().unwrap();
        iter.next();
        let to = iter.next().unwrap().parse::<u16>().unwrap();

        println!("from: {} to: {} count: {}", from, to, count);
        stacks.move_crates(from, to, count);
    }
    stacks.show_top()
}



fn part_2_function(_input: &Vec<&str>) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn part_1() {

        assert_eq!(part_1_function(&INPUT.lines().collect::<Vec<&str>>()), "cmz");
    }
    #[test]
    fn part_2() {

        assert_eq!(part_2_function(&INPUT.lines().collect::<Vec<&str>>()), 4);
    }
}








