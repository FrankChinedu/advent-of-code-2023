use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(name: &str) -> io::Result<Vec<String>> {
    let dir = std::env::current_dir().unwrap().display().to_string();
    let file_path = format!("{dir}/src/bin/{name}");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().peekable();
    let mut current_block_group = String::new();

    let mut string_arr = vec![];

    let length = include_str!("./input.txt")
        .lines()
        .collect::<Vec<_>>()
        .len()
        - 1;

    for (index, line) in lines.enumerate() {
        let line = line?;

        if line.trim().is_empty() || index == length {
            let new_string = current_block_group.clone();
            string_arr.push(new_string.to_string());
            current_block_group = String::new();
        } else {
            current_block_group.push_str(&line);
            current_block_group.push('\n');
        }
    }

    Ok(string_arr)
}

#[allow(dead_code)]
struct Seeds {
    seed: Vec<Seed>,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Seed {
    id: usize,
    // soil: usize,
    // fertilizer: usize,
    // water: usize,
    // light: usize,
    // temperature: usize,
    // humidity: usize,
    // location: usize,
}

impl Seed {
    #[allow(dead_code)]
    fn new(id: usize) -> Self {
        Self { id }
    }

    fn seed_to_soil(self, input: &str) -> SeedToSoil {
        SeedToSoil::new(input)
    }
}

#[allow(dead_code)]
fn create_map(input: &str) -> usize {
    let input = input.split(':').last().expect("has second");
    println!("compute_seed_location ={:?}", input);
    0
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct NumBerSettings {
    destination: usize,
    source: usize,
    range: usize,
    destination_end: usize,
    source_end: usize,
}

impl NumBerSettings {
    #[allow(dead_code)]
    fn set_destination_end(&mut self) {
        let des = self.destination + self.range;
        self.destination_end = des - 1;
    }
    fn set_source_end(&mut self) {
        let des = self.source + self.range;
        self.source_end = des - 1;
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct SeedToSoil {
    category: Vec<NumBerSettings>,
}

impl SeedToSoil {
    fn new(input: &str) -> Self {
        create_map(input);
        // println!("SeedToSoil ={:?}", input);
        Self {
            ..Default::default()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct SoilToFertilizer {
    category: Vec<NumBerSettings>,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct FertilizerToWater {
    category: Vec<NumBerSettings>,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct WaterToLight {
    category: Vec<NumBerSettings>,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct LightToTemperature {
    category: Vec<NumBerSettings>,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct TemperatureTohumidity {
    category: Vec<NumBerSettings>,
}

fn main() {
    let lines = read_file("text.txt").expect("unable to read file");
    let num = process(lines);
    println!("num -- {:?}", num);
}

fn process(input: Vec<String>) -> usize {
    let seeds = input.get(0).expect("seeds");
    let maps = input.get(1..).expect("msg");
    let seeds = seeds.split(':').collect::<Vec<_>>();
    let seeds = *seeds.last().expect("msg");
    let seeds = seeds.trim().split([' ', '\n']).collect::<Vec<_>>();

    let seed_to_soil_map = &maps[0];
    let _soil_to_fertilizer = &maps[1];
    let _fertilizer_to_water = &maps[2];
    let _water_to_light = &maps[3];
    let _light_to_temperature = &maps[4];
    let _temperature_to_humidity = &maps[5];
    let _humidity_to_location = &maps[6];

    for (index, id) in seeds.iter().enumerate() {
        if index == 0 {
            let seed = Seed::new(id.parse::<usize>().expect("not faile"));
            seed.seed_to_soil(seed_to_soil_map);
            println!("seed ={id}");
            // println!("seed {:?}", item);
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        let input = read_file("text.txt").expect("unable to read file");
        assert_eq!(35, process(input))
    }
}
