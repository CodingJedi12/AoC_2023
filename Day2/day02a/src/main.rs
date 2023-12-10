use day02a::get_legitimate_id_or_zero;

fn main() {
    println!(
        "{}",
        include_str!("../day2a_input.txt")
            .lines()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|line| get_legitimate_id_or_zero(line))
            .sum::<i32>(),
    );
}