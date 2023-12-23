use std::{collections::{BTreeSet, HashMap}, fmt::Debug};

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

#[derive(Clone)]
struct Data {
    field: Vec<Vec<Space>>,
}
impl Data {
    pub fn next_pos(&self, state: &WalkState) -> Vec<(usize, usize)> {
        let (x, y) = state.pos;
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
        result.into_iter()
            .filter(|pos| !state.path.contains(pos) && *pos != state.lastpos)
            .collect()
    }
    pub fn next_pos_from(&self, prev: Pos, next: Pos) -> Vec<(usize, usize)> {
        let (x, y) = next;
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
        result.into_iter()
            .filter(|pos| *pos != prev)
            .collect()
    }

    fn get(&self, x: usize, y: usize) -> Space {
        if y >= self.field.len() || x >= self.field[y].len() {
            Space::Wall
        } else {
            self.field[y][x]
        }
    }

    fn clear_slopes(&mut self) {
        for row in &mut self.field {
            for b in row {
                match b {
                    Space::Empty |
                    Space::Wall => (),
                    Space::SlopeN |
                    Space::SlopeE |
                    Space::SlopeS |
                    Space::SlopeW => {
                        *b = Space::Empty;
                    }
                }
            }
        }
    }

    #[allow(unused)]
    fn print(&self) {
        for row in &self.field {
            for spc in row {
                print!("{}", match spc {
                    Space::Empty => '.',
                    Space::Wall => '#',
                    Space::SlopeN => '^',
                    Space::SlopeE => '>',
                    Space::SlopeS => 'v',
                    Space::SlopeW => '<',
                });
            }
            println!("");
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
    lastpos: (usize, usize),
}

fn do_part1(data: &Data) -> usize {
    let target = (data.field[0].len() - 2, data.field.len() - 1);
    let mut paths = vec![
        WalkState {
            steps: 0,
            path: [(1usize, 0usize)].into(),
            pos: (1, 0),
            lastpos: (0, 0),
        }
    ];
    let mut max_walk = 0;

    while let Some(state) = paths.pop() {
        if state.pos == target {
            max_walk = max_walk.max(state.steps);
            continue;
        }
        let next_positions = data.next_pos(&state);
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
            for nextpos in data.next_pos(&state) {
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
    do_part2(data)
}}

type Pos = (usize, usize);

type GPos = usize;

struct Graph {
    start: GPos,
    finish: GPos,
    nodes: Vec<Vec<(GPos, usize)>>,
}

impl Graph {
    pub fn new(data: &Data) -> Graph {
        // pos, next_pos
        let mut to_visit = vec![((1, 0), (1, 1))];

        let mut nodes = vec![Vec::new()];
        let mut nodemap = HashMap::from([((1, 0), 0)]);

        while let Some((start, mut next)) = to_visit.pop() {
            let mut next_poses = data.next_pos_from(start, next);
            let mut len = 1;
            while next_poses.len() == 1 {
                // In a corridor, keep stepping
                let prev = next;
                next = next_poses[0];
                next_poses = data.next_pos_from(prev, next);
                len += 1;
            }
            if !nodemap.contains_key(&next) {
                nodemap.insert(next, nodes.len());
                nodes.push(Vec::new());
                // New node, so start a new trace.
                for next_pos in next_poses {
                    to_visit.push((next, next_pos));
                }
            }
            let pos0 = *nodemap.get(&start).unwrap();
            let pos1 = *nodemap.get(&next).unwrap();
            nodes[pos0].push((pos1, len));
            nodes[pos1].push((pos0, len));
        }

        Graph {
            start: 0,
            finish: *nodemap.get(&(data.field[0].len() - 2, data.field.len() - 1)).unwrap(),
            nodes
        }
    }

    fn do_solve(&self, seen: u64, cache: &mut HashMap<(u64, usize), Option<usize>>, pos: usize) -> Option<usize> {
        if pos == self.finish {
            return Some(0);
        }
        if let Some(v) = cache.get(&(seen, pos)) {
            return v.clone();
        }

        let mut best = None;
        for &(otherpos, len) in &self.nodes[pos] {
            if seen & (1<<otherpos) != 0 {
                continue;
            }
            if let Some(result) = self.do_solve(seen | (1<<otherpos), cache, otherpos) {
                if result + len > best.unwrap_or(0) {
                    best = Some(result + len);
                }
            }
        }

        cache.insert((seen, pos), best.clone());
        best
    }

    pub fn solve(&mut self) -> usize {
        assert!(self.nodes.len() <= 64);
        if let Some(best) = self.do_solve(0, &mut Default::default(), self.start) {
            best
        } else {
            panic!();
        }
    }
}

fn do_part2(data: &Data) -> usize {
    let mut data = data.clone();
    data.clear_slopes();

    let mut graph = Graph::new(&data);
    graph.solve()
}

#[allow(unused)]
fn do_print(state: &WalkState, data: &Data) {
    println!("---");
    for (y, row) in data.field.iter().enumerate() {
        for (x, spc) in row.iter().enumerate() {
            print!("{}", match spc {
                Space::Empty => {
                    if state.path.contains(&(x, y)) {
                        'O'
                    } else {
                        '.'
                    }
                }
                Space::Wall => '#',
                Space::SlopeN => '^',
                Space::SlopeE => '>',
                Space::SlopeS => 'v',
                Space::SlopeW => '<',
            });
        }
        println!("");
    }
    
}

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
    assert_eq!(part2(&data), 154);
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
