use std::collections::{HashMap};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Space {
    Empty,
    Round,
    Square,
}

type Data = Vec<Vec<Space>>;
fn parse_input(input: &str) -> Data {
    input.lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    match c {
                        '.' => Space::Empty,
                        'O' => Space::Round,
                        '#' => Space::Square,
                        _ => panic!(),
                    }
                })
                .collect()
        })
    .collect()
}

fn iter_col(data: &Data, c: usize) -> impl Iterator<Item=Space> + '_ {
    data.iter()
        .map(move |row| row[c])
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut sum = 0;
    for col in 0..data[0].len() {
        let height = data.len();
        let mut start_load = height;
        for (i, space) in iter_col(data, col).enumerate() {
            match space {
                Space::Empty => { }
                Space::Round => {
                    sum += start_load;
                    start_load -= 1;
                }
                Space::Square => {
                    start_load = height - i - 1;
                }
            }
        }
    }
    sum
}}

// Tilt to N and rotate by 90 deg
fn turn(data: &Data) -> Data {
    let mut result = Vec::new();
    for x in 0..data[0].len() {
        let mut row = Vec::new();

        let mut rocks = 0;
        let mut spaces = 0;
        for space in iter_col(data, x) {
            match space {
                Space::Empty => {
                    spaces += 1;
                }
                Space::Round => {
                    rocks += 1;
                }
                Space::Square => {
                    for _ in 0..rocks {
                        row.push(Space::Round);
                    }
                    for _ in 0..spaces {
                        row.push(Space::Empty);
                    }
                    row.push(Space::Square);
                    rocks = 0;
                    spaces = 0;
                }
            }
        }
        for _ in 0..rocks {
            row.push(Space::Round);
        }
        for _ in 0..spaces {
            row.push(Space::Empty);
        }

        row.reverse();
        result.push(row);
    }
    
    result
}

fn cycle(data: &Data) -> Data {
//    println!("Cycle...");
//    print_field(data);
    let mut newdata = turn(data);
//    print_field(&newdata);
    newdata = turn(&newdata);
//    print_field(&newdata);
    newdata = turn(&newdata);
//    print_field(&newdata);
    newdata = turn(&newdata);
//    print_field(&newdata);
//    println!("End cycle");
    newdata
}

#[allow(unused)]
fn print_field(data: &Data) {
    for row in data {
        for c in row {
            match c {
                Space::Empty => {
                    print!(".");
                }
                Space::Round => {
                    print!("O");
                }
                Space::Square => {
                    print!("#");
                }
            }
        }
        println!("");
    }
    println!("");
}

fn load(data: &Data) -> usize {
    let mut sum = 0;
    for (i, row) in data.iter().enumerate() {
        let weight = data.len() - i;
        sum += weight * row.iter().filter(|&s| *s == Space::Round).count();
    }
    sum
}

timeit!{
fn part2(data: &Data) -> usize {
    let mut cache: HashMap<Data, usize> = Default::default();
    let target = 1000000000;
    let mut i = 0;
    let mut curdata = data.clone();
    loop {
        curdata = cycle(&curdata);
//        print_field(&curdata);

        i += 1;
//        println!("After {i}: {}", load(&curdata));

        match cache.get_mut(&curdata) {
            Some(v) => {
//                println!("Found repeat {v} => {i}, val {}", load(&curdata));
                let dist = i - *v;
                if (target - i) % dist == 0 {
                    return load(&curdata);
                }
            }
            None => {
                cache.insert(curdata.clone(), i);
            }
        }
        assert!(i < target);
    }
}}

#[test]
fn test() {
    let tests = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 136);
    assert_eq!(part2(&data), 64);
}

fn main() -> std::io::Result<()>{
    let input = get_input(14)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
