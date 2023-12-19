use std::{collections::HashMap, str::FromStr};

#[allow(unused)]
use adventofcode2023::{get_input,parse_lines,parse_list,regex_parser,timeit};

type Value = usize;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum FieldID {
    X,
    M,
    A,
    S
}

impl FromStr for FieldID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => FieldID::X,
            "m" => FieldID::M,
            "a" => FieldID::A,
            "s" => FieldID::S,
            _ => panic!("FieldID::from_str({s})")
        })
    }
}

#[derive(Clone, Debug)]
struct Part {
    x: Value,
    m: Value,
    a: Value,
    s: Value,
}

impl Part {
    fn get(&self, field: FieldID) -> Value {
        match field {
            FieldID::X => self.x,
            FieldID::M => self.m,
            FieldID::A => self.a,
            FieldID::S => self.s,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum WorkflowID {
    Accept,
    Reject,
    Named(String)
}

impl FromStr for WorkflowID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => WorkflowID::Accept,
            "R" => WorkflowID::Reject,
            s => WorkflowID::Named(s.into()),
        })
    }
}

#[derive(Clone, Debug)]
enum Rule {
    Lt {
        field: FieldID,
        value: Value,
        target: WorkflowID,
    },
    Gt {
        field: FieldID,
        value: Value,
        target: WorkflowID,
    },
    Jmp(WorkflowID)
}

#[derive(Debug)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

regex_parser!(parse_part: Part {
    PART = r#"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}"# =>
        |x: Value, m: Value, a: Value, s: Value| Part { x, m, a, s }
});

regex_parser!(parse_rule: Rule {
    JMP = r#"^(\w+)$"# => | target: WorkflowID| Rule::Jmp(target),
    LT = r#"^(\w+)<(\d+):(\w+)"# =>
        | field: FieldID, value: Value, target: WorkflowID | {
            Rule::Lt { field, value, target } },
    GT = r#"^(\w+)>(\d+):(\w+)"# =>
        | field: FieldID, value: Value, target: WorkflowID | {
            Rule::Gt { field, value, target } }

});

regex_parser!(parse_wf: Workflow {
    WF = r#"(\w+)\{(.*)\}"# => | name: String, rules_str: String | {
        let rules = parse_list(&rules_str, ",");
        Workflow { name, rules }
    }
});

struct Data {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

fn parse_input(input: &str) -> Data {
    let (workflow_str, parts_str) = input.split_once("\n\n").unwrap();

    let workflows_v: Vec<Workflow> = parse_lines(workflow_str);
    let parts: Vec<Part> = parse_lines(parts_str);

    let workflows = workflows_v.into_iter()
        .map(|w| (w.name.clone(), w))
        .collect();

    Data {
        workflows,
        parts
    }
}

fn run_workflows<'p, 'w>(part: &'p Part, workflows: &'w HashMap<String, Workflow>) -> &'w WorkflowID {
    let mut wid = "in";
    loop {
        let wf = workflows.get(wid).unwrap();
        for rule in &wf.rules {
            match rule {
                Rule::Lt { field, value, target } => {
                    if part.get(*field) < *value {
                        match target {
                            WorkflowID::Accept |
                            WorkflowID::Reject => {
                                return target;
                            }
                            WorkflowID::Named(id) => wid = id,
                        }
                        break;
                    }
                }
                Rule::Gt { field, value, target } => {
                    if part.get(*field) > *value {
                        match target {
                            WorkflowID::Accept |
                            WorkflowID::Reject => {
                                return target;
                            }
                            WorkflowID::Named(id) => wid = id,
                        }
                        break;
                    }
                }
                Rule::Jmp(target) => {
                    match target {
                        WorkflowID::Accept |
                            WorkflowID::Reject => {
                                return target;
                            }
                        WorkflowID::Named(id) => wid = id,
                    }
                    break;
                }
            }
        }
    }
}

timeit!{
fn part1(data: &Data) -> Value {
    let mut result = 0;
    for part in &data.parts {
        match run_workflows(part, &data.workflows) {
            WorkflowID::Accept => {
                result += part.x + part.m + part.a + part.s;
            }
            WorkflowID::Reject => (),
            WorkflowID::Named(_) => panic!()
}
    }
    result
}}
timeit!{
fn part2(data: &Data) -> usize {
    unimplemented!()
}}

#[test]
fn test() {
    let tests = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
    let data = parse_input(&tests);

    assert_eq!(part1(&data), 19114);
//    assert_eq!(part2(&data), 0);
}

fn main() -> std::io::Result<()>{
    let input = get_input(19)?;

    let data = parse_input(&input);

    // Part 1
    println!("{}", part1(&data));

    // Part 2
    println!("{}", part2(&data));

    Ok(())
}
