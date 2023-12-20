use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug, Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse(line: &str) -> Self {
        let nums: Vec<u32> = line[1..line.len() - 1]
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

    fn get_mut(&mut self, r: &Rating) -> &mut u32 {
        match r {
            Rating::X => &mut self.x,
            Rating::M => &mut self.m,
            Rating::A => &mut self.a,
            Rating::S => &mut self.s,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug)]
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
            Self::Always
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

#[derive(Debug)]
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

    fn split(&self, min_part: Part, max_part: Part) -> ((Part, Part), Option<(Part, Part)>) {
        match &self.cond {
            Cond::Always => ((min_part, max_part), None),
            Cond::Lt(r, val) => {
                let mut t_max = max_part.clone();
                let mut f_min = min_part.clone();
                *t_max.get_mut(&r) = *val - 1;
                *f_min.get_mut(&r) = *val - 1;
                ((min_part, t_max), Some((f_min, max_part)))
            }
            Cond::Gt(r, val) => {
                let mut t_min = min_part.clone();
                let mut f_max = max_part.clone();
                *t_min.get_mut(&r) = *val;
                *f_max.get_mut(&r) = *val;
                ((t_min, max_part), Some((min_part, f_max)))
            }
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
                Outcome::Send(s) => name = s,
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

fn count_options(min: &Part, max: &Part) -> u64 {
    (max.x - min.x) as u64
        * (max.m - min.m) as u64
        * (max.a - min.a) as u64
        * (max.s - min.s) as u64
}

fn count_accepted(
    name: &str,
    workflows: &HashMap<&str, Vec<Filter>>,
    mut min_part: Part,
    mut max_part: Part,
) -> u64 {
    let filters = workflows.get(name).unwrap();

    let mut total: u64 = 0;

    for filter in filters {
        let f_case = match &filter.outcome {
            Outcome::Reject => {
                let (_, f_case) = filter.split(min_part.clone(), max_part.clone());
                f_case
            }
            Outcome::Accept => {
                let ((t_min, t_max), f_case) = filter.split(min_part.clone(), max_part.clone());
                total += count_options(&t_min, &t_max);
                f_case
            }
            Outcome::Send(next) => {
                let ((t_min, t_max), f_case) = filter.split(min_part.clone(), max_part.clone());
                total += count_accepted(&next, workflows, t_min.clone(), t_max.clone());
                f_case
            }
        };

        match f_case {
            Some((f_min, f_max)) => {
                min_part = f_min;
                max_part = f_max;
            }
            None => {
                break;
            }
        }
    }

    total
}

pub fn solution_2(input: &str) -> u64 {
    let (workflows_segment, _) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<&str, Vec<Filter>> =
        workflows_segment.lines().map(parse_workflow).collect();

    count_accepted(
        "in",
        &workflows,
        Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        },
        Part {
            x: 4000,
            m: 4000,
            a: 4000,
            s: 4000,
        },
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(456651, solution_1(include_str!("../input.txt")));
    }

    #[test]
    fn part_2() {
        assert_eq!(131899818301477, solution_2(include_str!("../input.txt")));
    }
}
