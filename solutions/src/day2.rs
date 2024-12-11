use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() -> io::Result<()> {
    let path: &Path = Path::new("files/red-nosed_report.txt");
    let file: File = File::open(&path)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut solution1: i32 = 0;
    let mut solution2: i32 = 0;

    for line_result in reader.lines() {
        let line: String = line_result?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            continue;
        }

        if is_safe(&parts) {
            solution1 += 1;
        }

        if problem_dampener(&parts) {
            solution2 += 1;
        }
    }

    println!("day2: {} and {}", solution1, solution2);

    Ok(())
}

fn is_safe(parts: &[&str]) -> bool {
    let mut direction: Option<bool> = None;

    for i in 0..parts.len() - 1 {
        let current_val: i32 = match parts[i].parse::<i32>() {
            Ok(v) => v,
            Err(_) => return false,
        };
        let next_val: i32 = match parts[i + 1].parse::<i32>() {
            Ok(v) => v,
            Err(_) => return false,
        };
        let diff: i32 = next_val - current_val;

        if diff.abs() > 3 || diff.abs() == 0 {
            return false;
        }

        match direction {
            None => {
                if diff > 0 {
                    direction = Some(true); // Increasing
                } else if diff < 0 {
                    direction = Some(false); // Decreasing
                }
            }
            Some(true) => {
                if diff <= 0 {
                    return false;
                }
            }
            Some(false) => {
                if diff >= 0 {
                    return false;
                }
            }
        }
    }

    true
}

fn problem_dampener(parts: &[&str]) -> bool {
    let mut safety_check: i32 = 0;
    let mut direction: Option<bool> = None;

    for i in 0..parts.len() - 1 {
        let current_val: i32 = match parts[i].parse::<i32>() {
            Ok(v) => v,
            Err(_) => return false,
        };
        let next_val: i32 = match parts[i + 1].parse::<i32>() {
            Ok(v) => v,
            Err(_) => return false,
        };
        let diff: i32 = next_val - current_val;

        if diff.abs() > 3 || diff.abs() == 0 {
            if safety_check == 0 {
                safety_check = 1;
                continue;
            } else {
                return false;
            }
        }

        match direction {
            None => {
                if diff > 0 {
                    direction = Some(true); // Increasing
                } else if diff < 0 {
                    direction = Some(false); // Decreasing
                }
            }
            Some(true) => {
                if diff <= 0 {
                    if safety_check == 0 {
                        safety_check = 1;
                        continue;
                    } else {
                        return false;
                    }
                }
            }
            Some(false) => {
                if diff >= 0 {
                    if safety_check == 0 {
                        safety_check = 1;
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        }
    }

    true
}
