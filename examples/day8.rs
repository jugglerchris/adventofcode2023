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

// Returns number of steps and ending point
fn steps_to<'a>(data: &'a Data, start_step: usize, start: &'a str) -> (usize, &'a str) {
    let mut pos = start;
    let mut steps = start_step;
    let mut moves = data.insns.iter().cloned().cycle().skip(start_step);
    loop {
        let entry = data.map.get(pos).unwrap();
        let mv = moves.next().unwrap();
        match mv {
            b'L' => { pos = &entry.0; }
            b'R' => { pos = &entry.1; }
            _ => panic!()
        }
        steps += 1;
        if pos.ends_with('Z') {
            break;
        }
    }
    (steps, pos)
}

fn lcm(a: usize, b: usize) -> usize {
    let g = adventofcode2023::gcd(a, b);
    (a / g) * b
}

timeit!{
fn part2(data: &Data) -> usize {
    let poses = data.map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| s.as_str())
        .collect::<Vec<&str>>();

    let mut first_hits = Vec::new();
    let mut repeat = Vec::new();
    for pos in poses {
        let (first, endpos) = steps_to(data, 0, pos);
        let (second, _) = steps_to(data, first, endpos);
        first_hits.push(first);
        repeat.push(second - first);
    }
    // It turns out that they all repeat at the ends of the list of
    // instructions, so we don't need to do any complicated CRT or
    // anything.
    repeat.into_iter()
          .fold(1, |a, b| lcm(a, b))
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
