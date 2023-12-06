use std::collections::{HashMap};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Id = usize;

#[derive(Debug)]
struct Map {
    source: String,
    dest: String,
    // src id to (dest id, len)
    map: Vec<(Id, Id, usize)>,
}

impl Map {
    fn lookup(&self, id: Id) -> Id {
        let m = self.map.binary_search_by(
            |item| {
                if item.0 > id {
                    std::cmp::Ordering::Greater
                } else if (item.0 + item.2) <= id {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        match m {
            Ok(idx) => {
                let item = self.map[idx];
                item.1 + (id - item.0)
            }
            Err(_) => {
                id
            }
        }
    }
    // Returns the whole range
    fn lookup_len(&self, id: Id) -> (Id, Id) {
        let m = self.map.binary_search_by(
            |item| {
                if item.0 > id {
                    std::cmp::Ordering::Greater
                } else if (item.0 + item.2) <= id {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        match m {
            Ok(idx) => {
                let item = self.map[idx];
                let offset = id - item.0;
                let new_id = item.1 + offset;
                let count = item.2 - offset; 
                (new_id, count)
            }
            Err(idx) => {
                if idx == self.map.len() {
                    (id, Id::MAX)
                } else {
                    let item = self.map[idx];
                    (id, item.0 - id)
                }
            }
        }
    }
}

struct Data {
    seeds: Vec<Id>,
    maps: HashMap<String, Map>,
}

impl Data {
    // Map from a seed to a different type
    pub fn map_to(&self, target: &str, seed: Id) -> Id {
        let mut cur_type = "seed";
        let mut id = seed;
        while cur_type != target {
            let map = self.maps.get(cur_type).unwrap();
            let new_id = map.lookup(id);
            cur_type = &map.dest;
            id = new_id;
        }
        id
    }
    // Returns (Id, num) pairs
    pub fn map_to_multiple(&self, target: &str, seed: Id, count: Id) -> Vec<(Id, Id)> {
        let mut cur_type = "seed";
        let mut ranges = vec![(seed, count)];
        while cur_type != target {
            let mut new_ranges = Vec::new();
            let map = self.maps.get(cur_type).unwrap();
            for (start, startlen) in ranges {
                let mut id = start;
                let mut len = startlen;
                while id < (start + startlen) {
                    let (new_id, mapped_len) = map.lookup_len(id);
                    let overlap = len.min(mapped_len);
                    new_ranges.push((new_id, overlap));
                    id += overlap;
                    len -= overlap;
                }
            }
            cur_type = &map.dest;
            ranges = new_ranges;
        }
        ranges
    }
}

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();

    // Parse seeds line
    let seeds: Vec<Id>;
    let mut line = lines.next().unwrap();

    {
        assert!(line.starts_with("seeds: "));
        let mut bits = line.split_whitespace();
        bits.next().unwrap(); // Skip seeds
        seeds = bits.map(|s| s.parse().unwrap()).collect();
    }
    assert_eq!(lines.next().unwrap(), "");

    let mut maps = HashMap::new();

    while let Some(line) = lines.next() {
        // Must be header line
        assert!(line.ends_with(" map:"));
        let names = line.split_whitespace().next().unwrap();
        let name_parts = names.split("-to-").collect::<Vec<_>>();
        assert_eq!(name_parts.len(), 2);
        let dest = name_parts[1].to_string();
        let source = name_parts[0].to_string();

        let mut map = Vec::new();
        while let Some(l) = lines.next() {
            if l.is_empty() {
                // End of section
                break;
            }
            let bits: Vec<Id> = l.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            assert_eq!(bits.len(), 3);
            map.push((bits[1], bits[0], bits[2]));
        }
        map.sort();

        let src2 = source.clone();
        maps.insert(src2, Map { source, dest, map });
    }

    Data {
        seeds,
        maps,
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    data.seeds
        .iter()
        .map(|&s| {
            data.map_to("location", s)
        })
        .min()
        .unwrap()
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut min_loc = Id::MAX;

    let mut seed_iter = data.seeds.iter().cloned();
    while let Some(seed) = seed_iter.next() {
        let seed_count = seed_iter.next().unwrap(); 

        let new_locs = data.map_to_multiple("location", seed, seed_count);
        let new_min = new_locs.into_iter()
                .min()
                .unwrap()
                .0;
        min_loc = min_loc.min(new_min);
    }
    min_loc
}}

#[test]
fn test() {
    let tests = r#"seeds: 79 14 55 13

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
    let data = parse_input(&tests);

    assert_eq!(data.map_to("location", 79), 82);
    assert_eq!(data.map_to("location", 14), 43);
    assert_eq!(data.map_to("location", 55), 86);
    assert_eq!(data.map_to("location", 13), 35);

    assert_eq!(part1(&data), 35);
    assert_eq!(part2(&data), 46);
}

fn main() -> std::io::Result<()>{
    let input = get_input(5)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
