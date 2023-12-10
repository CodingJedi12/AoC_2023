use std::collections::HashMap;
use regex::Regex;
use std::error::Error;

pub fn get_minimum_cubes(line: &str) -> i32 {
    let mut color_cubes = color_cube_map();
    get_cubes_in_game(line, &mut color_cubes).unwrap();
    println!("{:?}", color_cubes);
    let product = color_cubes.get(&"red".to_string()).unwrap().parse::<i32>().unwrap() 
    * color_cubes.get(&"green".to_string()).unwrap().parse::<i32>().unwrap() 
    * color_cubes.get(&"blue".to_string()).unwrap().parse::<i32>().unwrap();
    return product;
}

//Instantiates a color cube map to keep track of cube numbers
pub fn color_cube_map() -> HashMap<String, String> {
    let color_cubes: HashMap<String, String> = HashMap::new();
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