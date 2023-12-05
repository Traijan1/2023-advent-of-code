use std::io::Error;

// #[derive(Copy, Clone, Debug)]
// enum Type {
//     SeedToSoil,
//     SoilToFertilizer,
//     FertilizerToWater,
//     WaterToLight,
//     LightToTemperature,
//     TemperatureToHumidity,
//     HumidityToLocation,
// }

#[derive(Clone, Copy, Debug)]
struct Map {
    // map_type: Type,
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

type Maps = Vec<Vec<Map>>;

fn main() {
    let input = parse_input().unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut seeds: Vec<i64> = get_seeds(lines[0]);
    let maps: Maps = get_maps(lines[3..].to_vec());
    let mut locations = get_locations(maps, seeds);
    locations.sort();

    println!("Lowest location: {}", locations[0]);
}

fn get_seeds(seed_line: &str) -> Vec<i64> {
    let mut seeds = vec![];

    for seed in seed_line.replace("seeds: ", "").split(" ") {
        seeds.push(seed.parse::<i64>().unwrap());
    }

    seeds
}

fn get_locations(maps: Maps, mut seeds: Vec<i64>) -> Vec<i64> {
    let mut locations = seeds.clone();

    for map in maps {
        for (index, mut seed) in seeds.iter().enumerate() {
            for m in &map {
                if locations[index] >= m.source_range_start
                    && locations[index] < m.source_range_start + m.range_length
                {
                    let step = m.destination_range_start - m.source_range_start;
                    locations[index] += step;

                    break;
                }
            }
        }

        println!("Seeds: {:?}", locations);
    }

    locations
}

fn get_maps(lines: Vec<&str>) -> Maps {
    let mut maps: Maps = vec![];
    let mut current_map: Vec<Map> = vec![];

    for line in lines {
        if line.contains("map") {
            maps.push(current_map.clone());
            current_map.clear();

            continue;
        } else if line.is_empty() {
            continue;
        }

        let numbers = line
            .split(" ")
            .map(|val| val.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        if let [destination_range_start, source_range_start, range_length] = numbers.as_slice() {
            current_map.push(Map {
                destination_range_start: *destination_range_start,
                source_range_start: *source_range_start,
                range_length: *range_length,
            });
        }
    }

    maps.push(current_map.clone());

    maps
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
