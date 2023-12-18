use std::collections::{HashSet, BTreeSet};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

regex_parser!(dir: Dir {
    U = r#"U"# => | | Dir::Up,
    D = r#"D"# => | | Dir::Down,
    L = r#"L"# => | | Dir::Left,
    R = r#"R"# => | | Dir::Right
});

#[derive(Clone, Debug)]
pub struct Dig {
    dir: Dir,
    dist: isize,
    colour: String,
}

regex_parser!(parse_dig: Dig {
    DIG = r#"([UDLR]) (\d+) \(#([0-9a-f]{6})\)$"# =>
        | dir: Dir, dist: isize, colour: String | Dig { dir, dist, colour }
});

type Data = Vec<Dig>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CrossType {
    None,
    OnEdge,
    CrossEdge,
}

// Say whether we cr
fn crosses(pos: (isize, isize), edge: &((isize, isize), (isize, isize))) -> CrossType {
    if edge.0.0 == edge.1.0 {
        // Vertical edge
        if edge.0.0 == pos.0 {
            let y1 = edge.0.1.min(edge.1.1);
            let y2 = edge.0.1.max(edge.1.1);
            if y1 == pos.1 {
                CrossType::OnEdge
            } else if y1 <= pos.1 && pos.1 <= y2 {
                CrossType::CrossEdge
            } else {
                CrossType::None
            }
        } else {
            CrossType::None
        }
    } else if edge.0.1 == edge.1.1 {
        // Horizontal edge
        if edge.0.1 == pos.1 {
            let x1 = edge.0.0.min(edge.1.0);
            let x2 = edge.0.0.max(edge.1.0);
            if x1 <= pos.0 && pos.0 <= x2 {
                CrossType::OnEdge
            } else {
                CrossType::None
            }
        } else {
            CrossType::None
        }
    } else {
        panic!()
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut pos = (0, 0);
    let mut edges: Vec<((isize, isize), (isize, isize))> = Vec::new();

    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    for dig in data {
        let inc = match dig.dir {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        let old_pos = pos;
        pos.0 += dig.dist * inc.0;
        pos.1 += dig.dist * inc.1;
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);
        edges.push((old_pos,pos));
    }
    assert_eq!(pos, (0, 0));

    let mut dug = 0;
    for y in min_y..=max_y {
        let mut inside = false;
        let mut row = String::new();
        for x in min_x..=max_x {
            let mut on_edge = false;
            for edge in &edges {
                let cross = crosses((x, y), edge);
                match cross {
                    CrossType::None => {
                    }
                    CrossType::OnEdge => {
                        on_edge = true;
                    }
                    CrossType::CrossEdge => {
                        on_edge = true;
                        inside = !inside;
                    }
                }
            }
            if on_edge || inside {
                if on_edge {
                    row.push('x');
                } else {
                    row.push('#');
                }
                dug += 1;
            } else {
                row.push('.');
            }
        }
//        println!("{row}");
    }
    dug
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 62);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(18)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
