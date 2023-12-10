
pub fn scratchcards(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut total_scratchcards: u32 = 0;

    for (x, line) in lines.iter().enumerate() {
        total_scratchcards += 1; // Count the scratchcard for the current line
        total_scratchcards += get_additional_matches(x, line, &lines); // Add additional scratchcards based on matches
    }
    println!("{total_scratchcards}");
    total_scratchcards
}

pub fn get_additional_matches(x: usize, line: &str, lines: &[&str]) -> u32 {
    let mut total_additional: u32 = 0;
    let matches = get_matches(line);

    for i in 1..=matches as usize {
        if x + i < lines.len() {
            total_additional += 1; // Add one for each additional line
            total_additional += get_additional_matches(x + i, lines[x + i], lines); // Recursively add additional matches from the next line
        }
    }

    total_additional
}

pub fn get_matches(line: &str) -> u32 {
    let numbers: &&str = &line.split(':').nth(1).unwrap_or("");
    let winning_numbers = numbers.trim().split('|').next().unwrap_or("").split_whitespace().collect::<Vec<_>>();
    let numbers_to_match = numbers.trim().split('|').nth(1).unwrap_or("").split_whitespace().collect::<Vec<_>>();

    let mut matches: u32 = 0;
    for num in &winning_numbers {
        if numbers_to_match.contains(&num) {
            matches += 1;
        }
    }

    matches
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("../../day4_test.txt");
        assert_eq!(scratchcards(input), 30)
    }
}