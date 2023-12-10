
pub fn scratchcards(input: &str) -> u32 {
    let mut points: u32 = 0;
    let lines = input.lines().collect::<Vec<_>>();
    for line in lines {
        points += get_winning_nums(line);
    }
    
    println!("{points}");
    points
}

pub fn get_winning_nums(line: &str) -> u32 {
    let mut matches: u32 = 0;
    let numbers: &&str = &line.split(":").nth(1).unwrap_or("");
    let winning_numbers = numbers.trim().split("|").next().unwrap_or("").split_whitespace().collect::<Vec<_>>();
    let numbers_to_match = numbers.trim().split("|").nth(1).unwrap_or("").split_whitespace().collect::<Vec<_>>();
    for num in &winning_numbers {
        if numbers_to_match.contains(&num){
            matches = match matches {
                0 => 1,
                m if m > 0 => matches * 2,
                _ => matches
            }
        }
    }
    return matches;
    
    
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("../../day4_test.txt");
        assert_eq!(scratchcards(input), 13)
    }
}