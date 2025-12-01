use std::{fs, path::Path};
fn main() {
    day_one("./input.txt");
}

fn day_one(path_string: &str) {
    println!("Day One:");
    let file_path = Path::new(path_string);
    let input: String = fs::read_to_string(file_path).expect("Should have successfully read the file");
    let mut curr_val = 50;
    let mut count_part_one = 0;
    let mut count_part_two = 0;
    for line in input.lines() {
        let rotate_by: i32 = if let Some(num_string) = line.strip_prefix("L") {
            - num_string.parse::<i32>().expect("should be a valid integer")
        }
        else if let Some(num_string) = line.strip_prefix("R") {
            num_string.parse().expect("should be a valid integer")
        }
        else {
            panic!("Invalid input file");
        };

        let sign = rotate_by.signum();
        count_part_two += rotate_by.abs() / 100;
        for _ in 0..(rotate_by.abs() % 100) {
            curr_val += sign;
            curr_val %= 100;
            if curr_val == 0 {
                count_part_two += 1;
            }
        }
        if curr_val == 0 {
            count_part_one += 1;
        }
    }
    println!("    Part One: {}",count_part_one);
    println!("    Part Two: {}",count_part_two);
}





