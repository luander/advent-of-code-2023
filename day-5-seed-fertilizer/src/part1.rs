use std::collections::BTreeMap;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum MapType {
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct Almanac {
    maps: BTreeMap<MapType, Vec<Map>>,
}

impl Almanac {
    pub fn from_str(input: &[&str]) -> Self {
        let mut entries = input.iter();
        let mut maps: BTreeMap<MapType, Vec<Map>> = BTreeMap::new();

        if let Some(map_input) = entries.next() {
            maps.insert(
                MapType::Soil,
                map_input
                    .lines()
                    .skip(1)
                    .map(|line| Map::from_str(line, MapType::Soil))
                    .collect(),
            );
        }
        if let Some(map_input) = entries.next() {
            maps.insert(
                MapType::Fertilizer,
                map_input
                    .lines()
                    .skip(1)
                    .map(|line| Map::from_str(line, MapType::Fertilizer))
                    .collect(),
            );
        }
        if let Some(map_input) = entries.next() {
            maps.insert(
                MapType::Water,
                map_input
                    .lines()
                    .skip(1)
                    .map(|line| Map::from_str(line, MapType::Water))
                    .collect(),
            );
        }
        if let Some(map_input) = entries.next() {
            maps.insert(
                MapType::Light,
                map_input
                    .lines()
                    .skip(1)
                    .map(|line| Map::from_str(line, MapType::Light))
                    .collect(),
            );
        }
        if let Some(map_input) = entries.next() {
            maps.insert(
                MapType::Temperature,
                map_input
                    .lines()
                    .skip(1)
                    .map(|line| Map::from_str(line, MapType::Temperature))
                    .collect(),
            );
        }
        if let Some(map_input) = entries.next() {
            maps.insert(
                MapType::Humidity,
                map_input
                    .lines()
                    .skip(1)
                    .map(|line| Map::from_str(line, MapType::Humidity))
                    .collect(),
            );
        }
        if let Some(map_input) = entries.next() {
            maps.insert(
                MapType::Location,
                map_input
                    .lines()
                    .skip(1)
                    .map(|line| Map::from_str(line, MapType::Location))
                    .collect(),
            );
        }
        Self { maps }
    }

    pub fn get_mapped(&self, nr: i64) -> i64 {
        let mut result = nr;
        self.maps.iter().for_each(|(typ, maps)| {
            for map in maps {
                if map.in_range(result) {
                    result = map.maps_to(result);
                    break;
                }
            }
        });
        result
    }
}

#[derive(Debug)]
struct Map {
    from: i64,
    to: i64,
    diff: i64,
    typ: MapType,
}

impl Map {
    pub fn from_str(input: &str, map_type: MapType) -> Self {
        let mut items = input.split(' ');
        let dest: i64 = items.next().unwrap().parse().unwrap();
        let src: i64 = items.next().unwrap().parse().unwrap();
        let len: i64 = items.next().unwrap().parse().unwrap();
        Self {
            from: src,
            to: src + len - 1,
            diff: dest - src,
            typ: map_type,
        }
    }

    pub fn in_range(&self, seed_nr: i64) -> bool {
        self.from <= seed_nr && seed_nr <= self.to
    }

    pub fn maps_to(&self, seed_nr: i64) -> i64 {
        // seed_nr doesn't overlap
        // source <= seed_nr <= destination
        if self.from <= seed_nr && seed_nr <= self.to {
            seed_nr + self.diff
        } else {
            seed_nr
        }
    }
}

pub fn process(input: &str) -> i64 {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let seeds_line = &split_input[0].split(':').nth(1).unwrap().trim().split(' ');
    let almanac = Almanac::from_str(&split_input[1..]);
    let seeds = seeds_line
        .clone()
        .map(|seed| seed.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut results = Vec::new();
    for seed in seeds {
        results.push(almanac.get_mapped(seed));
    }
    results.into_iter().min().unwrap()
}

pub fn main() {
    let input1 = include_str!("../input.txt");
    let result = process(input1);
    println!("part1: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = process(test_input);
        assert_eq!(35, result);
    }
}
