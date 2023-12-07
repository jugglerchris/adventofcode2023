#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

fn card_value(card: char) -> u8 {
    match card {
        '0'..='9' => (card as u8) - b'0',
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card [{card}]"),
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandValue {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Ord, Eq, Clone)]
struct Hand {
    hand_value: HandValue,
    cards: [u8;5],

    bid: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_value == other.hand_value && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_value.partial_cmp(&other.hand_value) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}

type Data = Vec<Hand>;
fn parse_input(input: &str) -> Data {
    let mut result = Vec::new();

    for line in input.lines() {
        let (cardstr, bidstr) = line.split_once(' ').unwrap();
        let bid = bidstr.parse().unwrap();

        let mut cards = [0u8;5];
        for (i, card) in cardstr.chars().map(card_value).enumerate() {
            cards[i] = card;
        }

        let hand_value = calc_hand_value(&cards);

        result.push(Hand {
            hand_value, cards, bid
        });
    }

    result
}

fn calc_hand_value(cards: &[u8; 5]) -> HandValue {
    let mut counts = [0u8; 256];

    for &v in cards {
        counts[v as usize] += 1;
    }

    let mut countv = counts.into_iter()
                           .filter(|&c| c>0)
                           .collect::<Vec<_>>();
    countv.sort();
    match &countv[..] {
        &[5] => HandValue::Five,
        &[1, 4] => HandValue::Four,
        &[2, 3] => HandValue::FullHouse,
        &[1, 1, 3] => HandValue::Three,
        &[1, 2, 2] => HandValue::TwoPair,
        &[1, 1, 1, 2] => HandValue::Pair,
        &[1, 1, 1, 1, 1] => HandValue::HighCard,
        _ => panic!("Invalid hand: {:?}", &countv),
    }
}

fn calc_hand_value_2(cards: &[u8; 5]) -> HandValue {
    let mut counts = [0u8; 256];

    for &v in cards {
        counts[v as usize] += 1;
    }

    // Keep the jokers back
    let joker_index = card_value('J') as usize;
    let num_jokers = counts[joker_index];
    counts[joker_index] = 0;

    let mut countv = counts.into_iter()
                           .filter(|&c| c>0)
                           .collect::<Vec<_>>();
    countv.sort();
    if countv.len() == 0 {
        countv.push(num_jokers);
    } else {
        *countv.last_mut().unwrap() += num_jokers;
    }
    match &countv[..] {
        &[5] => HandValue::Five,
        &[1, 4] => HandValue::Four,
        &[2, 3] => HandValue::FullHouse,
        &[1, 1, 3] => HandValue::Three,
        &[1, 2, 2] => HandValue::TwoPair,
        &[1, 1, 1, 2] => HandValue::Pair,
        &[1, 1, 1, 1, 1] => HandValue::HighCard,
        _ => panic!("Invalid hand: {:?}", &countv),
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut data = data.iter().cloned().collect::<Vec<_>>();
    data.sort();
    data.into_iter()
        .enumerate()
        .map(|(i, hand)| {
            (i+1) * hand.bid
        })
        .sum()
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut data = data.iter().cloned().collect::<Vec<_>>();

    // Update the hand values
    for hand in &mut data {
        // Update the hand value
        hand.hand_value = calc_hand_value_2(&hand.cards);
        // Update Joker values
        for v in &mut hand.cards {
            if *v == 11 {
                *v = 0;
            }
        }
    }
    data.sort();
    data.into_iter()
        .enumerate()
        .map(|(i, hand)| {
            (i+1) * hand.bid
        })
        .sum()
}}

#[test]
fn test() {
    let tests = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 6440);
    assert_eq!(part2(&data), 5905);
}

fn main() -> std::io::Result<()>{
    let input = get_input(7)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
