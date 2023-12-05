use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let lines = read_file("input.txt").expect("unable to read file");
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
    let soil_to_fertilizer = &maps[1];
    let fertilizer_to_water = &maps[2];
    let water_to_light = &maps[3];
    let light_to_temperature = &maps[4];
    let temperature_to_humidity = &maps[5];
    let humidity_to_location = &maps[6];

    let locations = seeds
        .iter()
        .map(|id| {
            Seed::new(id.parse::<usize>().expect("not fail"))
                .transform_seed()
                .to_soil(seed_to_soil_map)
                .to_fertilizer(soil_to_fertilizer)
                .to_water(fertilizer_to_water)
                .to_light(water_to_light)
                .to_temperature(light_to_temperature)
                .to_humidity(temperature_to_humidity)
                .to_location(humidity_to_location)
        })
        .min()
        .expect("will not");
    locations

    // for (index, id) in seeds.iter().enumerate() {
    //     if index == 0 {
    //         let num = Seed::new(id.parse::<usize>().expect("not fail"))
    //             .transform_seed()
    //             .to_soil(seed_to_soil_map)
    //             .to_fertilizer(soil_to_fertilizer)
    //             .to_water(fertilizer_to_water)
    //             .to_light(water_to_light)
    //             .to_temperature(light_to_temperature)
    //             .to_humidity(temperature_to_humidity)
    //             .to_location(humidity_to_location);

    //         println!("seed ={num}");
    //         // println!("seed {:?}", item);
    //     }
    // }
    // 0
}

fn read_file(name: &str) -> io::Result<Vec<String>> {
    let dir = std::env::current_dir().unwrap().display().to_string();
    let file_path = format!("{dir}/src/bin/{name}");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().peekable();
    let mut current_block_group = String::new();

    let mut string_arr = vec![];

    let length = if name == "text.txt" {
        include_str!("./text.txt").lines().collect::<Vec<_>>().len() - 1
    } else {
        include_str!("./input.txt")
            .lines()
            .collect::<Vec<_>>()
            .len()
            - 1
    };

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
    fn new(id: usize) -> Self {
        Self { id }
    }

    fn transform_seed(self) -> SeedToSoil {
        SeedToSoil::new("", self.id)
    }
}

fn create_vec_number_settings(input: &str) -> Option<Vec<NumBerSettings>> {
    let input = input
        .split(':')
        .last()
        .expect("has second")
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let mut number_settings = vec![];

    for item in input {
        let numbers = item
            .split(' ')
            .map(|x| x.parse::<usize>().expect("msg"))
            .collect::<Vec<_>>();
        let mut number_setting = NumBerSettings::new(&numbers[..3]);
        number_setting.set_destination_end();
        number_setting.set_source_end();
        number_settings.push(number_setting);
    }

    if !number_settings.is_empty() {
        Some(number_settings)
    } else {
        None
    }
}

fn transform(number_settings: Vec<NumBerSettings>, num: usize) -> usize {
    let range_map = number_settings
        .iter()
        .filter(|x| {
            let range = x.source..=x.source_end;
            range.contains(&num)
        })
        .collect::<Vec<_>>();

    if !range_map.is_empty() {
        let number_setting = range_map[0];
        let num = number_setting.source_end - num;
        number_setting.destination_end - num
    } else {
        num
    }
}

#[derive(Debug, Default)]
struct NumBerSettings {
    destination: usize,
    destination_end: usize,
    source: usize,
    source_end: usize,
    range: usize,
}

impl NumBerSettings {
    fn new(input: &[usize]) -> Self {
        Self {
            destination: input[0],
            source: input[1],
            range: input[2],
            ..Default::default()
        }
    }

    fn set_destination_end(&mut self) {
        let des = self.destination + self.range;
        self.destination_end = des - 1;
    }

    fn set_source_end(&mut self) {
        let des = self.source + self.range;
        self.source_end = des - 1;
    }
}

#[derive(Debug, Default)]
struct SeedToSoil {
    seed: usize,
    category: Vec<NumBerSettings>,
}

impl SeedToSoil {
    fn to_soil(&self, input: &str) -> SoilToFertilizer {
        let soil = Self::new(input, self.seed).tranform();
        SoilToFertilizer::new("", soil)
    }

    fn new(input: &str, seed: usize) -> Self {
        match create_vec_number_settings(input) {
            Some(category) => Self { category, seed },
            None => Self {
                seed,
                ..Default::default()
            },
        }
    }

    fn tranform(self) -> usize {
        let number_settings = self.category;
        let num = self.seed;
        transform(number_settings, num)
    }
}

#[derive(Debug, Default)]
struct SoilToFertilizer {
    soil: usize,
    category: Vec<NumBerSettings>,
}

impl SoilToFertilizer {
    fn new(input: &str, soil: usize) -> Self {
        match create_vec_number_settings(input) {
            Some(category) => Self { category, soil },
            None => Self {
                soil,
                ..Default::default()
            },
        }
    }

    fn to_fertilizer(&self, input: &str) -> FertilizerToWater {
        let fertilizer = Self::new(input, self.soil).tranform();
        FertilizerToWater::new("", fertilizer)
    }

    fn tranform(self) -> usize {
        let number_settings = self.category;
        let num = self.soil;
        transform(number_settings, num)
    }
}

#[derive(Debug, Default)]
struct FertilizerToWater {
    fertilizer: usize,
    category: Vec<NumBerSettings>,
}

impl FertilizerToWater {
    fn new(input: &str, fertilizer: usize) -> Self {
        match create_vec_number_settings(input) {
            Some(category) => Self {
                category,
                fertilizer,
            },
            None => Self {
                fertilizer,
                ..Default::default()
            },
        }
    }

    fn to_water(&self, input: &str) -> WaterToLight {
        let fertilizer = Self::new(input, self.fertilizer).tranform();
        WaterToLight::new("", fertilizer)
    }

    fn tranform(self) -> usize {
        let number_settings = self.category;
        let num = self.fertilizer;
        transform(number_settings, num)
    }
}

#[derive(Debug, Default)]
struct WaterToLight {
    water: usize,
    category: Vec<NumBerSettings>,
}

impl WaterToLight {
    fn new(input: &str, water: usize) -> Self {
        match create_vec_number_settings(input) {
            Some(category) => Self { category, water },
            None => Self {
                water,
                ..Default::default()
            },
        }
    }

    fn to_light(&self, input: &str) -> LightToTemperature {
        let water = Self::new(input, self.water).tranform();
        LightToTemperature::new("", water)
    }

    fn tranform(self) -> usize {
        let number_settings = self.category;
        let num = self.water;
        transform(number_settings, num)
    }
}

#[derive(Debug, Default)]
struct LightToTemperature {
    light: usize,
    category: Vec<NumBerSettings>,
}

impl LightToTemperature {
    fn new(input: &str, light: usize) -> Self {
        match create_vec_number_settings(input) {
            Some(category) => Self { category, light },
            None => Self {
                light,
                ..Default::default()
            },
        }
    }

    fn to_temperature(&self, input: &str) -> TemperatureTohumidity {
        let light = Self::new(input, self.light).tranform();
        TemperatureTohumidity::new("", light)
    }

    fn tranform(self) -> usize {
        let number_settings = self.category;
        let num = self.light;
        transform(number_settings, num)
    }
}

#[derive(Debug, Default)]
struct TemperatureTohumidity {
    temperature: usize,
    category: Vec<NumBerSettings>,
}

impl TemperatureTohumidity {
    fn new(input: &str, temperature: usize) -> Self {
        match create_vec_number_settings(input) {
            Some(category) => Self {
                category,
                temperature,
            },
            None => Self {
                temperature,
                ..Default::default()
            },
        }
    }

    fn to_humidity(&self, input: &str) -> HumidityToLocation {
        let temperature = Self::new(input, self.temperature).tranform();
        HumidityToLocation::new("", temperature)
    }

    fn tranform(self) -> usize {
        let number_settings = self.category;
        let num = self.temperature;
        transform(number_settings, num)
    }
}

#[derive(Debug, Default)]
struct HumidityToLocation {
    humidity: usize,
    category: Vec<NumBerSettings>,
}

impl HumidityToLocation {
    fn new(input: &str, humidity: usize) -> Self {
        match create_vec_number_settings(input) {
            Some(category) => Self { category, humidity },
            None => Self {
                humidity,
                ..Default::default()
            },
        }
    }

    fn to_location(&self, input: &str) -> usize {
        Self::new(input, self.humidity).tranform()
    }

    fn tranform(self) -> usize {
        let number_settings = self.category;
        let num = self.humidity;
        transform(number_settings, num)
    }
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
