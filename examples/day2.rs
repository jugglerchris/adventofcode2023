#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};
use std::str::FromStr;

#[derive(Default, Clone)]
pub struct Round {
    pub blue: usize,
    pub green: usize,
    pub red: usize,
}

pub struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
}

pub enum Handful {
    Blue(usize),
    Red(usize),
    Green(usize)
}

regex_parser!(parse_handful: Handful {
    BLUE = r#"\s*(\d+) blue\s*"# => |count: usize| Handful::Blue(count),
    RED = r#"\s*(\d+) red\s*"# => |count: usize| Handful::Red(count),
    GREEN = r#"\s*(\d+) green\s*"# => |count: usize| Handful::Green(count)
});

impl FromStr for Round {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round: Round = Default::default();
        for handful in s.split(',').map(|s| FromStr::from_str(s).unwrap()) {
            use Handful::*;
            match handful {
                Red(v) => { round.red += v; }
                Green(v) => { round.green += v; }
                Blue(v) => { round.blue += v; }
            }
        }
        Ok(round)
    }
}

regex_parser!(parse_game: Game {
    MAIN = r#"Game (\d+): (.*)"# =>
        |id: usize, rest: String| {
            Game {
                id,
                rounds: rest.split(';').map(|s| FromStr::from_str(s).unwrap()).collect()
            }
        }
});

type Data = Vec<Game>;
fn parse_input(input: &str) -> Data {
    input.lines()
        .map(parse_game)
        .collect()
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut result = 0;
    'game: for game in data {
        for round in &game.rounds {
            if round.red > 12 || round.green > 13 || round.blue > 14 {
                continue 'game;
            }
        }
        result += game.id;
    }
    result
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 8);
    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(2)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
