use std::collections::{BTreeSet};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Copy, Clone, Debug)]
enum Space {
    Empty,
    Wall,
    SlopeN,
    SlopeE,
    SlopeS,
    SlopeW,
}

impl Space {
    pub fn is_empty(&self) -> bool {
        match self {
            Space::SlopeN |
            Space::SlopeE |
            Space::SlopeS |
            Space::SlopeW |
            Space::Empty => true,
            Space::Wall => false,
        }
    }
}

struct Data {
    field: Vec<Vec<Space>>,
}
impl Data {
    pub fn next_pos(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        // If on a slope, only one choice.
        match self.get(x, y) {
            Space::Empty => {}
            Space::Wall => panic!(),
            Space::SlopeN => {
                return vec![(x, y-1)];
            }
            Space::SlopeE => {
                return vec![(x+1, y)];
            }
            Space::SlopeS => {
                return vec![(x, y+1)];
            }
            Space::SlopeW => {
                return vec![(x-1, y)];
            }
        }

        let mut result = Vec::new();
        if y > 0 && self.get(x, y-1).is_empty() {
            result.push((x, y-1));
        }
        if x > 0 && self.get(x-1, y).is_empty() {
            result.push((x-1, y));
        }
        if self.get(x+1, y).is_empty() {
            result.push((x+1, y));
        }
        if self.get(x, y+1).is_empty() {
            result.push((x, y+1));
        }
        result
    }

    fn get(&self, x: usize, y: usize) -> Space {
        if y >= self.field.len() || x >= self.field[y].len() {
            Space::Wall
        } else {
            self.field[y][x]
        }
    }
}

fn parse_input(input: &str) -> Data {
    let field = input.lines()
        .map(|l| {
            l.chars()
             .map(|c| match c {
                 '.' => Space::Empty,
                 '#' => Space::Wall,
                 '>' => Space::SlopeE,
                 '<' => Space::SlopeW,
                 '^' => Space::SlopeN,
                 'v' => Space::SlopeS,
                 _ => panic!(),
             })
             .collect()
        })
        .collect();
    Data {
        field
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    do_part1(data)
}}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct WalkState {
    steps: usize,
    path: BTreeSet<(usize, usize)>,
    pos: (usize, usize),
}

fn do_part1(data: &Data) -> usize {
    let target = (data.field[0].len() - 2, data.field.len() - 1);
    let mut paths = vec![
        WalkState {
            steps: 0,
            path: [(1usize, 0usize)].into(),
            pos: (1, 0),
        }
    ];
    let mut max_walk = 0;

    while let Some(state) = paths.pop() {
        if state.pos == target {
            max_walk = max_walk.max(state.steps);
            continue;
        }
        let next_positions = data.next_pos(state.pos);
        if next_positions.len() == 1 {
            // Avoid clone in the common case
            let nextpos = next_positions[0];
            let mut state = state;
            if !state.path.contains(&nextpos) {
                state.pos = nextpos;
                state.path.insert(nextpos);
                state.steps += 1;
                paths.push(state);
            }
        } else {
            for nextpos in data.next_pos(state.pos) {
                if !state.path.contains(&nextpos) {
                    let mut new_state = state.clone();
                    new_state.pos = nextpos;
                    new_state.path.insert(nextpos);
                    new_state.steps += 1;
                    paths.push(new_state);
                }
            }
        }
    }

    max_walk
}

timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 94);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(23)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
