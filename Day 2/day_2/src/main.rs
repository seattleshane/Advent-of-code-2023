use std::fs;


fn main() {
    let file_path = r"C:\Users\shane\OneDrive\Documents\GitHub\Advent-of-code-2023\Day 2\day_2\src\example.txt";
    let contents: String = read_file_as_lines(&file_path);
    let contents_2: String = contents.clone();
    println!("{}", contents);
    let mut last_value: u16 = 0;
    let mut increasing: u16 = 0;
    let mut decreasing: u16 = 0;
    for line in contents.lines(){
        if last_value == 0{
            last_value = line.parse::<u16>().unwrap();
        }
        else if line.parse::<u16>().unwrap() > last_value {
            increasing += 1;
            last_value = line.parse::<u16>().unwrap();
        }
        else {
            decreasing += 1;
            last_value = line.parse::<u16>().unwrap();
        }
    }
    println!("Number of increases: {}, number of decreases {}", increasing, decreasing);
    let nums = contents_2.lines().map(|l| l.parse::<usize>().expect("must num")).collect::<Vec<usize>>();
    let mut vector_of_windows: Vec::<usize> = Vec::new();
    let mut last_value_2: usize = 0;
    let mut increasing_2: usize = 0;
    let mut decreasing_2: usize = 0;
    for i in 0.. nums.len()-2{
        vector_of_windows.push(nums[i .. i+3].iter().sum::<usize>());
    }
    println!("{:?}", vector_of_windows);
    for i in 0 .. vector_of_windows.len(){
        if last_value_2 == 0{
            last_value_2 = vector_of_windows[i];
        }
        else if vector_of_windows[i] > last_value_2{
            increasing_2 += 1;
            last_value_2 = vector_of_windows[i];
        }
        else {
            decreasing_2 += 1;
            last_value_2 = vector_of_windows[i];
        }
    }
    println!("The number of increases part 2  {}, number of decreases {}", increasing_2, decreasing_2)
}


pub fn read_file_as_lines(file_path: &str)-> String{
    fs::read_to_string(file_path).unwrap()
}