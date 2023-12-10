pub fn lowest_location(input: &str) -> usize {
    let (seeds_str, blocks_str) = input.split_once("\n\n").unwrap();
    let seeds = get_seeds(seeds_str);
    let blocks = get_block_tuples(blocks_str);
    let lowest_location: usize = seeds.iter()
         .map(|&seed| map_through_blocks(seed, &blocks)) // For every seed, map it all the way through the blocks
         .min() // Take th min number available after mapping
         .unwrap_or(0);
    println!("{}", lowest_location);
    lowest_location
}

fn get_seeds(seeds_str: &str) -> Vec<usize> {
    let seeds: Vec<usize> = seeds_str
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    return seeds;
}
fn get_block_tuples(blocks_str: &str) -> Vec<Vec<(usize, usize, usize)>> {
    let mut blocks_result: Vec<Vec<(usize, usize, usize)>> = Vec::new();
    let blocks = blocks_str
    .split("\n\n")
    .map(|b| b
        .split_once("\n")
        .unwrap()
        .1
        .split("\n")
        .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    for block in blocks.iter() {
        let mut block_result: Vec<(usize, usize, usize)> = Vec::new();
        for line in block {
            let nums = line
            .split_whitespace()
            .map(|num| num
                .parse::<usize>()
                .unwrap())
                .collect::<Vec<usize>>();
            block_result.push((nums[0], nums[1], nums[2]))

        }
        blocks_result.push(block_result);
    }
    return blocks_result;
}
fn map_number(number: usize, tuple: &[(usize, usize, usize)]) -> usize {
    for &(destination_start, source_start, range_length) in tuple {
        if number >= source_start && number < source_start + range_length {
            return destination_start + (number - source_start); // If number is in range, return the difference between the number and source and add the destination start
        }
    }
    number  // If the number is not in the mapping, it maps to itself
}
fn map_through_blocks(mut number: usize, blocks: &[Vec<(usize, usize, usize)>]) -> usize {
    for line in blocks {
        number = map_number(number, line);
        // Essentially remaps itself and passes itself onto the next block
    }
    number // Poops out final number at the destination 
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("../../day5_test.txt");
        assert_eq!(lowest_location(input), 35)
    }
}
