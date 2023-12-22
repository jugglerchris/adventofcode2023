use std::collections::{HashSet, HashMap, hash_map::Entry};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Point = (usize, usize, usize);

#[derive(Copy, Clone, Debug)]
struct Brick {
    pos0: Point,
    pos1: Point,
}
impl Brick {
    fn poses(&self) -> impl IntoIterator<Item = Point> {
        if self.pos0.0 != self.pos1.0 {
            let mn = self.pos0.0.min(self.pos1.0);
            let mx = self.pos0.0.max(self.pos1.0);
            let y = self.pos0.1;
            let z = self.pos0.2;
            (mn..=mx)
                .map(|x| (x, y, z))
                .collect::<Vec<_>>()
        } else if self.pos0.1 != self.pos1.1 {
            let mn = self.pos0.1.min(self.pos1.1);
            let mx = self.pos0.1.max(self.pos1.1);
            let x = self.pos0.0;
            let z = self.pos0.2;
            (mn..=mx)
                .map(|y| (x, y, z))
                .collect::<Vec<_>>()
        } else {
            let mn = self.pos0.2.min(self.pos1.2);
            let mx = self.pos0.2.max(self.pos1.2);
            let x = self.pos0.0;
            let y = self.pos0.1;
            (mn..=mx)
                .map(|z| (x, y, z))
                .collect::<Vec<_>>()
        }
    }
}

struct Data {
    bricks: Vec<Brick>,
}

regex_parser!(parse_brick: Brick {
    B = r#"^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$"# =>
        | x1: usize, y1: usize, z1: usize, x2: usize, y2: usize, z2 : usize|
            Brick {
                pos0: (x1, y1, z1),
                pos1: (x2, y2, z2)
            }
});

fn parse_input(input: &str) -> Data {
    Data {
        bricks: parse_lines(input)
    }
}

fn do_part1(data: &Data) -> usize {
    let mut bricks = data.bricks.clone();
    bricks.sort_by_key(|brick| brick.pos0.2.min(brick.pos1.2));

    let mut unsafe_bricks = HashSet::new();

    let mut space = HashMap::new();
    for (i, brick) in bricks.iter().enumerate() {
        let brick_z = brick.pos0.2.min(brick.pos1.2);
        let mut drop_by = 1;
        while brick_z > drop_by && can_drop(brick, &space, drop_by) {
            drop_by += 1;
        }
        drop_by -= 1;

        // Put the brick in place
        let mut resting_on = HashSet::new();
        for pos in brick.poses() {
            space.insert((pos.0, pos.1, pos.2 - drop_by), i);
            if let Some(blockno) = space.get(&(pos.0, pos.1, pos.2-drop_by-1)) {
                if *blockno != i {
                    resting_on.insert(*blockno);
                }
            }
        }
        if resting_on.len() == 1 {
            unsafe_bricks.insert(resting_on.into_iter().next().unwrap());
        }
    }
    bricks.len() - unsafe_bricks.len()
}

fn can_drop(brick: &Brick, space: &HashMap<Point, usize>, drop_by: usize) -> bool {
    for point in brick.poses() {
        if space.contains_key(&(point.0, point.1, point.2 - drop_by)) {
            return false;
        }
    }
    true
}

timeit!{
fn part1(data: &Data) -> usize {
    do_part1(data)
}}
fn do_part2(data: &Data) -> usize {
    let mut bricks = data.bricks.clone();
    bricks.sort_by_key(|brick| brick.pos0.2.min(brick.pos1.2));

    // Map from brick to bricks it's resting on
    let mut under: HashMap<usize, HashSet<usize>> = HashMap::new();

    let mut space = HashMap::new();
    for (i, brick) in bricks.iter().enumerate() {
        let brick_z = brick.pos0.2.min(brick.pos1.2);
        let mut drop_by = 1;
        while brick_z > drop_by && can_drop(brick, &space, drop_by) {
            drop_by += 1;
        }
        drop_by -= 1;

        // Put the brick in place
        let mut resting_on = HashSet::new();
        for pos in brick.poses() {
            space.insert((pos.0, pos.1, pos.2 - drop_by), i);
            if let Some(blockno) = space.get(&(pos.0, pos.1, pos.2-drop_by-1)) {
                if *blockno != i {
                    resting_on.insert(*blockno);
                }
            }
        }
        under.insert(i, resting_on);
    }
    (0..bricks.len())
        .map(|i| chain_len(&under, i))
        .sum()
}

fn chain_len(under: &HashMap<usize, HashSet<usize>>, i: usize) -> usize {
    let mut result = 0;
    let mut under = under.clone();

    let mut to_remove = vec![i];
    while let Some(blockno) = to_remove.pop() {
        under.iter_mut()
            .for_each(|(k, v)| {
                if v.remove(&blockno) {
                    // k was resting on us.
                    if v.is_empty() {
                        // ...and is now falling
                        to_remove.push(*k);
                        result += 1;
                    }
                }
            });
    }

    result
}

timeit!{
fn part2(data: &Data) -> usize {
    do_part2(data)
}}

#[test]
fn test() {
    let tests = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 5);
    assert_eq!(part2(&data), 7);
}

fn main() -> std::io::Result<()>{
    let input = get_input(22)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
