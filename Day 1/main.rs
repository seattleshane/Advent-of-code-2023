mod util;
fn main() {
    let file_path = r"C:\Users\shane\OneDrive\Desktop\Advent 2023\Day 1\src\test.txt";
    let contents: String = util::read_file_as_lines(&file_path);
    println!("{}", contents);
    let mut totals: Vec<u16> = Vec::new();
    let mut current_total: u16 = 0;
    for line in contents.lines(){
        if line.len() > 0{
            current_total = current_total + line.parse::<u16>().unwrap();
        }
        else{
            totals.append(&current_total);
            current_total = 0;

        }
    }

}
