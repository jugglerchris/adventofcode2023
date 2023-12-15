
#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

type Data<'a> = Vec<&'a str>;
fn parse_input(input: &str) -> Data<'_> {
    input.trim().split(',').collect()
}

fn hash(s: &[u8]) -> u8 {
    let mut result = 0u16;
    for &b in s {
        result += b as u16;
        result *= 17;
        result &= 0xff;
    }
    result as u8
}

timeit!{
fn part1(data: &Data) -> usize {
    data.iter()
        .map(|s| hash(s.as_bytes()) as usize)
        .sum()
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for insn in data {
        if insn.ends_with('-') {
            // Remove a lens
            let name = &insn[0..(insn.len() - 1)];
            let boxnum = hash(name.as_bytes()) as usize;
            let bx = &mut boxes[boxnum];
            let mut idx = None; 
            for (i, (n, _v)) in bx.iter().enumerate() {
                if *n == name {
                    idx = Some(i);
                    break;
                }
            }
            if let Some(idx) = idx {
                bx.remove(idx);
            }
        } else {
            let (name, v) = insn.split_once('=').unwrap();
            let boxnum = hash(name.as_bytes()) as usize;
            let pow = v.parse().unwrap();
            let bx = &mut boxes[boxnum];
            let mut found = false;
            for (n, v) in bx.iter_mut() {
                if *n == name {
                    *v = pow;
                    found = true;
                    break;
                }
            }
            if !found {
                bx.push((name, pow));
            }
        }
    }
    let mut sum = 0;
    for i in 0..256 {
        for (j, v) in boxes[i].iter().enumerate() {
            sum += (i+1) * (j+1) * v.1;
        }
    }
    sum
}}

#[test]
fn test() {
    assert_eq!(hash(b"HASH"), 52);
    let tests = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 1320);
    assert_eq!(part2(&data), 145);
}

fn main() -> std::io::Result<()>{
    let input = get_input(15)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
