use std::collections::{HashMap, VecDeque};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,regex_parser,timeit};

#[derive(Clone, Debug)]
enum ModuleType {
    FlipFlop {
        state: bool,
    },
    Conjunction {
        inputs: HashMap<String, bool>,
    },
    Broadcast {
    },
    Rx {}
}

impl Module {
    pub fn is_conj(&self) -> bool {
        match self.module {
            ModuleType::Conjunction { .. } => true,
            _ => false,
        }
    }
    pub fn has_output(&self, out: &str) -> bool {
        for output in &self.outputs {
            if output == out {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug)]
struct Module {
    name: String,
    module: ModuleType,
    outputs: Vec<String>,
}

regex_parser!(parse_module: Module {
    BRD = r#"broadcaster -> (.*)"# => | output_str: String | {
        Module {
            name: "broadcaster".into(),
            module: ModuleType::Broadcast {},
            outputs: output_str.split(", ")
                            .map(|s| s.to_string())
                            .collect(),
        }
    },
    FF = r#"^%(\w+) -> ([\w, ]+)$"# => | name: String, output_str: String | {
        Module {
            name,
            module: ModuleType::FlipFlop {
                state: false,
            },
            outputs: output_str.split(", ")
                            .map(|s| s.to_string())
                            .collect(),
        }
    },
    CONJ = r#"&(\w+) -> ([\w, ]+)$"# => | name: String, output_str: String | {
        Module {
            name,
            module: ModuleType::Conjunction {
                inputs: Default::default(),
            },
            outputs: output_str.split(", ")
                            .map(|s| s.to_string())
                            .collect(),
        }
    }
});

type Data = Vec<Module>;

fn parse_input(input: &str) -> Data {
    parse_lines(input)
}

struct Machine {
    modules: HashMap<String, Module>,
    pulse_true: usize,
    pulse_false: usize,
    finished: bool,
}

impl Machine {
    pub fn from(data: &Data) -> Machine {
        let mut modules: Vec<(String, Module)> =
            data.iter()
                .map(|m| (m.name.clone(), m.clone()))
                .collect();
        let conjes = modules.iter()
                            .filter(|(_, m)| m.is_conj())
                            .map(|(n, _)| n.clone())
                            .collect::<Vec<String>>();
        for conj in &conjes {
            let all_inputs = modules.iter()
                                .filter(|(_, m)| m.has_output(conj))
                                .map(|(n, _)| n.clone())
                                .collect::<Vec<String>>();
            modules.iter_mut()
                .for_each(|(n, m)| {
                    if n == conj {
                        if let ModuleType::Conjunction { ref mut inputs, .. } = &mut m.module {
                            for input in &all_inputs {
                                inputs.insert(input.into(), false);
                            }
                        }
                    }
                });
        }

        modules.push(("rx".into(), Module {
            name: "rx".into(),
            module: ModuleType::Rx {  },
            outputs: Default::default(),
        }));

        Machine {
            modules: modules.into_iter().collect(),
            pulse_true: 0,
            pulse_false: 0,
            finished: false,
        }
    }

    pub fn pulse_count(&self) -> usize {
        self.pulse_true * self.pulse_false
    }

    pub fn prod(&mut self) {
        let mut pulses = VecDeque::new();
        pulses.push_back(("button".to_string(), "broadcaster".to_string(), false));

        while let Some((src, dest, val)) = pulses.pop_front() {
            //eprintln!("{src} => {dest} {val}");
            if val {
                self.pulse_true += 1;
            } else {
                self.pulse_false += 1;
            }
            let mut m = self.modules.get_mut(&dest);
            if let Some(m) = m {
                let newpulse = match &mut m.module {
                    ModuleType::FlipFlop { state } => {
                        if !val {
                            *state = !*state;
//                            eprintln!(" FF {dest} => {}", *state);
                            Some(*state)
                        } else {
                            None
                        }
                    }
                    ModuleType::Conjunction { inputs } => {
                        *inputs.get_mut(&src).unwrap() = val;
                        let newval = !inputs.values().all(|v| *v);
                        Some(newval)
                    }
                    ModuleType::Broadcast { } => {
                        Some(val)
                    }
                    ModuleType::Rx {} => {
                        if !val {
                            self.finished = true;
                        }
                        None
                    }
                };
                if let Some(val) = newpulse {
                    for output in &m.outputs {
                        pulses.push_back((dest.clone(), output.clone(), val));
                    }
                }
            }
        }
    }
}

timeit!{
fn part1(data: &Data) -> usize {
    let mut machine = Machine::from(data);

    for _ in 0..1000 {
        machine.prod();
    }
    machine.pulse_count()
}}
timeit!{
fn part2(data: &Data) -> usize {
    let mut machine = Machine::from(data);
    let mut count = 0;

    while !machine.finished {
        machine.prod();
        count += 1;
        if (count % 10000) == 0 {
            dbg!(count);
        }
    }
    count
}}

#[test]
fn test() {
    let test1 = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;
    let test2 = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;
    let data1 = parse_input(&test1);
    let data2 = parse_input(&test2);

    assert_eq!(part1(&data1), 32000000);
    assert_eq!(part1(&data2), 11687500);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(20)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
