mod util;
use core::num;

use itertools::Itertools;
fn main() {
    struct Bag{
        red_cubes: u16,
        green_cubes: u16,
        blue_cubes: u16
    }
    let contents: String = util::read_file_as_lines(r"src\input.txt");
    let contents_2: String = contents.clone();
    dbg!(&contents);
    // let part_one_solution: u16 = part_one(contents);
    let part_one_answer = part_one(contents);
    dbg!(part_one_answer);
    // let part_two_solution: u16 = part_two(contents_2);
    // dbg!(part_one_solution);
    // dbg!(part_two_solution);
}

fn part_one(contents: String) -> i32 {
    contents.lines().map(
        |c| match c.split_once(':').take(){
            Some(part) => part,
            None => ("","")
        }   ).map(|parts| match parts.1.split_once('|').take(){
            Some(numbers) => numbers,
            None => ("","")
            }
        ).map(|numbers| {
            let my_numbers: Vec<&str> = numbers.1.split_whitespace().collect();
            dbg!(&my_numbers);
            let winning_numbers: Vec<&str> = numbers.0.split_whitespace().collect();
            dbg!(&winning_numbers);
            my_numbers.iter().fold(0,| acc, x| {
                if winning_numbers.contains(&x){
                    acc + 1
                    }
                else{
                    acc
                    }
             
                }
            )
        }
    ).map(|x: i32| {
        if x > 0{
            (2 as i32).pow((x-1) as u32)
        }
        else{
            0
        }
    }).inspect(|x| {dbg!(x);}).sum()
}


fn part_two(contents: String) -> u16 {
    let game_board = contents.lines().map(
        |c| match c.split_once(':').take(){
            Some(part) => part,
            None => ("","")
        }   ).map(|parts| match parts.1.split_once('|').take(){
            Some(numbers) => numbers,
            None => ("","")
            }
        );
    game_board.map(|numbers| {
            let my_numbers: Vec<&str> = numbers.1.split_whitespace().collect();
            dbg!(&my_numbers);
            let winning_numbers: Vec<&str> = numbers.0.split_whitespace().collect();
            dbg!(&winning_numbers);
            my_numbers.iter().fold(0,| acc, x| {
                if winning_numbers.contains(&x){
                    acc + 1
                    }
                else{
                    acc
                    }
             
                }
            )
        }
    ).map(|x: i32| {
        if x > 0{
            (2 as i32).pow((x-1) as u32)
        }
        else{
            0
        }
    }).inspect(|x| {dbg!(x);}).sum()
}
