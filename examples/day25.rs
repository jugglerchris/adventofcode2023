use std::collections::{HashMap, HashSet};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Debug)]
struct Data {
    names: Vec<String>,
    nodes: HashMap<usize, Vec<usize>>,
}

impl Data {
    fn edges(&self) -> Vec<u64> {
        let mut result = Vec::new();
        for (k,v) in &self.nodes {
            for d in v {
                if k < d {
                    result.push((*k as u64)<<32 | (*d as u64));
                }
            }
        }
        result
    }

    fn group_size(&self, ignore_edges: &[u64]) -> usize {
        let mut nodes = HashSet::new();
        let mut to_visit = vec![0];

        while let Some(n) = to_visit.pop() {
            nodes.insert(n);
            let links = self.nodes.get(&n).unwrap();
            for other in links {
                let linkval = if n < *other {
                    n<<32 | *other
                } else {
                    *other<<32 | n
                } as u64;
                if ignore_edges.contains(&linkval) {
                    continue;
                }
                if !nodes.contains(other) {
                    to_visit.push(*other);
                }
            }
        }
        nodes.len()
    }
}

fn get_id(src: &str, names: &mut Vec<String>, nodes: &mut HashMap<usize, Vec<usize>>, name_map: &mut HashMap<String, usize>) -> usize {
    if let Some(&id) = name_map.get(src) {
        id
    } else {
        let id = names.len();
        let srcname = src.to_string();
        names.push(srcname.clone());
        name_map.insert(srcname, id);
        nodes.insert(id, Vec::new());
        id
    }
}


fn parse_input(input: &str) -> Data {
    let mut names = Vec::new();
    let mut nodes = HashMap::new();
    let mut name_map = HashMap::new();

    for line in input.lines() {
        let (src, dest_str) = line.split_once(": ").unwrap();

        let src_id = get_id(src, &mut names, &mut nodes, &mut name_map);

        for dest in dest_str.split(" ") {
            let dest_id = get_id(dest, &mut names, &mut nodes, &mut name_map);
            nodes.get_mut(&src_id).unwrap().push(dest_id);
            nodes.get_mut(&dest_id).unwrap().push(src_id);
        }
    }

    Data {
        names, nodes
    }
}

fn do_part1(data: &Data) -> usize {
    let edges = data.edges();

    let mut ignore_edges = Vec::new();
    for i in 0..edges.len() {
        ignore_edges.push(edges[i]);
        for j in i+1..edges.len() {
            ignore_edges.push(edges[j]);
            for k in j+1..edges.len() {
                ignore_edges.push(edges[k]);
                let size = data.group_size(&ignore_edges);
                if size < data.names.len() {
                    return size * (data.names.len() - size);
                }
                ignore_edges.pop();
            }
            ignore_edges.pop();
        }
        ignore_edges.pop();
    }
    todo!()
}

timeit!{
fn part1(data: &Data) -> usize {
    do_part1(data)
}}

fn do_part2(data: &Data) -> usize {
    todo!()
}

timeit!{
fn part2(data: &Data) -> usize {
    do_part2(data)
}}

#[test]
fn test() {
    let tests = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 54);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(25)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
