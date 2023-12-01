mod util;

fn main(){
    let file_path = r"C:\Users\shane\OneDrive\Documents\GitHub\Advent-of-code-2023\Day 1\input.txt";
    let contents: String = util::read_file_as_lines(&file_path);
    println!("{}", contents);
    let mut totals: Vec<usize> = Vec::new();
    let mut current_total: usize = 0;
    for line in contents.lines(){
        if line.len() > 0{
            match line.parse::<usize>(){
                Ok(number) =>   current_total = current_total + number,
                Err(error) => println!("{}", error)
            }
        }
        else {
            totals.push(current_total);
            current_total = 0;
        }
    }
    let top_three: usize = get_top_three(&totals);
    println!("Maximum calories {}",totals.iter().max().unwrap());
    println!("Total calories of the top three elves {}", top_three)
}
fn get_top_three(totals: &Vec<usize>) -> usize{
    println!("Total: {:?}", &totals);
    let mut totals: Vec<usize> = totals.clone();
    totals.sort();
    totals.reverse();
    println!("Top Three totals: {:?}", &totals);
    let sorted_totals: &[usize] = &totals[0..3];
    println!("top three sorted: {:?}", &sorted_totals);
    sorted_totals.iter().sum()
}