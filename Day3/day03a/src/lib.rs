pub fn find_parts() {
    let lines = include_str!("../../day3_input.txt").lines().collect::<Vec<_>>();
    // Initialize Sum
    let mut sum = 0;

    // Itarate through each line with y as the iterator (y as row)
    for (y, line) in lines.iter().enumerate() {
        // Initialize x to keep track of the column we're in
        let mut x = 0;
        // Unitl we reach the end of the given line
        while x < line.len() {
            // Grab the character at the given x, y point
            let ch = line.chars().nth(x).unwrap();

            if ch.is_digit(10) {
                let mut num_str = String::new();
                // Initialize an ending value so we can continue to iterate and grab the whole digit
                let mut num_end = x;

                // If you get to the end of a line, or you reach a ch that != a digit exit
                while num_end < line.len() && line.chars().nth(num_end).unwrap().is_digit(10) {
                    num_str.push(line.chars().nth(num_end).unwrap());
                    num_end += 1;
                }

                let num = num_str.parse::<i32>().unwrap();

                // Pass the digit we have into the function to see if any are adjacent to a symbol
                if is_any_digit_adjacent_to_symbol(x, num_end - 1, y, &lines) {
                    sum += num;
                }
                // Reassign x to num_end so we don't repeat
                x = num_end;
            } else {
                x += 1;
            }
        }
    }

    println!("Sum of part numbers: {}", sum);
}

fn is_any_digit_adjacent_to_symbol(start_x: usize, end_x: usize, y: usize, schematic: &[&str]) -> bool {
    // step through the columns (x) in the row (y) and see if it is adjacent to symbol
    for x in start_x..=end_x {
        if is_adjacent_to_symbol(x, y, schematic) {
            return true;
        }
    }
    false
}

pub fn is_adjacent_to_symbol(x: usize, y: usize, schematic: &[&str]) -> bool {
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
        if adjacent_x >= 0 
        && (adjacent_x as usize) < schematic[0].len() 
        && adjacent_y >= 0 
        && (adjacent_y as usize) < schematic.len() {
            // Grab the character at lines[y].chars().nth(x).unwrap()
            // So essentially grab the row and then in the row, grab the nth (column)
            let adjacent_char = schematic[adjacent_y as usize]
                                        .chars()
                                        .nth(adjacent_x as usize)
                                        .unwrap();

            // If the ch is not a '.' and is not a digit, then its a symbol
            if adjacent_char != '.' && !adjacent_char.is_ascii_digit() {
                return true;
            }
        }
    }

    return false;
}

