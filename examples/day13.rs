#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Row = u32;

struct Field {
    num_cols: usize,
    rows: Vec<u32>,
}

fn nearly_eq(a: u32, b: u32) -> bool {
    let diffs = a ^ b;
    let ones = diffs.count_ones();
    ones <= 1
}

impl Field {
    pub fn col(&self, i: usize) -> u32 {
        let mut result = 0;
        for r in &self.rows {
            result <<= 1;
            result |= (r >> i) & 1;
        }
        result
    }
    pub fn vert_sym(&self) -> Option<usize> {
        'i: for i in 0..self.num_cols {
            if self.col(i) == self.col(i+1) {
                for offs in 0.. {
                    if offs > i {
                        return Some(self.num_cols - i - 1);
                    } else if i+offs+1 >= self.num_cols {
                        return Some(self.num_cols - i - 1);
                    }
                    if self.col(i-offs) != self.col(i+offs+1) {
                        continue 'i;
                    }
                }
            }
        }
        None
        
    }
    pub fn horz_sym(&self) -> Option<usize> {
        'i: for i in 0..(self.rows.len()-1) {
            if self.rows[i] == self.rows[i+1] {
                for offs in 0.. {
                    if offs > i {
                        return Some(i+1);
                    } else if i+offs+1 >= self.rows.len() {
                        return Some(i+1);
                    }
                    if self.rows[i-offs] != self.rows[i+offs+1] {
                        continue 'i;
                    }
                }
            }
        }
        None
    }
    pub fn vert_sym_smudged(&self) -> Option<usize> {
        'i: for i in 0..self.num_cols {
            if nearly_eq(self.col(i), self.col(i+1)) {
                let mut smudges = 0;
                for offs in 0.. {
                    if offs > i {
                        if smudges == 1 {
                            return Some(self.num_cols - i - 1);
                        } else {
                            continue 'i;
                        }
                    } else if i+offs+1 >= self.num_cols {
                        if smudges == 1 {
                            return Some(self.num_cols - i - 1);
                        } else {
                            continue 'i;
                        }
                    }
                    let diffs = self.col(i-offs) ^ self.col(i+offs+1);
                    smudges += diffs.count_ones();
                    if smudges > 1 {
                        continue 'i;
                    }
                }
            }
        }
        None
        
    }
    pub fn horz_sym_smudged(&self) -> Option<usize> {
        'i: for i in 0..(self.rows.len()-1) {
            if nearly_eq(self.rows[i], self.rows[i+1]) {
                let mut smudges = 0;
                for offs in 0.. {
                    if offs > i {
                        if smudges == 1 {
                            return Some(i+1);
                        } else {
                            continue 'i;
                        }
                    } else if i+offs+1 >= self.rows.len() {
                        if smudges == 1 {
                            return Some(i+1);
                        } else {
                            continue 'i;
                        }
                    }
                    let diffs = self.rows[i-offs] ^ self.rows[i+offs+1];
                    smudges += diffs.count_ones();
                    if smudges > 1 {
                        continue 'i;
                    }
                }
            }
        }
        None
    }
}

fn make_row(s: &str) -> Row {
    let mut result = Default::default();
    for c in s.chars() {
        result = result << 1;
        if c == '#' {
            result |= 1;
        }
    }
    result
}

type Data = Vec<Field>;
fn parse_input(input: &str) -> Data {
    let fieldstrs = input.split("\n\n");
    let mut result = Vec::new();
    for fieldstr in fieldstrs {
        let num_cols = fieldstr.lines().next().unwrap().len();
        let rows = fieldstr.lines()
            .map(make_row)
            .collect();
        result.push(Field { num_cols, rows });
    }
    result
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut result = 0;
    for field in data {
        if let Some(n) = field.vert_sym() {
            result += n;
        }
        if let Some(n) = field.horz_sym() {
            result += 100 * n;
        }
    }
    result
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut result = 0;
    for field in data {
        if let Some(n) = field.vert_sym_smudged() {
            result += n;
        }
        if let Some(n) = field.horz_sym_smudged() {
            result += 100 * n;
        }
    }
    result
}}

#[test]
fn test() {
    let tests = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 405);
    assert_eq!(part2(&data), 400);
}

fn main() -> std::io::Result<()>{
    let input = get_input(13)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
