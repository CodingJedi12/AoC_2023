use day02b::get_minimum_cubes;

fn main() {
    println!(
        "{}",
        include_str!("../day2b_input.txt")
            .lines()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|line| get_minimum_cubes(line))
            .sum::<i32>(),
    );
}
