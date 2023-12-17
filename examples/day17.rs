use std::collections::{BinaryHeap, HashMap};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Data = Vec<Vec<u8>>;
fn parse_input(input: &str) -> Data {
    input.lines()
        .map(|l| l.as_bytes().iter().map(|&b| b - b'0').collect())
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Ord, Debug)]
struct CellState {
    cost: usize,
    lastdir: Dir,
    x: usize,
    y: usize,
}

impl PartialOrd for CellState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse compare on cost, so that lowest cost comes out first from
        // the BinaryHeap.
        match other.cost.partial_cmp(&self.cost) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.lastdir.partial_cmp(&other.lastdir) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

fn nextmoves(data: &Data, state: &CellState, min_moves: usize, max_moves: usize) -> Vec<CellState> {
    let mut newstates = Vec::new();
    let width = data[0].len();
    let height = data.len();
    match state.lastdir {
        Dir::Up | Dir::Down => {
            // do left/right moves
            let mut cost = state.cost;
            for i in 1..=max_moves {
                if i > state.x {
                    break;
                }
                let x = state.x - i;
                cost += data[state.y][x] as usize;
                if i >= min_moves {
                    newstates.push(CellState { cost, lastdir: Dir::Left, x, y: state.y });
                }
            }

            cost = state.cost;
            for i in 1..=max_moves {
                let x = state.x + i;
                if x >= width {
                    break;
                }
                cost += data[state.y][x] as usize;
                if i >= min_moves {
                    newstates.push(CellState { cost, lastdir: Dir::Right, x, y: state.y });
                }
            }
        }
        Dir::Left | Dir::Right => {
            // do up/down moves
            let mut cost = state.cost;

            for i in 1..=max_moves {
                if i > state.y {
                    break;
                }
                let y = state.y - i;
                cost += data[y][state.x] as usize;
                if i >= min_moves {
                    newstates.push(CellState { cost, lastdir: Dir::Up, x: state.x, y });
                }
            }

            cost = state.cost;
            for i in 1..=max_moves {
                let y = state.y + i;
                if y >= height {
                    break;
                }
                cost += data[y][state.x] as usize;
                if i >= min_moves {
                    newstates.push(CellState { cost, lastdir: Dir::Down, x: state.x, y });
                }
            }
        }
    }
    newstates
}

fn solve(data: &Data, min_move: usize, max_move: usize) -> usize {
    let mut seen = HashMap::new();
    let mut options = BinaryHeap::new();
    let mut best_cost = usize::MAX;

    let height = data.len();
    let width = data[0].len();
    let target = (width-1, height-1);

    // Initial starting points - we could have entered the top-left corner 
    // from two directions.
    options.push(CellState {
        cost: 0,
        lastdir: Dir::Right,
        x: 0,
        y: 0,
    });
    options.push(CellState {
        cost: 0,
        lastdir: Dir::Down,
        x: 0,
        y: 0,
    });

    while let Some(state) = options.pop() {
        if state.cost > best_cost {
            break;
        }

        for newstate in nextmoves(data, &state, min_move, max_move) {
            let seenkey = (newstate.x, newstate.y, newstate.lastdir);
            let prevseen = seen.get_mut(&seenkey);
            if match prevseen {
                Some(cost) => {
                    // Only process if the new one is lower cost
                    *cost > newstate.cost
                }
                // Do want to process it if we haven't seen it before
                None => true,
            } {
                if (newstate.x, newstate.y) == target {
                    best_cost = best_cost.min(newstate.cost);
                }
                options.push(newstate.clone());
                seen.insert((newstate.x, newstate.y, newstate.lastdir), newstate.cost);
            }
        }
        //print!("\r{}", seen.len());
    }

    best_cost
}

timeit!{
fn part1(data: &Data) -> usize {
    solve(data, 1, 3)
}}
timeit!{
fn part2(data: &Data) -> usize {
    solve(data, 4, 10)
}}

#[test]
fn test() {
    let tests = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 102);
    assert_eq!(part2(&data), 94);
}

fn main() -> std::io::Result<()>{
    let input = get_input(17)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
