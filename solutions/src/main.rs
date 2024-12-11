mod day1;
mod day2;
mod day3;

fn main() {
    match day1::main() {
        Ok(solution) => solution,
        Err(e) => eprintln!("Error occurred: {}", e),
    }
    match day2::main() {
        Ok(solution) => solution,
        Err(e) => eprintln!("Error occurred: {}", e),
    }
    match day3::main() {
        Ok(solution) => solution,
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
