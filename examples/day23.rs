use std::{collections::{BTreeSet, HashSet, HashMap}, fmt::Debug};

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

#[derive(Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Edge(Pos, Pos, usize);

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge(({},{})->({},{}) len {})",
            self.0.0, self.0.1,
            self.1.0, self.1.1,
            self.2)
    }
}

impl Edge {
    pub fn new(pos1: Pos, pos2: Pos, len: usize) -> Edge {
        if pos1 > pos2 {
            Edge(pos2, pos1, len)
        } else {
            Edge(pos1, pos2, len)
        }
    }
}

struct Graph {
    start: Pos,
    finish: Pos,
    edges: BTreeSet<Edge>,
}

impl Graph {
    pub fn new(data: &Data) -> Graph {
        // pos, next_pos
        let mut to_visit = vec![((1, 0), (1, 1))];
        let mut nodes = HashSet::from([(1, 0)]);
        let mut edges = BTreeSet::new();

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
            edges.insert(Edge::new(start, next, len));
            if !nodes.contains(&next) {
                nodes.insert(next);
                // New node, so start a new trace.
                for next_pos in next_poses {
                    to_visit.push((next, next_pos));
                }
            }
        }

        /*
        {
            let mut data: HashMap<Pos, Vec<Edge>> = HashMap::new();
            for edge in &edges {
                for pos in [edge.0, edge.1] {
                    data.entry(pos)
                        .or_default()
                        .push(*edge);
                }
            }
            let mut vdata = data.into_iter().collect::<Vec<_>>();
            vdata.sort_by_key(|x| x.1.len());
            for (p, es) in vdata {
                println!("({},{}): {} edges", p.0, p.1, es.len());
            }
        }
        */

        Graph {
            start: (1, 0),
            finish: (data.field[0].len() - 2, data.field.len() - 1),
            edges
        }
    }

    pub fn tot_lens(&self) -> usize {
        self.edges
            .iter()
            .map(|e| e.2)
            .sum()
    }

    pub fn minimize(&mut self) {
        'l: loop {
            let mut edges = self.edges.iter().cloned().collect::<Vec<_>>();
            // Shortest first
            edges.sort_by_key(|e| e.2);
            dbg!(&edges);

            for edge in edges {
                if self.can_remove(&edge) {
                    eprintln!("Removing {:?}", edge);
                    self.edges.remove(&edge);
                    continue 'l;
                }
            }
            break;
        }
        dbg!(&self.edges);
    }

    fn can_remove(&self, edge: &Edge) -> bool {
        let edges0 =
            self.edges
                .iter()
                .filter(|e| e.0 == edge.0 || e.1 == edge.0)
                .count();
        let edges1 =
            self.edges
                .iter()
                .filter(|e| e.0 == edge.1 || e.1 == edge.1)
                .count();
        if edges0 == 1 {
            if edge.0 == self.start || edge.0 == self.finish {
                return false;
            }
            // Dead end
            return true;
        }
        if edges1 == 1 {
            if edge.1 == self.start || edge.1 == self.finish {
                return false;
            }
            // Dead end
            return true;
        }

        let mut edges = self.edges.clone();
        edges.remove(&edge);

        Self::is_connected(&edges, edge.0, edge.1)
    }

    fn is_connected(edges: &BTreeSet<Edge>, start: (usize, usize), finish: (usize, usize)) -> bool {
        let mut found = BTreeSet::from([start]);

        if start == finish {
            return true;
        }

        loop {
            let mut added = false;
            for edge in edges {
                let have0 = found.contains(&edge.0);
                let have1 = found.contains(&edge.1);
                match (have0, have1) {
                    (true, true) => (),
                    (true, false) => {
                        if edge.1 == finish {
                            return true;
                        }
                        found.insert(edge.1);
                        added = true;
                    }
                    (false, true) => {
                        if edge.0 == finish {
                            return true;
                        }
                        found.insert(edge.0);
                        added = true;
                    }
                    (false, false) => ()
                }
            }
            if !added {
                return false;
            }
        }
    }

    fn pathlen(&self) -> usize {
        let mut pos = self.start;
        let mut dist = 0;
        let mut edges = self.edges.clone();
        while pos != self.finish {
            let mut connections = Vec::new();
            for edge in &edges {
                if edge.0 == pos || edge.1 == pos {
                    connections.push(*edge);
                }
            }
            for edge in &connections {
                edges.remove(edge);
            }
            dbg!(&connections);
            let others = connections.into_iter()
                .map(|e| if e.0 == pos {
                     (e.1, e.2)
                } else {
                     (e.0, e.2)
                })
                .collect::<Vec<_>>();
            for (otherpos, len) in others {
                if Self::is_connected(&edges, otherpos, self.finish) {
                    pos = otherpos;
                    dist += len;
                    break;
                }
            }
        }
        dist
    }

    fn do_solve(&mut self, cache: &mut HashMap<(BTreeSet<Edge>, (usize, usize)), Option<(Vec<Pos>, usize)>>, pos: (usize, usize)) -> Option<(Vec<Pos>, usize)> {
        if pos == self.finish {
            return Some((Vec::new(), 0));
        }
        if let Some(v) = cache.get(&(self.edges.clone(), pos)) {
            return v.clone();
        }

        let edges = self.edges.iter()
                        .filter(|&e| e.0 == pos || e.1 == pos)
                        .cloned()
                        .collect::<Vec<Edge>>();
        // Remove all the edges
        for edge in &edges {
            self.edges.remove(edge);
        }
        let mut best = 0;
        let mut best_path = None;
        for edge in &edges {
            let otherpos = if edge.0 == pos { edge.1 } else { edge.0 };
            if let Some((path, result)) = self.do_solve(cache, otherpos) {
                if result + edge.2 > best {
                    best = result + edge.2;
                    best_path = Some(path);
                }
            }
        }
        // Put the edges back
        for edge in edges {
            self.edges.insert(edge);
        }
        let result = if let Some(mut best_path) = best_path {
            best_path.push(pos);
            Some((best_path, best))
        } else {
            None
        };

        cache.insert((self.edges.clone(), pos), result.clone());
        result
    }

    pub fn solve(&mut self) -> usize {
        if let Some((path, best)) = self.do_solve(&mut Default::default(), self.start) {
            //dbg!(path);
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
