use std::ops::{BitOrAssign, BitAndAssign};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type BitPiece = u64;

#[derive(Clone, Debug)]
struct Row {
    bits: usize,
    // Low bits at pieces[0]
    pieces: Vec<BitPiece>,
}

impl Row {
    const BITS: usize = BitPiece::BITS as usize;
    pub fn new() -> Row {
        Row {
            bits: 0,
            pieces: Default::default(),
        }
    }

    fn shift_left(&mut self) {
        let new_bits = self.bits+1;
        let new_num_pieces = (new_bits + Self::BITS -1) / Self::BITS;
        if new_num_pieces > self.pieces.len() {
            self.pieces.push(0);
        }

        for i in (0..self.pieces.len()).rev() {
            let mut newval = self.pieces[i] << 1;
            if i > 0 {
                newval |= self.pieces[i-1] >> (Self::BITS-1);
            }
            self.pieces[i] = newval;
        }
        self.bits = new_bits;
    }

    pub fn shift_in(&mut self, val: bool) {
        self.shift_left();
        if val {
            self.pieces[0] |= 1;
        }
    }

    fn count_ones(&self) -> usize {
        self.pieces
            .iter()
            .cloned()
            .map(|n| n.count_ones() as usize)
            .sum()
    }

    fn or_shift_2(&mut self, other: &Row) {
        self.bits = self.bits.max(other.bits);
        while self.pieces.len() < other.pieces.len() {
            self.pieces.push(0);
        }
        for i in 0..other.pieces.len() {
            self.pieces[i] |= other.pieces[i] << 1;
            self.pieces[i] |= other.pieces[i] >> 1;
            if i > 0 {
                self.pieces[i] |= other.pieces[i-1] >> (Self::BITS-1);
            }
            if i+1 < other.pieces.len() {
                self.pieces[i] |= other.pieces[i+1] << (Self::BITS-1);
            }
        }
    }

    fn get(&self, j: usize) -> bool {
        let wordidx = j / Self::BITS;
        let bitidx = j % Self::BITS;
        if wordidx >= self.pieces.len() {
            false
        } else {
            (self.pieces[wordidx] >> bitidx) & 1 == 1
        }
    }
}

impl BitOrAssign<&Row> for Row {
    fn bitor_assign(&mut self, rhs: &Row) {
        self.bits = self.bits.max(rhs.bits);
        for i in 0..rhs.pieces.len() {
            if i >= self.pieces.len() {
                self.pieces.push(rhs.pieces[i]);
            } else {
                self.pieces[i] |= rhs.pieces[i];
            }
        }
    }
}

impl BitAndAssign<&Row> for Row {
    fn bitand_assign(&mut self, rhs: &Row) {
        self.bits = self.bits.max(rhs.bits);
        for i in 0..rhs.pieces.len() {
            if i >= self.pieces.len() {
                self.pieces.push(0);
            } else {
                self.pieces[i] &= rhs.pieces[i];
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Data {
    plots: Vec<Row>,
    spots: Vec<Row>,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.plots.len() {
            for j in 0..self.plots[i].bits {
                if !self.plots[i].get(j) {
                    write!(f, "#")?;
                } else if self.spots[i].get(j) {
                    write!(f, "O")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Data {
    pub fn count_spots(&self) -> usize {
        self.spots
            .iter()
            .map(|row| row.count_ones())
            .sum()
    }

    pub fn next(&self) -> Self {
        let mut new_spots = Vec::new();
        for i in 0..self.spots.len() {
            let mut new_row = Row::new();
            new_row.or_shift_2(&self.spots[i]);
            if i > 0 {
                new_row |= &self.spots[i-1];
            }
            if i+1 < self.spots.len() {
                new_row |= &self.spots[i+1];
            }
            new_row &= &self.plots[i];
            new_spots.push(new_row);
        }

        Data {
            plots: self.plots.clone(),
            spots: new_spots,
        }
    }

}

fn parse_input(input: &str) -> Data {
    let mut plots = Vec::new();
    let mut spots = Vec::new();
    for row in input.lines() {
        let mut plot = Row::new();
        let mut spot = Row::new();

        for c in row.chars() {
            plot.shift_in(c != '#');
            spot.shift_in(c == 'S');
        }
        plots.push(plot);
        spots.push(spot);
    }

    Data {
        plots,
        spots
    }
}

timeit!{
fn part1(data: &Data, steps: usize) -> usize {
    let mut data = data.clone();
    for _ in 0..steps {
        data = data.next();
    }
    data.count_spots()
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data, 6), 16);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(21)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data, 64));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
