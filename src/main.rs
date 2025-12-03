use std::{fs, ops::RangeInclusive, path::Path, result};
fn main() {
    day_one("./input_day_one.txt");
    day_two("./input_day_two.txt");
    day_three("./input_day_three.txt");
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

struct IdRange {
    start: u64,
    end: u64
}

impl IdRange {
    fn new_from_string(range_text: &str) -> Option<Self> {
        let range_numbers = range_text
            .split('-')
            .map(|s| match s.strip_suffix('\n') {
                Some(stripped) => stripped,
                None => s,
            })
            .filter_map(|s| s.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        if range_numbers.len() != 2 {
            return None;
        }
        let start = range_numbers.get(0)?.clone();
        let end = range_numbers.get(1)?.clone();
        return Some(IdRange { start, end });
    }

    fn iter(&self) -> RangeInclusive<u64> {
        self.start..=self.end
    }
}

fn day_two(path_string: &str) {
    println!("Day Two:");
    let file_path = Path::new(path_string);
    let input: String = fs::read_to_string(file_path).expect("Should have successfully read the file");
    let mut invalid_id_list_part_one: Vec<u64> = Vec::new();
    let mut invalid_id_list_part_two: Vec<u64> = Vec::new();
    for range_text in input.split(',') {
        let id_range = IdRange::new_from_string(range_text)
            .expect("Should be a valid input");
        
        for id in id_range.iter() {
            if is_invalid_id_part_one(id) {
                invalid_id_list_part_one.push(id);
            }
            if is_invalid_id_part_two(id) {
                invalid_id_list_part_two.push(id);
            }
        }
    }
    let total_invalid_ids_part_one: u64 = invalid_id_list_part_one.iter().sum();
    let total_invalid_ids_part_two: u64 = invalid_id_list_part_two.iter().sum();
    println!("    Part One: {}",total_invalid_ids_part_one);
    println!("    Part Two: {}",total_invalid_ids_part_two);
}

fn is_invalid_id_part_one(id: u64) -> bool {
    let id_string = id.to_string();
    let length = id_string.len();
    let (start, end) = id_string.split_at(length / 2);
    return start.eq(end);
}

fn is_invalid_id_part_two(id: u64) -> bool {
    let id_string = id.to_string();
    let length = id_string.len();
    for segment_size in 1..=(length/2) {
        if repeats_with_freq(&id_string, segment_size) {
            return true;
        }
    }
    let (start, end) = id_string.split_at(id_string.len() / 2);
    return start.eq(end);
}

fn repeats_with_freq(string: &str, freq: usize) -> bool {
    let (prefix, rest) = string.split_at(freq);
    let mut rest = rest;
    if rest.is_empty() {
        return false;
    }
    loop {
        rest = match rest.strip_prefix(prefix) {
            Some(s) => {
                if s.is_empty() {
                    return true;
                }
                s
            },
            None => return false
        }
    }
}

fn day_three(path_string: &str) {
    println!("Day Two:");
    let file_path = Path::new(path_string);
    let input: String = fs::read_to_string(file_path).expect("Should have successfully read the file");
    let mut total_joltage_part_one = 0;
    let mut total_joltage_part_two = 0;
    for bank_string in input.lines() {
        total_joltage_part_one += process_bank(bank_string, 2);
        total_joltage_part_two += process_bank(bank_string,12);
    }
    println!("    Part One: {}",total_joltage_part_one);
    println!("    Part Two: {}",total_joltage_part_two);
}

// unused, used to figure out first problem, but has been replaced with more flexible solution
fn process_bank_two_cells(bank_string: &str) -> i64 {
    // map to i64s
    let mut bank: Vec<i64> = bank_string.as_bytes().iter().map(|c| (c.clone() - 48).into() ).collect();
    let last = bank.pop().expect("bank should have at least one item");
    let mut max = -1;
    let mut max_pos = 0;
    for (i,val) in bank.iter().cloned().enumerate() {
        if val > max {
            max = val;
            max_pos = i;
        }

    }
    bank.push(last);
    let second = bank.iter()
        .cloned()
        .skip(max_pos + 1)
        .max()
        .expect("should be at least one element after this");
    let result = max * 10 + second;
    return result;
}

fn process_bank(bank_string: &str, n: usize) -> i64 {
    // map to i64s
    let mut bank: Vec<i64> = bank_string.as_bytes().iter().map(|c| (c.clone() - 48).into() ).collect();
    // take the last n - 1 cells, as we want the max not in those
    let mut last_cells: Vec<i64> = bank.split_off(bank.len() - n + 1).into_iter().rev().collect();
    let mut cells_to_use: Vec<i64> = Vec::new();
    // check that our assumptions hold
    assert!(n - 1 == last_cells.len());
    loop {
        // find the value and position of the maximum cell that we can take
        let mut max = -1;
        let mut max_pos = 0;
        for (i,val) in bank.iter().cloned().enumerate() {
            if val > max {
                max = val;
                max_pos = i;
            }
        }
        // we want to save the maximum cell we have found
        cells_to_use.push(max);
        // we know we are done when we have found all the digits we need
        if cells_to_use.len() == n {
            break;
        }
        // skip over the cells before and including the max cell, so now we just look at the bank after
        // that point
        bank = bank.into_iter().skip(max_pos + 1).collect();
        // we want to add a cell which could now be included back from the last cells
        bank.push(last_cells.pop().expect("Should still be cells remaining"));
    }
    // calculate by treating list as base 10 digits
    let mut curr_digit = 1;
    let mut result = 0;
    while let Some(cell_value) = cells_to_use.pop() {
        result += cell_value * curr_digit;
        curr_digit *= 10;
    }
    return result;

}




