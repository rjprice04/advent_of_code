use std::error;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
// Better solution
fn main() -> Result<()> {
   let mut max = include_str!("./input/input1.txt")
   .split("\n\n")
   .map(|x| {
        return x
            .lines()
            .flat_map(str::parse::<usize>)
            .sum::<usize>();
   })
   .collect::<Vec<usize>>();

   max.sort_by(|x, y| y.cmp(&x));

   println!("part 1: {:?}", max.get(0));

   println!("part 2: {:?}", max.iter().take(3).sum::<usize>());

   Ok(())
}

// use std::{fs::{self, File}, error, io::{self, BufReader, BufRead}, vec, cmp::Ordering, f32::consts::E};
// // Change the alias to `Box<error::Error>`.
// type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// #[derive(Debug, Copy, Clone)]
// struct Elf {
//     id: i32,
//     calories: i32,
// }

// impl Elf {
//    fn new(id: i32, calories: i32) -> Self {
//         Elf { id , calories   }
//    }
// }
// fn main() -> Result<()>{
//     let path = "input.txt";
//     let file = File::open(path).unwrap();
//     let all_calories: Vec<i32> = BufReader::new(file)
//                                 .lines()
//                                 .map(|x| x.unwrap().parse::<i32>().unwrap_or_default()).collect();
//     let mut all_elfs = Vec::new();
//     let mut total_calories = 0;
//     let mut id = 1;
//     for i in 0..all_calories.len() {
//         let calorie = all_calories.get(i).unwrap().to_owned();

//         if calorie == 0 {
//             id += 1;
//             all_elfs.push(Elf::new(id, total_calories));
//             total_calories = 0;
//         } else {
//             total_calories += calorie;
//         }
//     }
//     all_elfs.sort_by(|x, y| x.calories.cmp(&y.calories));

//     // Part 1 for the most caloriess
//     let elf = all_elfs.iter().last();
//     println!("{:?}", elf);
//     // Elf { id: 205, calories: 69206 }

//     let len = all_elfs.len();
//     let mut total = all_elfs.get(len-3).unwrap().calories;
//     total += all_elfs.get(len-2).unwrap().calories;
//     total += all_elfs.get(len-1).unwrap().calories;



//     println!("{:?}", total);
//     Ok(())
// }