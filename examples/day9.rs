#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Data = Vec<Vec<isize>>;
fn parse_input(input: &str) -> Data {
    input.lines()
         .map(|l| {
             l.split_whitespace()
              .map(|s| s.parse().unwrap())
              .collect()
         })
         .collect()
}

timeit!{
fn part1(data: &Data) -> isize {
    let mut sum = 0;
    for seq in data {
        let mut derivs = vec![seq.clone()];
        // Calculate the derivatives
        loop {
            let mut vals = Vec::new();
            for pair in derivs.last().unwrap().windows(2) {
                vals.push(pair[1] - pair[0]);
            }
            if vals.iter().all(|v| *v == 0) {
                break;
            } else {
                derivs.push(vals);
            }
        }
        // Now extrapolate
        let mut inc = 0;
        while let Some(mut v) = derivs.pop() {
            inc = v.pop().unwrap() + inc;
        }
        sum += inc;
    }
    sum
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 114);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(9)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
