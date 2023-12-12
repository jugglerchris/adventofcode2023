use std::collections::HashMap;

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

struct Row {
    springs: Vec<Spring>,
    runs: Vec<usize>,
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.springs {
            match s {
                Spring::Operational => write!(f, ".")?,
                Spring::Damaged => write!(f, "#")?,
                Spring::Unknown => write!(f, "?")?,
            }
        }
        write!(f, " ")?;
        for run in &self.runs {
            write!(f, "{},", run)?;
        }
        Ok(())
    }
}

type Data = Vec<Row>;
fn parse_input(input: &str) -> Data {
    let mut result = Vec::new();
    for line in input.lines() {
        let (springs, runs) = line.split_once(' ').unwrap();
        let springs = springs
            .chars()
            .map(|c| match c {
                '?' => Spring::Unknown,
                '#' => Spring::Damaged,
                '.' => Spring::Operational,
                _ => panic!()
            })
            .collect();
        let runs = runs
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        result.push(Row { springs, runs });
    }
    result
}

fn count_matches(row: &Row) -> usize {
    let tot_failed: u32 = row.runs.iter().sum::<usize>() as u32;
    let _tot_ok = row.springs.len() as u32 - tot_failed;
    let num_unknown = row.springs.iter().cloned().filter(|&s| s == Spring::Unknown).count() as u32;
    let num_known_failed = row.springs.iter().cloned().filter(|&s| s == Spring::Damaged).count() as u32;
    let needed_damaged = tot_failed - num_known_failed;

    let mut count = 0;
    'main: for mut repl in 0u64..=((1<<num_unknown)-1) {
        if repl.count_ones() != needed_damaged {
            continue;
        }
        let mut cur_run = 0;
        let mut counts = row.runs.iter();
        for &s in &row.springs {
            let guess = if s == Spring::Unknown {
                let bit = repl & 1;
                repl = repl >> 1;
                if bit == 1 {
                    Spring::Damaged
                } else {
                    Spring::Operational
                }
            } else {
                s
            };
            match guess {
                Spring::Operational => {
                    if cur_run > 0 {
                        match (cur_run, counts.next()) {
                            (_, None) => { continue 'main; }
                            (a, Some(&b)) => {
                                if a != b {
                                    continue 'main;
                                }
                            }
                        }
                    }
                    cur_run = 0;
                }
                Spring::Damaged => {
                    cur_run += 1;
                }
                Spring::Unknown => unreachable!()
            }
        }
        // Check if a current run matches
        if cur_run > 0 {
            let next_run = counts.next();
            if let Some(&r) = next_run {
                if r != cur_run {
                    continue 'main;
                }
            } else {
                continue 'main;
            }
        }
        if !counts.next().is_none() {
            continue 'main;
        }
        count += 1;
    }
    count
}

timeit!{
fn part1(data: &Data) -> usize {
    data.iter()
        .map(count_matches)
        .sum()
}}
timeit!{
fn part1_2(data: &Data) -> usize {
    data.iter()
        .map(count_matches2)
        .sum()
}}

fn expand_part2(row: &Row) -> Row {
    let mut springs = row.springs.clone();
    for _ in 0..4 {
        springs.push(Spring::Unknown);
        springs.extend_from_slice(&row.springs);
    }
    let mut runs = row.runs.clone();
    for _ in 0..4 {
        runs.extend_from_slice(&row.runs);
    }
    Row {
        springs, runs
    }
}

fn to_bits<I: Iterator<Item = bool>>(i: I) -> u128 {
    let mut result = 0u128;
    for b in i {
        result <<= 1;
        if b {
            result |= 1;
        }
    }
    result
}

type Cache<'a> = HashMap<(&'a [usize], u128, u128, u128), usize>;

fn count_placings<'c, 'a:'c>(cache: &'c mut Cache<'a>, runs: &'a [usize], set: u128, maybe: u128, clear: u128) -> usize {
    if let Some(result) = cache.get(&(runs, set, maybe, clear)) {
        return *result;
    }
//    eprintln!("count_placings({runs:?}, {set:b}, {maybe:b}, {clear:b}");
    if runs.len() == 0 {
//        eprintln!("Return early");
        return if set == 0 { 1 } else { 0 };
    }
    let next_piece = runs[0];
    let piece_bits = (1u128 << next_piece) - 1;
    let num_bits = 128 - maybe.leading_zeros();
//    dbg!((next_piece, piece_bits, num_bits));

    // Minimum space to the right including gaps
    let mut first_pos: u32 = runs[1..].iter().map(|&n| (n as u32) +1).sum();

    let num_bits_known_set = 128 - set.leading_zeros();
//    dbg!((num_bits_known_set, next_piece));
    if num_bits_known_set >= next_piece as u32{
        first_pos = first_pos.max(num_bits_known_set - next_piece as u32)
    }

    if num_bits < next_piece as u32 {
        return 0;
    }
    let last_pos = num_bits - next_piece as u32;

    let mut count = 0;
//    dbg!((first_pos, last_pos));

    for shift in first_pos..=last_pos {
        let shifted = piece_bits << shift;

        if (shifted & clear) != 0 {
            // Can't overlap any clear bits
            continue;
        }

        // The following bit must not be set.
        if shift > 0 && ((1<<(shift-1)) & set != 0) {
            continue;
        }

        let next_mask = if shift == 0 {
            0
        } else {
            (1 << (shift -1)) - 1
        };
        count += count_placings(cache, &runs[1..],
                                set & next_mask,
                                maybe & next_mask,
                                clear & next_mask);
    }
//    dbg!(count)
    cache.insert((runs, set, maybe, clear), count);
    count
}

fn count_matches2(row: &Row) -> usize {
    eprintln!("{row} len {}", row.springs.len());
    assert!(row.springs.len() <= 128);

    // Useful bitsets
    let bits_damaged = to_bits(
        row.springs
           .iter()
           .map(|&s| s == Spring::Damaged));
    let bits_maybedamaged = to_bits(
        row.springs
           .iter()
           .map(|&s| s != Spring::Operational));
    let bits_clear = to_bits(
        row.springs
           .iter()
           .map(|&s| s == Spring::Operational));

    let mut cache: Cache = HashMap::new();
    let result = count_placings(&mut cache, &row.runs, bits_damaged, bits_maybedamaged, bits_clear);
//    dbg!(result)
    result
}

timeit!{
fn part2(data: &Data) -> usize {
    data.iter()
        .map(expand_part2)
        .map(|r| count_matches2(&r))
        .sum()
}}

#[test]
fn test() {
    let tests = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 21);
    assert_eq!(part1_2(&data), 21);
    assert_eq!(part2(&data), 525152);
}

fn main() -> std::io::Result<()>{
    let input = get_input(12)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
