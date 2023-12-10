use std::collections::HashSet;

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

struct Data {
    field: Vec<Vec<u8>>,
    start_pos: (usize, usize),
}

impl Data {
    pub fn is_connected(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
        if x1 == x2 {
            // Vertical connection
            let n1 = y1.min(y2);
            let n2 = y1.max(y2);
            // Check if the top one (lower Y coord) connects down (or might)
            if ![b'|', b'7', b'F', b'S'].contains(&self.field[n1][x1]) {
                return false;
            }
            if ![b'|', b'L', b'J', b'S'].contains(&self.field[n2][x1]) {
                return false;
            }
            true
        } else if y1 == y2 {
            // Horizontal connection
            let n1 = x1.min(x2);
            let n2 = x1.max(x2);
            // Check if the left one connects right (or might)
            if ![b'-', b'L', b'F', b'S'].contains(&self.field[y1][n1]) {
                return false;
            }
            if ![b'-', b'J', b'7', b'S'].contains(&self.field[y1][n2]) {
                return false;
            }
            true
        } else {
            // Not vert/horiz connected
            panic!()
        }
    }
    pub fn adjacent(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if x > 0 && self.is_connected((x, y), (x-1, y)) {
            result.push((x-1, y));
        }
        if x+1 < self.field[y].len() && self.is_connected((x, y), (x+1, y)) {
            result.push((x+1, y));
        }
        if y > 0 && self.is_connected((x, y), (x, y-1)) {
            result.push((x, y-1));
        }
        if y+1 < self.field.len() && self.is_connected((x, y), (x, y+1)) {
            result.push((x, y+1));
        }
        result
    }
    pub fn connected(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let adjac = self.adjacent((x, y));
        adjac.into_iter()
             .filter(|&pos| self.is_connected((x, y), pos))
             .collect()
    }
}

fn parse_input(input: &str) -> Data {
    let field = input.lines()
        .map(|l| Vec::from(l.as_bytes()))
        .collect::<Vec<Vec<u8>>>();

    let mut start_pos = None;
    'top: for (y, row) in field.iter().enumerate() {
        for (x, &b) in row.iter().enumerate() {
            if b == b'S' {
                start_pos = Some((x, y));
                break 'top;
            }
        }
    };
    let start_pos = start_pos.unwrap();

    Data {
        field, start_pos
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut last_pos = data.start_pos;
    let mut pos = data.start_pos;
    let mut count = 0;

    loop {
        let connections = data.connected(pos);
        assert_eq!(connections.len(), 2);
        let new_pos = if connections[0] == last_pos {
            connections[1]
        } else {
            // The first time around, last_pos will be wrong, so we'll pick
            // a random direction
            connections[0]
        };
        last_pos = pos;
        pos = new_pos;
        count += 1;
        if pos == data.start_pos {
            return count / 2;
        }
    }
}}

timeit!{
fn part2(data: &Data) -> usize {
    let mut last_pos = data.start_pos;
    let mut pos = data.start_pos;

    let mut in_pipe = HashSet::new();

    loop {
        let connections = data.connected(pos);
        assert_eq!(connections.len(), 2);
        let new_pos = if connections[0] == last_pos {
            connections[1]
        } else {
            // The first time around, last_pos will be wrong, so we'll pick
            // a random direction
            connections[0]
        };
        last_pos = pos;
        pos = new_pos;
        in_pipe.insert(pos);
        if pos == data.start_pos {
            break;
        }
    }

    let mut count = 0;
    //let mut outmap = Vec::new();
    for (y, row) in data.field.iter().enumerate() {
        //let mut out_row = String::new();

        let mut inside = false;

        for (x, _) in row.iter().enumerate() {
            if in_pipe.contains(&(x, y)) {
                // Think of the trace being just below the centre of the square,
                // so that we don't have to worry about horizontal parts, and just
                // consider a crossing if this cell connects to the one to the
                // south.
                if data.is_connected((x,y), (x, y+1)) {
                    inside = !inside;
                }
                //out_row.push(data.field[y][x] as char);
            } else if inside {
                count += 1;
                //out_row.push('I');
            } else {
                //out_row.push('O');
            }
        }
        //outmap.push(out_row);
    }
    /*
    for s in outmap {
        println!("{}", s);
    }
    */
    count
}}

#[test]
fn test() {
    let tests = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 8);

    let test2_1 = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
    let test2_2 = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

    let data2_1 = parse_input(&test2_1);
    let data2_2 = parse_input(&test2_2);
    assert_eq!(part2(&data2_1), 4);
    assert_eq!(part2(&data2_2), 8);
}

fn main() -> std::io::Result<()>{
    let input = get_input(10)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
