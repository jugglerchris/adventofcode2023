#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Copy, Clone, Eq, PartialEq)]
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
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
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
//    assert_eq!(part2(&data), 0);
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
