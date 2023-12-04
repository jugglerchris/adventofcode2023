use std::collections::HashSet;

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub have: Vec<usize>,
    pub winning: Vec<usize>,
    pub num_winning: usize,
}

impl Card {
    pub fn new(
        id: usize,
        have: Vec<usize>,
        winning: Vec<usize>) -> Card {

        let winning_set = winning.iter().cloned().collect::<HashSet<usize>>();
        let num_winning = have
            .iter()
            .cloned()
            .filter(|n| winning_set.contains(n))
            .count();

        Card {
            id,
            have,
            winning,
            num_winning
        }
    }
}

regex_parser!(parse_card: Card {
    X = r#"^Card\s*(\d+):\s*(.*) \| (.*)"# =>
        |id: usize, winning_str: String, have_str: String | {
            let have = have_str.split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
            let winning = winning_str.split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();

            Card::new(id, have, winning)
        }
});

type Data = Vec<Card>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut sum = 0;
    for card in data {
        let num_wins = card.num_winning;
        if num_wins > 0 {
            sum += 1 << (num_wins - 1);
        }
    }
    sum
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut num_cards = vec![1; data.len()];

    for i in 0..data.len() {
        let card = &data[i];
        let count = num_cards[i];
        let num_wins = card.num_winning;
        if num_wins > 0 {
            for j in (i+1)..=(i+num_wins) {
                num_cards[j] += count;
            }
        }
    }
    num_cards.iter().cloned().sum()
}}

#[test]
fn test() {
    let tests = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 13);
    assert_eq!(part2(&data), 30);
}

fn main() -> std::io::Result<()>{
    let input = get_input(4)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
