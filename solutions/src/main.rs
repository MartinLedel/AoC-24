mod day1;

fn main() {
    match day1::main() {
        Ok(solution) => solution,
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
