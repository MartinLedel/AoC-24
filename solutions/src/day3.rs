use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() -> io::Result<()> {
    let path: &Path = Path::new("files/mull_it_over_memory.txt");
    let file: File = File::open(&path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut solution1: i64 = 0;
    let mut solution2: i64 = 0;
    let re_mul: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_dont: Regex = Regex::new(r"don't\(\)").unwrap();
    let re_do: Regex = Regex::new(r"do\(\)").unwrap();
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<String>, _>>()?;

    for line in &lines {
        solution1 += standard_mul(line, &re_mul).unwrap();
    }

    let combined: String = lines.join("");
    solution2 += expanded_mul(&combined, &re_mul, &re_dont, &re_do).unwrap();

    println!("day3: {} and {}", solution1, solution2);

    Ok(())
}

fn standard_mul(line: &str, re_mul: &Regex) -> io::Result<i64> {
    let mut mul_num: i64 = 0;

    for cap in re_mul.captures_iter(&line) {
        let num1: i64 = cap[1].parse().unwrap();
        let num2: i64 = cap[2].parse().unwrap();
        mul_num += num1 * num2;
    }

    Ok(mul_num)
}

fn expanded_mul(combined: &str, re_mul: &Regex, re_dont: &Regex, re_do: &Regex) -> io::Result<i64> {
    let mut mul_num: i64 = 0;
    let mut in_dont: bool = false;
    let mut last_pos: usize = 0;

    for cap in re_mul.captures_iter(combined) {
        let pre_match: &str = &combined[last_pos..cap.get(0).unwrap().end()];

        if re_dont.is_match(pre_match) {
            in_dont = true;
        }

        if in_dont {
            if re_do.is_match(pre_match) {
                in_dont = false;
            } else {
                continue;
            }
        }

        let num1: i64 = cap[1].parse().unwrap();
        let num2: i64 = cap[2].parse().unwrap();
        mul_num += num1 * num2;

        last_pos = cap.get(0).unwrap().end();
    }

    Ok(mul_num)
}
