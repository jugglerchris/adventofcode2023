#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

type Data = Vec<Race>;
fn parse_input(input: &str) -> Data {
    let mut lines = input.lines();
    let times = lines.next()
                         .unwrap()
                         .split_whitespace()
                         .skip(1)
                         .map(|s| s.parse().unwrap())
                         .collect::<Vec<usize>>();

    let distances = lines.next()
                         .unwrap()
                         .split_whitespace()
                         .skip(1)
                         .map(|s| s.parse().unwrap())
                         .collect::<Vec<usize>>();

    assert_eq!(lines.next(), None);
    times.into_iter()
         .zip(distances)
         .map(|(time, distance)| Race { time, distance })
         .collect()
}

fn parse_input2(input: &str) -> Race {
    let mut lines = input.lines();
    let time_str = lines.next()
                        .unwrap()
                        .chars()
                        .filter(|&c| '0' <= c && c <= '9')
                        .collect::<String>();
    let dist_str = lines.next()
                        .unwrap()
                        .chars()
                        .filter(|&c| '0' <= c && c <= '9')
                        .collect::<String>();
    let time: usize = time_str.parse().unwrap();
    let distance: usize = dist_str.parse().unwrap();

    assert_eq!(lines.next(), None);

    Race {
        time, distance
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut result = 1;

    for race in data {
        // t = time
        // n = length of press
        // distance = (t - n) * n
        // When matching the given distance d:
        // d = (t-n)*n
        // n^2 - t*n + d = 0
        let t = race.time as f64;
        let d = race.distance as f64 + 0.000000001;

        let cross1 = (t - (t*t - 4.0*d).sqrt()) / 2.0;
        let cross2 = (t + (t*t - 4.0*d).sqrt()) / 2.0;
        let cross1 = cross1.ceil();
        let cross2 = cross2.floor();

        let num_ways = (cross2 - cross1) + 1.0;
        result *= num_ways as usize;
    }

    result
}}
timeit!{
fn part2(race: &Race) -> usize {
    // t = time
    // n = length of press
    // distance = (t - n) * n
    // When matching the given distance d:
    // d = (t-n)*n
    // n^2 - t*n + d = 0
    let t = race.time as f64;
    let d = race.distance as f64 + 0.000000001;

    let cross1 = (t - (t*t - 4.0*d).sqrt()) / 2.0;
    let cross2 = (t + (t*t - 4.0*d).sqrt()) / 2.0;
    let cross1 = cross1.ceil();
    let cross2 = cross2.floor();

    let num_ways = (cross2 - cross1) + 1.0;
    num_ways as usize
}}

#[test]
fn test() {
    let tests = r#"Time:      7  15   30
Distance:  9  40  200"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 288);

    let data2 = parse_input2(&tests);
    assert_eq!(part2(&data2), 71503);
}

fn main() -> std::io::Result<()>{
    let input = get_input(6)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    let data2 = parse_input2(&input);

    // Part 2
    println!("{}", part2(&data2));

    Ok(())
}
