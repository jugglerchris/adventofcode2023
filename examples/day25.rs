use std::collections::{HashMap, HashSet};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Debug)]
struct Data {
    names: Vec<String>,
    nodes: HashMap<usize, Vec<usize>>,
}

fn edge_enc(a: usize, b: usize) -> u64 {
    if a < b {
        (a as u64)<<32 | (b as u64)
    } else {
        (b as u64)<<32 | (a as u64)
    }
}

impl Data {
    fn edges(&self) -> Vec<u64> {
        let mut result = Vec::new();
        for (k,v) in &self.nodes {
            for d in v {
                if k < d {
                    result.push(edge_enc(*k, *d));
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
                let linkval = edge_enc(n, *other);
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

    #[allow(unused)]
    fn find_group(&self, i: usize) -> Vec<usize> {
        let links = self.nodes.get(&i).unwrap();

        let mut all: HashSet<usize> = links.iter().cloned().collect();
        all.insert(i);

        for l in links {
            let other_links = self.nodes.get(l).unwrap();
            let mut other_all: HashSet<usize> = other_links.iter().cloned().collect();
            other_all.insert(*l);
            all = all.intersection(&other_all).cloned().collect();
        }
        all.into_iter().collect()
    }

    #[allow(unused)]
    fn print_dot(&self) {
        println!("graph foo {{");
        for (&n, ls) in self.nodes.iter() {
            print!("    {}", self.names[n]);
            for &l in ls {
                print!(" -- {}", self.names[l]);
            }
            println!("");
        }
        println!("}}");
    }

    fn route(&self, n1: usize, n2: usize) -> Option<Vec<usize>> {
        let mut routes: HashMap<usize, Vec<usize>> = Default::default();

        routes.insert(n1, vec![n1]);

        let mut queue = vec![n1];

        while !queue.is_empty() {
            let mut new_queue = vec![];
            for pt in queue {
                for dest in self.nodes.get(&pt).unwrap() {
                    if !routes.contains_key(dest) {
                        let mut v = routes.get(&pt).unwrap().clone();
                        v.push(*dest);
                        if *dest == n2 {
                            return Some(v);
                        } else {
                            routes.insert(*dest, v);
                        }
                        new_queue.push(*dest);
                    }
                }
            }
            queue = new_queue;
        }
        None
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

    /*
    for i in 0..data.names.len() {
        let grp = data.find_group(i);
        println!("Found group: {:?}", grp);
    }
    */
    let mut edgehist: HashMap<u64, usize> = HashMap::new();
    let num_nodes = data.names.len();
    for i in 0..num_nodes/4 {
        let j = i + num_nodes/2;
        let p = data.route(i, j).unwrap();
        for i in 0..(p.len() - 1) {
            let n1 = p[i];
            let n2 = p[i+1];
            let edge = edge_enc(n1, n2);
            *edgehist.entry(edge).or_default() += 1;
        }
    }
    let mut hist_entries: Vec<_> = edgehist.into_iter()
        .collect();
    hist_entries.sort_by_key(|v| usize::MAX - v.1);

    let mut ignore_edges = Vec::new();
    for i in 2..edges.len() {
        ignore_edges.push(hist_entries[i].0);
        for j in 1..i {
            ignore_edges.push(hist_entries[j].0);
            for k in 0..j {
                ignore_edges.push(hist_entries[k].0);
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

//    data.print_dot();

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
