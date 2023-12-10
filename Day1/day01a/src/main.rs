use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    // Runs the function, but uses the _ in order to say "We don't need to use the value that is returned"
    let _ = day_1a_solution();
}

// Declares the function and its's type, Result<T, E>, with the error return being a Box error. Results returns Okay or Error
fn day_1a_solution() -> Result<(), Box<dyn std::error::Error>> {
    // Instantiates the file variable
    let file_path = "./day1a_input.txt";
    // Instantiates sum and set it to be mutable (changeable)
    let mut sum = 0;
    
    // Opens the file using the File tool and sets it to a variable
    // The "?" is used in order to essentially do a 'try, catch' block with the task
    // If the task is successful, it moves on. If it fails (can't find the file or can't open the file), it throws an error and stops running the code
    let file = File::open(file_path)?;
    // Generates a reader to read the file
    let reader = BufReader::new(file);
    // Iterates through the lines in the file using the reader
    for line in reader.lines() {
        // If there's a problem reading the line, eject and report error
        let line = line?;
        // Sets up a mutable vector (list)
        let mut all_nums_in_line = Vec::new();
        // For every character in the line, check to see if it's numeric and push it to the list
        for char in line.chars() {
            if char.is_numeric() {
                all_nums_in_line.push(char);
            }
        }
        // Declare an i32 variable that takes the first and last item in the vector and formats to a combined string before parsing
        let calibration_value: i32 = format!("{}{}", all_nums_in_line[0], all_nums_in_line[all_nums_in_line.len() - 1]).parse().expect("Something went wrong parsing to int");
        sum += calibration_value;
    }

    println!("Sum: {}", sum);
    Ok(())
}
