#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Symbol(u8);

#[derive(Debug)]
struct PartInfo {
    val: usize,
    x: usize,
    y: usize,
    digits: usize,
}

struct Data {
    symbols: HashMap<(usize, usize), Symbol>,
    parts: Vec<PartInfo>,
}

fn parse_input(input: &str) -> Data {
    let mut symbols = HashMap::new();
    let mut parts = Vec::new();

    let mut part = None;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().cloned().enumerate() {
            if (x == 0 || !c.is_ascii_digit()) && part.is_some() {
                parts.push(part.take().unwrap());
            }
            match c {
                b'.' => {}
                b'0'..=b'9' => {
                    if part.is_none() {
                        part = Some(PartInfo {
                            val: (c - b'0') as usize,
                            x, y,
                            digits: 1
                        });
                    } else {
                        let part = part.as_mut().unwrap();
                        assert!(x == 0 || (part.x == x - part.digits && part.y == y));
                        part.val = (part.val * 10) + (c - b'0') as usize;
                        part.digits += 1;
                    }
                }
                c => {
                    symbols.insert((x, y), Symbol(c));
                }
            }
        }
    }
    if let Some(part) = part {
        parts.push(part);
    }
    Data { symbols, parts }
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut result = 0;
    'parts: for part in &data.parts {
        for y in part.y.saturating_sub(1)..=part.y+1 {
            for x in part.x.saturating_sub(1)..=(part.x + part.digits) {
                if data.symbols.contains_key(&(x, y)) {
                    result += part.val;
                    continue 'parts;
                }
            }
        }
    }
    result
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut sum = 0;
    for ((x, y), symbol) in &data.symbols {
        let x = *x;
        let y = *y;
        if let Symbol(b'*') = symbol {
            let mut adjacent = Vec::new();
            for part in &data.parts {
                if x+1 >= part.x && x <= part.x+part.digits &&
                   y+1 >= part.y && y <= part.y+1 {
                    adjacent.push(part);
                }
            }
            if adjacent.len() == 2 {
                sum += adjacent[0].val * adjacent[1].val;
            }
        }
    }
    sum
}}

#[test]
fn test() {
    let tests = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 4361);
    assert_eq!(part2(&data), 467835);
}

fn main() -> std::io::Result<()>{
    let input = get_input(3)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
