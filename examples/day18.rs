use std::collections::BTreeSet;

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

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Copy, Clone)]
enum EdgeType {
    Up,   // At the bottom of the edge
    Down, // At the top of the edge
    Cross // Edge goes up and down from here
}
timeit!{
fn part2(data: &Data) -> isize {
    let mut pos = (0, 0);
    let mut edges: Vec<((isize, isize), (isize, isize))> = Vec::new();

    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    let mut y_values = BTreeSet::new();

    for dig in data {
        let inc = match dig.colour.as_bytes()[5] {
            b'3' => (0, -1),
            b'1' => (0, 1),
            b'2' => (-1, 0),
            b'0' => (1, 0),
            _ => panic!(),
        };
        let dist = isize::from_str_radix(&dig.colour[0..5], 16).unwrap();
        let old_pos = pos;
        pos.0 += dist * inc.0;
        pos.1 += dist * inc.1;
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);
        y_values.insert(pos.1);

        if inc.0 == 0 {
            edges.push((old_pos,pos));
        }
    }
    assert_eq!(pos, (0, 0));
    let y_values = y_values.into_iter().collect::<Vec<_>>();

    let mut dug = 0;

    let mut num_inside_in_row = 0;
    let mut last_y = 0;
    for y in y_values.into_iter() {
        // Add up the rows between the previous and current y we've considered
        dug += num_inside_in_row * (y - last_y - 1);

        num_inside_in_row = 0;
        last_y = y;

        let mut row_edges: Vec<_> = edges.iter()
            .filter(|e| (e.0.1.min(e.1.1) <= y &&
                         e.0.1.max(e.1.1) >= y))
            .map(|e| {
                let ymin = e.0.1.min(e.1.1);
                let ymax = e.0.1.max(e.1.1);
                let x = e.0.0;
                if ymin == y {
                    (x, EdgeType::Down)
                } else if ymax == y {
                    (x, EdgeType::Up)
                } else {
                    (x, EdgeType::Cross)
                }
            })
            .collect();
        row_edges.sort();

        let mut inside = false;
        // Some(x value at start of horizontal edge)
        let mut start_inside = None;

        // State for tracking current row
        let mut hor_start = None;
        let mut row_inside_start = None;

        for (x, et) in row_edges {
            // First work out what will happen for the rows below
            match et {
                EdgeType::Up => {
                    // Ignore, as we're working just below the horizontal line.
                }
                EdgeType::Down |
                EdgeType::Cross => {
                    // Crossing from inside 
                    if inside {
                        num_inside_in_row += x - start_inside.take().unwrap() + 1;
                        inside = false;
                    } else {
                        assert!(start_inside.is_none());
                        start_inside = Some(x);
                        inside = true;
                    }
                }
            }

            // Now handle the current row
            match (et, hor_start, row_inside_start) {
                (EdgeType::Cross, None, None) => {
                    // Cross from outside to inside
                    row_inside_start = Some(x);
                }
                (EdgeType::Cross, None, Some(start)) => {
                    // Cross from inside to outside
                    dug += x - start + 1;
                    row_inside_start = None;
                }
                (EdgeType::Cross, Some(_hstart), _) => {
                    // Can't have a crossing inside a horizontal run.
                    panic!();
                }
                (EdgeType::Up, None, None) => {
                    // Start of horizontal run, and outside
                    hor_start = Some(x);
                }
                (EdgeType::Down, None, None) => {
                    // Start of horizontal run, crossing to inside
                    hor_start = Some(x);
                    row_inside_start = Some(x);
                }
                (EdgeType::Up, Some(hstart), None) => {
                    // end of a horizontal run, otherwise outside
                    dug += x - hstart + 1;
                    hor_start = None;
                }
                (EdgeType::Up, None, Some(start)) => {
                    // Start of horizontal run, and we're inside
                    // Add previous bit of inside
                    dug += x - start;
                    hor_start = Some(x);
                }
                (EdgeType::Down, None, Some(start)) => {
                    // Start of horizontal run, and we were inside
                    // Add previous bit of inside
                    dug += x - start;
                    row_inside_start = None;
                    hor_start = Some(x);
                }
                (EdgeType::Up, Some(hstart), Some(_start)) => {
                    // End of horizontal edge, staying inside
                    dug += x - hstart + 1;
                    hor_start = None;
                    // Start new inside section, as we've alreayd covered
                    // up to here.
                    row_inside_start = Some(x+1);
                }
                (EdgeType::Down, Some(hstart), None) => {
                    // End of horizontal run, were outside but now inside.
                    dug += x - hstart + 1;
                    hor_start = None;
                    row_inside_start = Some(x+1);
                }
                (EdgeType::Down, Some(hstart), Some(_start)) => {
                    // End of horizontal run, were inside but now outside
                    dug += x - hstart + 1;
                    hor_start = None;
                    row_inside_start = None;
                }
            }
        }
        assert!(start_inside.is_none());
        assert!(row_inside_start.is_none());
        assert!(!inside);
    }
    dug
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

    let test2_sq = r#"R 0 (#000020)
R 0 (#000021)
R 0 (#000022)
R 0 (#000023)"#;
    let test2_sq_data = parse_input(&test2_sq);
    assert_eq!(part2(&test2_sq_data), 9);

    assert_eq!(part2(&data), 952408144115);
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
