#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Data<'a> = Vec<&'a [u8]>;
fn parse_input(input: &str) -> Data<'_> {
    input.trim().as_bytes().split(|&b| b == b',').collect()
}

fn hash(s: &[u8]) -> u8 {
    let mut result = 0u16;
    for &b in s {
        result += b as u16;
        result *= 17;
        result &= 0xff;
    }
    result as u8
}

timeit!{
fn part1(data: &Data) -> usize {
    data.iter()
        .map(|s| hash(s) as usize)
        .sum()
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    assert_eq!(hash(b"HASH"), 52);
    let tests = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 1320);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(15)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
