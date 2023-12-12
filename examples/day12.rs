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
fn part2(_data: &Data) -> usize {
    unimplemented!()
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
//    assert_eq!(part2(&data), 0);
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
