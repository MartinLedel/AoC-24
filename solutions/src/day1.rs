use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() -> io::Result<()> {
    let path = Path::new("files/historian_ids.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut num_list1: Vec<i32> = Vec::new();
    let mut num_list2: Vec<i32> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let left: i32 = parts[0].parse().expect("Invalid number on the left");
            let right: i32 = parts[1].parse().expect("Invalid number on the right");

            num_list1.push(left);
            num_list2.push(right);
        } else {
            println!("Skipping invalid line: {}", line);
        }
    }

    let mut solution_vec: Vec<i32> = Vec::new();

    num_list1.sort();
    num_list2.sort();

    for (_i, (a, b)) in num_list1.iter().zip(num_list2.iter()).enumerate() {
        let mut distance: i32 = a - b;
        if distance < 0 {
            distance = distance * -1;
        }
        solution_vec.push(distance);
    }

    let solution1: i32 = solution_vec.iter().sum();
    let mut solution2: i32 = 0;

    for &left_item in &num_list1 {
        let mut count = 0;
        count += num_list2.iter().filter(|&&x| x == left_item).count() as i32;
        solution2 += left_item * count;
    }
    println!("day1: {} and {}", solution1, solution2);

    Ok(())
}
