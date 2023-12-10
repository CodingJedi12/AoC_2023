use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let _ = day_1b_solution();
}

fn day_1b_solution() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./day1b_input.txt";
    let mut sum = 0;
    let words_to_num = create_words_to_num_map();
    let conjoined_num_map = conjoined_num_map();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let line = compare_string_to_map(&line, &conjoined_num_map);
        let line = compare_string_to_map(&line, &words_to_num);
        let mut all_nums_in_line = Vec::new();
        for char in line.chars() {
            if char.is_numeric() {
                all_nums_in_line.push(char);
            }
        }
        let calibration_value: i32 = format!("{}{}", all_nums_in_line[0], all_nums_in_line[all_nums_in_line.len() - 1]).parse().expect("Something went wrong parsing to int");
        sum += calibration_value;
    }
    println!("{sum}");
    Ok(())
}
fn create_words_to_num_map() -> HashMap<&'static str, &'static str> {
    let mut words_to_num = HashMap::new();
    words_to_num.insert("zero", "0");
    words_to_num.insert("one", "1");
    words_to_num.insert("two", "2");
    words_to_num.insert("three", "3");
    words_to_num.insert("four", "4");
    words_to_num.insert("five", "5");
    words_to_num.insert("six", "6");
    words_to_num.insert("seven", "7");
    words_to_num.insert("eight", "8");
    words_to_num.insert("nine", "9");
    words_to_num
}
fn conjoined_num_map() -> HashMap<&'static str, &'static str> {
    let mut words_to_num = HashMap::new();
    words_to_num.insert("oneight", "1eight");
    words_to_num.insert("twone", "2one");
    words_to_num.insert("threeight", "3eight");
    words_to_num.insert("fiveight", "5eight");
    words_to_num.insert("eightwo", "8two");
    words_to_num.insert("eighthree", "8three");
    words_to_num.insert("nineight", "9eight");
    words_to_num
}
fn compare_string_to_map(input: &str, words_to_num: &HashMap<&str, &str>) -> String { 
    let mut result = String::from(input);
    for (key, value) in words_to_num {
        result = result.replace(key, value);
    }
    result
} 