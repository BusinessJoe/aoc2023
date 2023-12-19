use std::{
    collections::HashMap,
    io::{self, Read},
};

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse(line: &str) -> Self {
        let nums: Vec<u32> = line[1..line.len()-1]
            .split('=')
            .skip(1)
            .map(|s| s.split(',').next().unwrap())
            .map(|s| s.parse().expect(&format!("{}", s)))
            .collect();

        Self {
            x: nums[0],
            m: nums[1],
            a: nums[2],
            s: nums[3],
        }
    }

    fn get(&self, r: &Rating) -> u32 {
        match r {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }
}

enum Rating {
    X,
    M,
    A,
    S,
}

impl Rating {
    fn parse(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!("{}", s),
        }
    }
}

#[derive(Clone)]
enum Outcome {
    Send(String),
    Reject,
    Accept,
}

impl Outcome {
    fn parse(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Send(s.to_string()),
        }
    }
}

enum Cond {
    Lt(Rating, u32),
    Gt(Rating, u32),
    Always,
}

impl Cond {
    fn parse(s: &str) -> Self {
        if let Some((c, num)) = s.split_once('<') {
            Self::Lt(Rating::parse(c), num.parse().unwrap())
        } else if let Some((c, num)) = s.split_once('>') {
            Self::Gt(Rating::parse(c), num.parse().unwrap())
        } else {
            panic!("{}", s)
        }
    }

    fn passes(&self, part: &Part) -> bool {
        match self {
            Self::Always => true,
            Self::Lt(r, val) => part.get(r) < *val,
            Self::Gt(r, val) => part.get(r) > *val,
        }
    }
}

struct Filter {
    cond: Cond,
    outcome: Outcome,
}

impl Filter {
    fn parse(s: &str) -> Self {
        match s.split_once(':') {
            None => Self {
                cond: Cond::Always,
                outcome: Outcome::parse(s),
            },
            Some((cond, outcome)) => Self {
                cond: Cond::parse(cond),
                outcome: Outcome::parse(outcome),
            },
        }
    }

    fn get_outcome(&self, part: &Part) -> Option<Outcome> {
        if self.cond.passes(part) {
            Some(self.outcome.clone())
        } else {
            None
        }
    }
}

fn parse_workflow(line: &str) -> (&str, Vec<Filter>) {
    let tokens: Vec<&str> = line.split(['{', '}']).collect();
    let name = tokens[0];

    let filters = tokens[1].split(',').map(Filter::parse).collect();

    (name, filters)
}

pub fn solution_1(input: &str) -> u32 {
    let (workflows_segment, parts_segment) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<&str, Vec<Filter>> =
        workflows_segment.lines().map(parse_workflow).collect();
    let parts: Vec<Part> = parts_segment.lines().map(Part::parse).collect();

    let mut p1 = 0;
    for part in parts {

        let mut name = "in".to_string();
        let accepted = loop {
            let filters = workflows.get(name.as_str()).unwrap();
            let mut out: Option<Outcome> = None;
            for f in filters {
                out = f.get_outcome(&part);
                if out.is_some() {
                    break;
                }
            }
            let out = out.unwrap();
            match out {
                Outcome::Send(s) => {
                    name = s
                },
                Outcome::Accept => break true,
                Outcome::Reject => break false,
            }
        };

        if accepted {
            p1 += part.x + part.m + part.a + part.s;
        }
    }

    p1
}

pub fn solution_2(input: &str) -> i64 {
    0
}

fn main() {
    let stdin = io::stdin();
    let mut input: String = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let p1 = solution_1(&input);
    println!("Part 1: {p1}");
    let p2 = solution_2(&input);
    println!("Part 2: {p2}");
}
