use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum ValueMap<T> {
    Seed(T),
    Soil(T),
    Fertilizer(T),
    Water(T),
    Light(T),
    Temperature(T),
    Humidity(T),
    Location(T),
}

impl<T> ValueMap<T> {
    fn next(self) -> ValueMap<T> {
        match self {
            ValueMap::Seed(value) => ValueMap::Soil(value),
            ValueMap::Soil(value) => ValueMap::Fertilizer(value),
            ValueMap::Fertilizer(value) => ValueMap::Water(value),
            ValueMap::Water(value) => ValueMap::Light(value),
            ValueMap::Light(value) => ValueMap::Temperature(value),
            ValueMap::Temperature(value) => ValueMap::Humidity(value),
            ValueMap::Humidity(value) => ValueMap::Location(value),
            _ => panic!("You should not be here"),
        }
    }

    fn replace(&self, val: T) -> Self {
        match self {
            ValueMap::Seed(_) => ValueMap::Seed(val),
            ValueMap::Soil(_) => ValueMap::Soil(val),
            ValueMap::Fertilizer(_) => ValueMap::Fertilizer(val),
            ValueMap::Water(_) => ValueMap::Water(val),
            ValueMap::Light(_) => ValueMap::Light(val),
            ValueMap::Temperature(_) => ValueMap::Temperature(val),
            ValueMap::Humidity(_) => ValueMap::Humidity(val),
            ValueMap::Location(_) => ValueMap::Location(val),
        }
    }

    fn find_type(&self) -> Option<MappingType> {
        match self {
            ValueMap::Seed(_) => Some(MappingType::SeedToSoil),
            ValueMap::Soil(_) => Some(MappingType::SoilToFertilizer),
            ValueMap::Fertilizer(_) => Some(MappingType::FertilizerToWater),
            ValueMap::Water(_) => Some(MappingType::WaterToLight),
            ValueMap::Light(_) => Some(MappingType::LightToTemperature),
            ValueMap::Temperature(_) => Some(MappingType::TemperatureToHumidity),
            ValueMap::Humidity(_) => Some(MappingType::HumidityToLocation),
            ValueMap::Location(_) => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum MappingType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl FromStr for MappingType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed-to-soil" => Ok(Self::SeedToSoil),
            "soil-to-fertilizer" => Ok(Self::SoilToFertilizer),
            "fertilizer-to-water" => Ok(Self::FertilizerToWater),
            "water-to-light" => Ok(Self::WaterToLight),
            "light-to-temperature" => Ok(Self::LightToTemperature),
            "temperature-to-humidity" => Ok(Self::TemperatureToHumidity),
            "humidity-to-location" => Ok(Self::HumidityToLocation),
            _ => panic!("Unknown mapping type"),
        }
    }
}

type Value = usize;

#[derive(Debug)]
struct Mapping {
    destination_range_start: usize,
    source_range_start: usize,
    length: usize,
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split_whitespace();
        let destination_range_start = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let source_range_start = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let length = parts.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(Self {
            destination_range_start,
            source_range_start,
            length,
        })
    }
}

impl Mapping {
    fn map_value(&self, value: Value) -> Option<Value> {
        if value < self.source_range_start || value >= self.source_range_start + self.length {
            return None;
        }

        let offset = value - self.source_range_start;
        if offset >= self.length {
            return None;
        }

        let destination = self.destination_range_start + offset;
        Some(destination)
    }

    fn map(&self, value: ValueMap<Value>) -> Option<ValueMap<Value>> {
        if let ValueMap::Location(_) = value {
            return None;
        }

        let val = match value {
            ValueMap::Seed(value) => ValueMap::Seed(self.map_value(value)?),
            ValueMap::Soil(value) => ValueMap::Soil(self.map_value(value)?),
            ValueMap::Fertilizer(value) => ValueMap::Fertilizer(self.map_value(value)?),
            ValueMap::Water(value) => ValueMap::Water(self.map_value(value)?),
            ValueMap::Light(value) => ValueMap::Light(self.map_value(value)?),
            ValueMap::Temperature(value) => ValueMap::Temperature(self.map_value(value)?),
            ValueMap::Humidity(value) => ValueMap::Humidity(self.map_value(value)?),
            _ => panic!("You should not be here"),
        };

        let next = val.next();
        Some(next)
    }
}

type MappingTuple = (MappingType, Vec<Mapping>);

struct Map {
    mappings: [MappingTuple; 7],
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: Vec<MappingTuple> = Vec::new();
        let maps_s = s.split("\n\n");

        for mapping_with_title in maps_s {
            let mut mapping = mapping_with_title.split(" map:\n");
            let title = mapping.next().expect("Title not found").parse()?;

            let mappings = mapping
                .next()
                .expect("Mappings not found")
                .lines()
                .filter_map(|v| Mapping::from_str(v).ok())
                .collect::<Vec<Mapping>>();

            let tuple = (title, mappings);
            result.push(tuple);
        }

        if result.len() != 7 {
            println!("Wrong number of maps");
            return Err(());
        }

        Ok(Self {
            mappings: [
                result.pop().ok_or(())?,
                result.pop().ok_or(())?,
                result.pop().ok_or(())?,
                result.pop().ok_or(())?,
                result.pop().ok_or(())?,
                result.pop().ok_or(())?,
                result.pop().ok_or(())?,
            ],
        })
    }
}

impl Map {
    fn map(&self, value: ValueMap<Value>) -> Option<ValueMap<Value>> {
        let next_map_type = value.find_type()?;
        let mut next_value_map: Vec<ValueMap<Value>> = self
            .mappings
            .iter()
            .filter(|(t, _)| t == &next_map_type)
            .flat_map(|(_, ms)| ms)
            .filter_map(|m| m.map(value))
            .collect();

        if next_value_map.len() == 0 {
            return Some(value.next());
        }

        if next_value_map.len() > 1 {
            panic!("More than one next value map");
        }

        let next = next_value_map.pop();
        next
    }
}

#[derive(Debug, PartialEq)]
enum UpdateRange {
    Unchanged,
    Moved(SeedRange),
    Split {
        unmoved: SeedRange,
        moved: SeedRange,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct SeedRange {
    start: usize,
    end: usize,
}

#[derive(Debug, Copy, Clone)]
struct SeedRangeMap {
    src: SeedRange,
    dst: SeedRange,
}

impl SeedRange {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn with_length(start: usize, length: usize) -> Self {
        let end = start + length - 1;
        Self::new(start, end)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.start > other.end || self.end < other.start {
            return None;
        }

        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        Some(Self::new(start, end))
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn shift_right(&self, offset: usize) -> Self {
        Self::new(self.start + offset, self.end + offset)
    }

    fn shift_left(&self, offset: usize) -> Self {
        Self::new(self.start - offset, self.end - offset)
    }

    fn shift(&self, offset: isize) -> Self {
        if offset < 0 {
            self.shift_left(offset.abs() as usize)
        } else {
            self.shift_right(offset as usize)
        }
    }
}

impl From<&Mapping> for SeedRange {
    fn from(value: &Mapping) -> Self {
        Self::with_length(value.source_range_start, value.length)
    }
}

impl From<&Mapping> for SeedRangeMap {
    fn from(value: &Mapping) -> Self {
        Self {
            src: value.into(),
            dst: SeedRange::with_length(value.destination_range_start, value.length),
        }
    }
}

impl Default for SeedRange {
    fn default() -> Self {
        Self::new(usize::MAX, usize::MAX)
    }
}

struct AlmanacRanges {
    mappings: Vec<(MappingType, Vec<SeedRangeMap>)>,
    seeds: Vec<SeedRange>,
}

impl AlmanacRanges {
    fn new(mut almanac: Almanac) -> Self {
        let mut seeds = Vec::new();
        let mut almanac_seeds = std::mem::take(&mut almanac.seeds);
        almanac_seeds.reverse();

        while almanac_seeds.len() > 0 {
            let seed = almanac_seeds.pop().expect("No seed found");
            let range = almanac_seeds.pop().expect("No range found");

            let seed_range = SeedRange::with_length(seed, range);
            seeds.push(seed_range);
        }

        let mappings = almanac
            .maps
            .mappings
            .into_iter()
            .map(|(t, m)| (t, m.into_iter().map(|m| SeedRangeMap::from(&m)).collect()))
            .collect();

        Self { mappings, seeds }
    }

    fn apply_map_on_seed_range(
        range: &SeedRange,
        SeedRangeMap { dst, src }: &SeedRangeMap,
    ) -> UpdateRange {
        if src.contains(&range) {
            UpdateRange::Moved(range.shift(dst.start as isize - src.start as isize))
        } else if let Some(intersection) = src.intersection(&range) {
            let moved = intersection.shift(dst.start as isize - src.start as isize);
            let unmoved = if range.start < intersection.start {
                SeedRange::new(range.start, intersection.start - 1)
            } else {
                SeedRange::new(intersection.end + 1, range.end)
            };
            UpdateRange::Split { unmoved, moved }
        } else {
            UpdateRange::Unchanged
        }
    }

    fn update_ranges(
        seed_range_from: SeedRange,
        seed_range_map: &Vec<(MappingType, Vec<SeedRangeMap>)>,
    ) -> Vec<SeedRange> {
        let seed = ValueMap::Seed(seed_range_from);
        let mut next_seed_ranges = vec![seed];
        let mut result_seeds = Vec::new();

        while let Some(seed_range) = next_seed_ranges.pop() {
            let seed = match seed_range {
                ValueMap::Seed(v) => v,
                ValueMap::Soil(v) => v,
                ValueMap::Fertilizer(v) => v,
                ValueMap::Water(v) => v,
                ValueMap::Light(v) => v,
                ValueMap::Temperature(v) => v,
                ValueMap::Humidity(v) => v,
                ValueMap::Location(v) => {
                    result_seeds.push(v);
                    continue;
                }
            };

            let mappings: &Vec<SeedRangeMap> = seed_range_map
                .into_iter()
                .find(|(t, _)| t == &seed_range.find_type().expect("No mapping found"))
                .map(|(_, ms)| ms)
                .expect("No mapping found");

            let applied_list: Vec<UpdateRange> = mappings
                .iter()
                .map(|m| Self::apply_map_on_seed_range(&seed, &m))
                .collect();

            let seed_range_next = seed_range.next();
            if applied_list.iter().all(|v| v == &UpdateRange::Unchanged) {
                next_seed_ranges.push(seed_range_next);
                continue;
            }

            for applied in applied_list {
                match applied {
                    UpdateRange::Moved(new_seed_range) => {
                        next_seed_ranges.push(seed_range_next.replace(new_seed_range))
                    }
                    UpdateRange::Split { unmoved, moved } => {
                        next_seed_ranges.push(seed_range.replace(unmoved));
                        next_seed_ranges.push(seed_range_next.replace(moved));
                    }
                    UpdateRange::Unchanged => (),
                }
            }
        }
        result_seeds
    }

    fn find_minimal_location(&mut self) -> Value {
        let seeds = std::mem::take(&mut self.seeds);
        let mappings = std::mem::take(&mut self.mappings);

        std::thread::scope(move |s| {
            let mut threads = Vec::new();
            for seed_range in seeds {
                let mapping_clone = mappings.clone();
                let h1 = s.spawn(move || Self::update_ranges(seed_range, &mapping_clone));
                threads.push(h1);
            }

            threads
                .into_iter()
                .map(|h| h.join().unwrap())
                .fold(Vec::new(), |mut acc, mut v| {
                    acc.append(&mut v);
                    acc
                })
        })
        .iter()
        .fold(usize::MAX, |acc, v| acc.min(v.start))
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Map,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.splitn(2, "\n\n");
        let seeds = lines
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse seed"))
            .collect();
        let maps = lines.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Self { seeds, maps })
    }
}

impl Almanac {
    fn find_locations(&self) -> Vec<Value> {
        let mut locations = Vec::new();
        for seed in &self.seeds {
            let mut value = ValueMap::Seed(*seed);
            while let Some(next) = self.maps.map(value) {
                value = next;
            }
            if let ValueMap::Location(location) = value {
                locations.push(location);
            }
        }
        locations
    }

    fn find_minimal_location(&self) -> Value {
        *self.find_locations().iter().min().unwrap()
    }
}

fn part1(input: &str) -> usize {
    input.parse::<Almanac>().unwrap().find_minimal_location()
}

fn part2(input: &str) -> usize {
    let almanac = input.parse::<Almanac>().unwrap();
    AlmanacRanges::new(almanac).find_minimal_location()
}

pub fn _day() {
    let input = include_str!("../input/day5.txt");
    println!(
        "Day 5\t Part 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

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
56 93 4"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_mapping() {
        let mapping = "50 98 2".parse::<Mapping>().unwrap();
        assert_eq!(mapping.map_value(53), None);
        assert_eq!(mapping.map_value(99), Some(51));
        let mapping = "52 50 48".parse::<Mapping>().unwrap();
        assert_eq!(mapping.map_value(79), Some(81));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 46);
    }
}
