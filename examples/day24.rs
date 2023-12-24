use std::{ops::{Div, Mul, Sub, Add}, cmp::Ordering, fmt::Display};

use adventofcode2023::gcd;
#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Coord = i128;
type Vec3 = [Coord; 3];

#[derive(Copy, Clone, Hash, Debug)]
struct Stone {
    pos: Vec3,
    vel: Vec3,
}

regex_parser!(parse_stone: Stone {
    S = r#"(-?\d+), (-?\d+), (-?\d+) @ *(-?\d+), *(-?\d+), *(-?\d+)$"# =>
        |x: Coord, y: Coord, z: Coord, vx: Coord, vy: Coord, vz: Coord|
           Stone {
               pos: [x, y, z],
               vel: [vx, vy, vz],
           }
});

type Data = Vec<Stone>;
fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

timeit!{
fn part1(data: &Data) -> usize {
    do_part1(data, 200000000000000, 400000000000000)
}}

fn do_part1(data: &Data, c1: Coord, c2: Coord) -> usize {
    let mut result = 0;
    for i in 1..data.len() {
        for j in 0..i {
            if will_collide(&data[i], &data[j], c1, c2) {
                result += 1;
            }
        }
    }
    result
}

#[derive(Copy, Clone)]
struct Rat {
    num: Coord,
    den: Coord,
}

impl Rat {
    pub fn recip(&self) -> Rat {
        Rat {
            num: self.den,
            den: self.num,
        }
    }

    pub fn norm(&self) -> Rat {
        let num = self.num;
        let den = self.den;
        let (num, den) = if den < 0 {
            (-num, -den)
        } else {
            (num, den)
        };
        let gcd = gcd(num.abs() as usize, den.abs() as usize);
        if gcd > 1 {
            Rat {
                num: num / (gcd as Coord),
                den: den / (gcd as Coord),
            }
        } else {
            Rat { num, den }
        }
    }

    pub fn is_pos(&self) -> bool {
        let n = self.norm();
        n.num > 0
    }
}

impl From<Coord> for Rat {
    fn from(value: Coord) -> Self {
        Rat {
            num: value,
            den: 1,
        }
    }
}

impl Div<Rat> for Rat {
    type Output = Rat;

    fn div(self, rhs: Rat) -> Self::Output {
        self * rhs.recip()
    }
}

impl Mul<Rat> for Rat {
    type Output = Rat;

    fn mul(self, rhs: Rat) -> Self::Output {
        let num = self.num.checked_mul(rhs.num).unwrap();
        let den = self.den.checked_mul(rhs.den).unwrap();
        Rat {
            num,
            den
        }.norm()
    }
}

impl Mul<Coord> for Rat {
    type Output = Rat;

    fn mul(self, rhs: Coord) -> Self::Output {
        self * Rat::from(rhs)
    }

}

impl Sub<Rat> for Rat {
    type Output = Rat;

    fn sub(self, rhs: Rat) -> Self::Output {
        let den = self.den.checked_mul(rhs.den).unwrap();
        let num1 = self.num.checked_mul(rhs.den).unwrap();
        let num2 = rhs.num.checked_mul(self.den).unwrap();
        let num = num1.checked_sub(num2).unwrap();

        Rat {
            num,
            den,
        }.norm()
    }
}
impl Add<Rat> for Rat {
    type Output = Rat;

    fn add(self, rhs: Rat) -> Self::Output {
        let den = self.den.checked_mul(rhs.den).unwrap();
        let num1 = self.num.checked_mul(rhs.den).unwrap();
        let num2 = rhs.num.checked_mul(self.den).unwrap();
        let num = num1.checked_add(num2).unwrap();

        Rat {
            num,
            den,
        }.norm()
    }
}

impl PartialEq<Rat> for Rat {
    fn eq(&self, other: &Rat) -> bool {
        let n1 = self.norm();
        let n2 = other.norm();
        n1.num == n2.num && n1.den == n2.den
    }
}

impl PartialOrd<Rat> for Rat {
    fn partial_cmp(&self, other: &Rat) -> Option<std::cmp::Ordering> {
        println!("{self} cmp {other}");
        println!("  sub = {}", *other - *self);
        if self == other {
            Some(Ordering::Equal)
        } else if (*other - *self).is_pos() {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq<Coord> for Rat {
    fn eq(&self, other: &Coord) -> bool {
        self.eq(&Rat::from(*other))
    }
}

impl PartialOrd<Coord> for Rat {
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        self.partial_cmp(&Rat::from(*other))
    }
}

impl Display for Rat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(self.den > 0);
        if self.den == 1 {
            write!(f, "Rat({})", self.num)
        } else {
            write!(f, "Rat({}/{})", self.num, self.den)
        }
    }
}

fn will_collide(s1: &Stone, s2: &Stone, c1: Coord, c2: Coord) -> bool {
    // Do the lines cross?
    // y = m1*x + b1 = m2*x + b2
    //   => (m1 - m2)*x = (b2 - b1)
    //   => x = (b2 - b1) / (m1 - m2)
    println!("will_collide {s1:?} {s2:?}");
    let m1 = Rat::from(s1.vel[1]) / Rat::from(s1.vel[0]);
    let m2 = Rat::from(s2.vel[1]) / Rat::from(s2.vel[0]);

    println!("m1 = {m1}, m2 = {m2}");

    // y = m*x + b
    // => b = y - (m*x)
    println!("y = {}", Rat::from(s1.pos[1]));
    println!("m1*{} = {}", s1.pos[0], m1 * s1.pos[0]);
    let b1 = Rat::from(s1.pos[1]) - m1 * s1.pos[0];
    let b2 = Rat::from(s2.pos[1]) - m2 * s2.pos[0];

    println!("b1 = {b1}, b2 = {b2}");

    // Solve for matching x
    if m1 == m2 {
        println!("m1 == m2, fail");
        return false;
    }
    let x = (b2 - b1) / (m1 - m2);

    if x < c1 || x > c2 {
        println!("x out of range ({x})");
        return false;
    }
    // Solve for y
    let y = m1 * x + b1;
    if y < c1 || y > c2 {
        println!("y out of range ({y})");
        return false;
    }
    
    // Now see if it's in the past
    let t1 = (x - Rat::from(s1.pos[0])) / Rat::from(s1.vel[0]);
    if t1 < 0 {
        println!("s1 in past");
        return false;
    }
    let t2 = (x - Rat::from(s2.pos[0])) / Rat::from(s2.vel[0]);
    if t2 < 0 {
        println!("s2 in past");
        return false;
    }

    println!("  => will collide");
    return true;
}

timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
    let data = parse_input(&tests);

    assert_eq!(do_part1(&data, 7, 27), 2);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(24)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
