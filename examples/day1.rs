#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Data = String;
fn parse_input(input: &str) -> Data {
    input.into()
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut sum = 0usize;
    for line in data.lines() {
        let digits = line
            .chars()
            .filter(|c| *c >= '0' && *c <= '9')
            .map(|c| (c as usize) - '0' as usize)
            .collect::<Vec<_>>();
        sum += digits[0] * 10 + digits.last().unwrap();
    }
    sum
}}

timeit!{
fn part2(data: &Data) -> usize {
    let mut sum = 0usize;
    let digits = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    for line in data.lines() {
        let mut vals = Vec::new();
        for i in 0..line.len() {
            for (s, v) in &digits {
                if line[i..].starts_with(s) {
                    vals.push(v);
                    break;
                }
            }
        }
        sum += dbg!(vals[0] * 10 + *vals.last().unwrap());
    }
    sum
}}

#[test]
fn test() {
    let test1 = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    let test2 = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    let data1 = parse_input(&test1);
    let data2 = parse_input(&test2);

    assert_eq!(part1(&data1), 142);
    assert_eq!(part2(&data2), 281);
}

fn main() -> std::io::Result<()>{
    let input = get_input(1)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
