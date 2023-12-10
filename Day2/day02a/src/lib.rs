use std::collections::HashMap;
use regex::Regex;
use std::error::Error;

pub fn get_legitimate_id_or_zero(line: &str) -> i32 {
    let mut color_cubes = color_cube_map();
    let game_id = get_game_id(line).unwrap().parse::<i32>().unwrap();
    get_cubes_in_game(line, &mut color_cubes).unwrap();
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    if color_cubes.get(&"red".to_string()).unwrap().parse::<i32>().unwrap() > 12 
    || color_cubes.get(&"green".to_string()).unwrap().parse::<i32>().unwrap() > 13 
    || color_cubes.get(&"blue".to_string()).unwrap().parse::<i32>().unwrap() > 14 {
        return 0
    }
    return game_id;
}

//Instantiates a color cube map to keep track of cube numbers
pub fn color_cube_map() -> HashMap<String, String> {
    let mut color_cubes: HashMap<String, String> = HashMap::new();
    color_cubes.insert("red".to_string(), "0".to_string());
    color_cubes.insert("blue".to_string(), "0".to_string());
    color_cubes.insert("green".to_string(), "0".to_string());
    color_cubes
}

// Grabs Game ID out of the Input String
pub fn get_game_id(line: &str) -> Result<&str, Box<dyn Error>> {
    let re = Regex::new(r"Game (\d+):")?;
    let caps = re.captures(line).ok_or("No Game ID Found")?;
    let game_id = caps.get(1).ok_or("No Game ID Found")?.as_str();
    return Ok(game_id);
}
// Iterates over line and grabs values of color and inserts to map
pub fn get_cubes_in_game(line: &str, color_cubes: &mut HashMap<String, String> ) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"(\d+) (\w+)")?;
    for cap in re.captures_iter(line) {
        let number_str = &cap[1];
        let color = cap[2].to_string();

        if let Ok(number) = number_str.parse::<i32>() {
            color_cubes.entry(color)
                .and_modify(|e| {
                    if e.parse::<i32>().unwrap_or(0) < number {
                        *e = number.to_string();
                    }
                })
                .or_insert(number.to_string());
        }
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_returns_one_if_game_is_possible() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(get_legitimate_id_or_zero(input), 1);
    }

    #[test]
    fn it_returns_zero_if_game_is_impossible() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        assert_eq!(get_legitimate_id_or_zero(input), 0);
    }
}