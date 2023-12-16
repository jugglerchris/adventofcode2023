use std::collections::HashSet;

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Sq {
    Empty,
    HorizSplit,
    VertSplit,
    Slash,
    BSlash
}

impl From<char> for Sq {
    fn from(value: char) -> Self {
        match value {
            '.' => Sq::Empty,
            '-' => Sq::HorizSplit,
            '|' => Sq::VertSplit,
            '/' => Sq::Slash,
            '\\' => Sq::BSlash,
            _ => panic!()
        }
    }
}

type Data = Vec<Vec<Sq>>;
fn parse_input(input: &str) -> Data {
    input.lines()
         .map(|l| l.chars().map(From::from).collect())
         .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn next_pos((x, y): (usize, usize), width: usize, height: usize, dir: Dir) -> Option<(usize, usize)> {
    match dir {
        Dir::Up => if y > 0 { Some((x, y-1)) } else { None },
        Dir::Down => if y+1 < height { Some((x, y+1)) } else { None },
        Dir::Left => if x > 0 { Some((x-1, y)) } else { None },
        Dir::Right => if x+1 < width { Some((x+1, y)) } else { None },
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    let height = data.len();
    let width = data[0].len();
    let mut visited = HashSet::new();
    visited.insert(((0, 0), Dir::Right));
    let mut beams = vec![((0, 0), Dir::Right)];
    while !beams.is_empty() {
        // List of possible next positions
        let mut new_beams = vec![];
        for ((x, y), dir) in beams {
            use Dir::*;
            use Sq::*;
            let dirs = match (dir, data[y][x]) {
                (_, Empty) => vec![dir],
                (Right|Left, HorizSplit) => vec![dir],
                (Up|Down, VertSplit) => vec![dir],
                (Right|Left, VertSplit) => vec![Up, Down],
                (Down|Up, HorizSplit) => vec![Left, Right],
                (Right, Slash) => vec![Up],
                (Left, Slash) => vec![Down],
                (Up, Slash) => vec![Right],
                (Down, Slash) => vec![Left],
                (Right, BSlash) => vec![Down],
                (Left, BSlash) => vec![Up],
                (Up, BSlash) => vec![Left],
                (Down, BSlash) => vec![Right],
            };
            for d in dirs {
                if let Some(pos) = next_pos((x, y), width, height, d) {
                    if visited.insert((pos, d)) {
                        new_beams.push((pos, d));
                    }
                }
            }
        }
        beams = new_beams;
    }
    visited.into_iter().map(|(pos, _)| pos).collect::<HashSet<(usize, usize)>>().len()
}}
timeit!{
fn part2(_data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 46);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(16)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
