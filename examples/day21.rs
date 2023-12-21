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
    fn set(&mut self, j: usize, val: bool) {
        let wordidx = j / Self::BITS;
        let bitidx = j % Self::BITS;
        if val {
            self.pieces[wordidx] |= 1<< bitidx;
        } else {
            self.pieces[wordidx] &= !(1<< bitidx);
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

fn count_start_from(data: &Data, x: usize, y: usize, steps: usize) -> usize 
{
    let mut data = data.clone();
    data.spots[y].set(x, true);

    part1(&data, steps)
}

timeit!{
fn part2(orig_data: &Data, steps: usize) -> usize {
    // The input has no rocks along the outer edges or the centres,
    // so the propagation will be full speed.
    // The result will be a large diamond with (abs(x)+abs(y)) <= steps
    // away from the centre.
    let mut total = 0;

    let height = orig_data.spots.len();
    let width = orig_data.spots[0].bits;
    assert_eq!(width, height);
    assert!(width & 1 == 1);
    let hhei = height / 2;
    let hwid = width / 2;

    let mut data = orig_data.clone();
    assert!(data.spots[hhei].get(hwid));
    data.spots[hhei].set(hwid, false);

    let full_from_edge_odd = count_start_from(&data, 0, hhei, 1001);
    let full_from_edge_even = count_start_from(&data, 0, hhei, 1000);

    // Full centre square (need the parity right)
    total += part1(orig_data, steps % 200);

    // Along each axis, each square (counting from centre square being -1):
    // * Has the first edge visited at time (hwid + 1 + N*width)
    // * Is complete at (hwid + (N+1)*width + hhei) = (N+2)*width - 1.
    //   - Let's just say (N+2)*width = steps
    //   - So N+2 = steps/width
    //   -    N = (steps / width) - 2
    // 
    let complete_each_edge = (steps / width) - 2;
    assert!(complete_each_edge % 2 == 0); // Don't have to worry about whether we start with even
                                          // or odd
    // Handle the complete ones along all four axes
    total += (complete_each_edge/2) * (full_from_edge_odd + full_from_edge_even) * 4;

    // Next, handle the incomplete ones.
    let axis_partial_start_time = hwid + 1 + complete_each_edge * width;
    let remain_edge_steps = steps - axis_partial_start_time;
    total += count_start_from(&data, 0, hhei, remain_edge_steps);
    total += count_start_from(&data, width-1, hhei, remain_edge_steps);
    total += count_start_from(&data, hwid, 0, remain_edge_steps);
    total += count_start_from(&data, hwid, height-1, remain_edge_steps);

    if remain_edge_steps >= width {
        // We started another
        total += count_start_from(&data, 0, hhei, remain_edge_steps - width);
        total += count_start_from(&data, width-1, hhei, remain_edge_steps - width);
        total += count_start_from(&data, hwid, 0, remain_edge_steps - width);
        total += count_start_from(&data, hwid, height-1, remain_edge_steps - width);
    }

    // Now deal with the bulk of squares off axis, which we reach from the corners.
    // In each quadrant:
    // One square is reached at time hwid + hhei + 2 == width + 1 == height + 1
    // After that, the next diagonal (with one more square) is reached width steps later.
    {
        let mut t = height + 1;
        let mut n = 1;
        let mut num_odd = 0;
        let mut num_even = 0;
        while t <= steps {
            let remain = steps - t;
            if remain < 2*width {
                let num_set_for_one =
                    count_start_from(&data, 0, 0, remain) +
                    count_start_from(&data, width-1, 0, remain) +
                    count_start_from(&data, 0, height-1, remain) +
                    count_start_from(&data, width-1, height-1, remain);
                total += num_set_for_one * n;
            } else if (remain & 1) == 1 {
                num_odd += n;
            } else {
                num_even += n;
            }
            t += width;
            n += 1;
        }

        let full_from_corner_even = count_start_from(&data, 0, 0, 1000);
        let full_from_corner_odd = count_start_from(&data, 0, 0, 1001);

        total += num_odd * 4 * full_from_corner_odd;
        total += num_even * 4 * full_from_corner_even;
    }

    total
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
//    assert_eq!(part2(&data, 6), 16);
//    assert_eq!(part2(&data, 10), 60);
    assert_eq!(part2(&data, 50), 1594);
    assert_eq!(part2(&data, 100), 6536);
    assert_eq!(part2(&data, 500), 167004);
    assert_eq!(part2(&data, 1000), 668697);
    assert_eq!(part2(&data, 5000), 16733044);
}

fn main() -> std::io::Result<()>{
    let input = get_input(21)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data, 64));

    // Part 2
    println!("{}", part2(&data, 26501365));

    Ok(())
}
