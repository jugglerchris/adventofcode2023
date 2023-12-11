use std::collections::HashSet;

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Data = HashSet<(usize, usize)>;
fn parse_input(input: &str) -> Data {
    let mut result = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.chars().enumerate() {
            if b == '#' {
                result.insert((x, y));
            }
        }
    }
    result
}

fn dist((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    let mut d = 0;
    if x1 > x2 {
        d += x1 - x2;
    } else {
        d += x2 - x1;
    }
    if y1 > y2 {
        d += y1 - y2;
    } else {
        d += y2 - y1;
    }
    d
}

fn with_expansion(data: &Data, exp: usize) -> usize {
    let mut row_map = Vec::new();
    let mut col_map = Vec::new();

    let x_values =
        data.iter()
            .map(|p| p.0)
            .collect::<HashSet<usize>>();
    let y_values =
        data.iter()
            .map(|p| p.1)
            .collect::<HashSet<usize>>();
    let x_max = x_values.iter().cloned().max().unwrap();
    let y_max = y_values.iter().cloned().max().unwrap();

    {
        let mut x = 0usize;
        for x_in in 0..=x_max {
            col_map.push(x);
            if x_values.contains(&x_in) {
                x += 1;
            } else {
                // Expansion
                x += exp;
            }
        }
    }
    {
        let mut y = 0usize;
        for y_in in 0..=y_max {
            row_map.push(y);
            if y_values.contains(&y_in) {
                y += 1;
            } else {
                // Expansion
                y += exp;
            }
        }
    }
    let mut sum = 0;
    for g1 in data.iter() {
        for g2 in data.iter() {
            if g1.0 > g2.0 || (g1.0 == g2.0 && g1.1 > g2.1) {
                let x1 = col_map[g1.0];
                let y1 = row_map[g1.1];
                let x2 = col_map[g2.0];
                let y2 = row_map[g2.1];
                sum += dist((x1, y1), (x2, y2));
            }
        }
    }
    sum
}

timeit!{
fn part1(data: &Data) -> usize {
    with_expansion(data, 2)
}}

timeit!{
fn part2(data: &Data) -> usize {
    with_expansion(data, 1000000)
}}

#[test]
fn test() {
    let tests = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 374);
    assert_eq!(with_expansion(&data, 10), 1030);
    assert_eq!(with_expansion(&data, 100), 8410);
}

fn main() -> std::io::Result<()>{
    let input = get_input(11)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
