use std::collections::HashMap;

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

struct Data {
    insns: Vec<u8>,
    map: HashMap<String, (String, String)>,
}

pub struct MapEntry {
    src: String,
    left: String,
    right: String,
}

regex_parser!(parse_map_entry: MapEntry {
    X = r#"^(\w*) = \((\w+), (\w+)\)$"# => 
        |src: String, left: String, right: String| 
          MapEntry{ src, left, right }
});

fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();
    let insns = lines.next()
                     .unwrap()
                     .as_bytes()
                     .into();

    lines.next().unwrap();

    let map = lines
        .map(parse_map_entry)
        .map(|entry| (entry.src, (entry.left, entry.right)))
        .collect();

    Data {
        insns,
        map
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut pos = "AAA";
    let mut steps = 0;

    let mut moves = data.insns.iter().cloned().cycle();

    while pos != "ZZZ" {
        let entry = data.map.get(pos).unwrap();
        match moves.next().unwrap() {
            b'L' => { pos = &entry.0; }
            b'R' => { pos = &entry.1; }
            _ => panic!()
        }
        steps += 1;
    }
    steps
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut poses = data.map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();
    let mut steps = 0;
    let mut moves = data.insns.iter().cloned().cycle();

    while !poses.iter()
             .all(|p| p.ends_with('Z')) {

        let mv =  moves.next().unwrap();
        for pos in &mut poses {
            let entry = data.map.get(*pos).unwrap();
            match mv {
                b'L' => { *pos = &entry.0; }
                b'R' => { *pos = &entry.1; }
                _ => panic!()
            }
        }
        steps += 1;
    }
    steps
}}

#[test]
fn test() {
    let test1 = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    let test2 = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;
    let data1 = parse_input(&test1);
    let data2 = parse_input(&test2);

    assert_eq!(part1(&data1), 2);
    assert_eq!(part1(&data2), 6);

    let test_part2 = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
    assert_eq!(part2(&parse_input(test_part2)), 6);
}

fn main() -> std::io::Result<()>{
    let input = get_input(8)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
