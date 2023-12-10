use std::collections::HashSet;

pub fn find_sum(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    // Initialize Sum
    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '*' {
                if let Some(gear_ratio) = calculate_gear_ratio(x, y, &lines) {
                    sum += gear_ratio;
                };
            }
        }
    }
    println!("{sum}");
    return sum;
}
pub fn calculate_gear_ratio(x: usize, y: usize, schematic: &[&str]) -> Option<i32> {
    let mut adjacent_parts = Vec::new();
    let mut processed_positions = HashSet::new();
    let offsets = [(-1, -1), // Left and Down
                                    (-1, 0), // Left
                                    (-1, 1), // Left and Up
                                    (0, -1), // Down
                                    (0, 1), // Up
                                    (1, -1), // Right and Down
                                    (1, 0), // Right
                                    (1, 1)]; // Right and Up

    for &(offset_x, offset_y) in &offsets {
        // Assign adjacent x, y as the original (convert to isize bc of negatives) and add the offset
        let adjacent_x = x as isize + offset_x;
        let adjacent_y = y as isize + offset_y;

        // Check adjacent x, y to see if we have not overstepped our 2d matrix
        if is_in_bounds(x, y, schematic) {
            let adjacent_char = schematic[adjacent_y as usize]
                                        .chars()
                                        .nth(adjacent_x as usize)
                                        .unwrap();

            if adjacent_char.is_ascii_digit() {
                let (part_number, start_x) = extract_part_number(adjacent_x as usize, adjacent_y as usize, &schematic);
    
                // Check if this part number's position has already been processed
                if processed_positions.insert((start_x, adjacent_y as usize)) {
                    adjacent_parts.push(part_number);
                }
            }
        }
    }
    if adjacent_parts.len() == 2 {
        Some(adjacent_parts[0] * adjacent_parts[1])
    } else {
        None
    }
}

pub fn extract_part_number(x: usize, y: usize, schematic: &[&str]) -> (i32, usize) {
    let line = schematic[y];
    let mut start = x;
    let mut end = x;

    while start > 0 && line.chars().nth(start - 1).unwrap().is_ascii_digit() {
        start -= 1;
    }
    while end < line.len() && line.chars().nth(end).unwrap().is_ascii_digit() {
        end += 1;
    }

    let num_str = &line[start..end];

    let part_number = num_str.parse::<i32>().unwrap_or(0);
    return (part_number, start);
}

pub fn is_in_bounds(x: usize, y: usize, schematic: &[&str]) -> bool {
    (x as usize) < schematic[0].len()
    && (y as usize) < schematic.len()
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn sum_of_gear_ratios() {
        let input = include_str!("../../day3_test.txt");
        assert_eq!(find_sum(input), 467835);
    }

}