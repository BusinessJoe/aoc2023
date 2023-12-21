use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conj,
}

#[derive(Clone)]
struct Module {
    t: ModuleType,
    name: String,
    ins: Vec<String>,
    outs: Vec<String>,
    state: bool,
}

impl Module {
    fn parse(line: &str) -> Self {
        let (name_token, outs_token) = line.split_once(" -> ").unwrap();
        let outs: Vec<String> = outs_token.split(", ").map(|s| s.to_string()).collect();

        let (name, t) = if name_token.starts_with('%') {
            (name_token[1..].to_string(), ModuleType::FlipFlop)
        } else if name_token.starts_with('&') {
            (name_token[1..].to_string(), ModuleType::Conj)
        } else {
            ("broadcaster".to_string(), ModuleType::Broadcaster)
        };

        Self {
            t,
            name,
            ins: vec![],
            outs,
            state: false,
        }
    }
}

type ModuleMap = HashMap<String, Module>;

fn prop_signal(modules: &mut ModuleMap) -> (u32, u32) {
    let mut high_count = 0;
    let mut low_count = 0;

    let mut event_queue = VecDeque::new();
    event_queue.push_back(("broadcaster".to_string(), false));

    while let Some((name, incoming)) = event_queue.pop_front() {
        if incoming {
            high_count += 1;
        } else {
            low_count += 1;
        }

        if let Some(m) = modules.get(&name) {
            match m.t {
                ModuleType::Broadcaster => {
                    let m = modules.get_mut(&name).unwrap();
                    m.state = incoming;
                    for name in &m.outs {
                        event_queue.push_back((name.clone(), m.state));
                    }
                }
                ModuleType::FlipFlop => {
                    if !incoming {
                        let m = modules.get_mut(&name).unwrap();
                        m.state = !m.state;
                        for name in &m.outs {
                            event_queue.push_back((name.clone(), m.state));
                        }
                    }
                }
                ModuleType::Conj => {
                    let all_ins = m
                        .ins
                        .iter()
                        .map(|name| modules.get(name).unwrap().state)
                        .all(|b| b);
                    let m = modules.get_mut(&name).unwrap();
                    m.state = !all_ins;
                    for name in &m.outs {
                        event_queue.push_back((name.clone(), m.state));
                    }
                }
            }
        }
    }

    (high_count, low_count)
}

fn prop_signal_2(modules: &mut ModuleMap, monitor: &str) -> Vec<usize> {
    let mut high_times = Vec::new();

    let mut event_queue = VecDeque::new();
    event_queue.push_back(("broadcaster".to_string(), false, 0));

    while let Some((name, incoming, time)) = event_queue.pop_front() {
        if let Some(m) = modules.get(&name) {
            match m.t {
                ModuleType::Broadcaster => {
                    let m = modules.get_mut(&name).unwrap();
                    m.state = incoming;
                    for name in &m.outs {
                        event_queue.push_back((name.clone(), m.state, time + 1));
                    }
                }
                ModuleType::FlipFlop => {
                    if !incoming {
                        let m = modules.get_mut(&name).unwrap();
                        m.state = !m.state;
                        for name in &m.outs {
                            event_queue.push_back((name.clone(), m.state, time + 1));
                        }
                    }
                }
                ModuleType::Conj => {
                    let all_ins = m
                        .ins
                        .iter()
                        .map(|name| modules.get(name).unwrap().state)
                        .all(|b| b);
                    let m = modules.get_mut(&name).unwrap();
                    m.state = !all_ins;
                    for name in &m.outs {
                        event_queue.push_back((name.clone(), m.state, time + 1));
                    }
                }
            }

            let m = modules.get(&name).unwrap();
            if m.name == monitor && m.state {
                high_times.push(time);
            }
        }
    }

    high_times
}

fn parse_modules(input: &str) -> ModuleMap {
    let mut modules: ModuleMap = input
        .lines()
        .map(Module::parse)
        .map(|m| (m.name.clone(), m))
        .collect();

    // Populate ins
    for m_out in modules.clone().values() {
        for name in &m_out.outs {
            if let Some(m_in) = &mut modules.get_mut(name) {
                m_in.ins.push(m_out.name.clone());
            }
        }
    }

    modules
}

pub fn solution_1(input: &str) -> u32 {
    let mut modules = parse_modules(input);

    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        let (h, l) = prop_signal(&mut modules);
        high += h;
        low += l;
    }

    high * low
}

fn get_deps<'a>(name: &'a str, modules: &'a ModuleMap) -> Vec<&'a str> {
    let mut deps: HashSet<&str> = HashSet::new();

    let mut queue = vec![name];

    while let Some(name) = queue.pop() {
        if deps.contains(name) {
            continue;
        }
        deps.insert(name);
        for n in &modules.get(name).unwrap().ins {
            queue.push(n);
        }
    }

    let mut deps: Vec<&str> = deps
        .into_iter()
        .filter(|&name| {
            if let Some(Module {
                t: ModuleType::FlipFlop,
                ..
            }) = modules.get(name)
            {
                true
            } else {
                false
            }
        })
        .collect();

    deps.sort();
    deps
}

#[derive(Debug)]
struct HighLoop {
    num_states: usize,
    highs: HashSet<(usize, usize)>,
    loop_idx: usize,
}

impl HighLoop {
    fn new(monitor: &str, mut modules: ModuleMap) -> Self {
        let deps: Vec<String> = get_deps(monitor, &modules)
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut highs: HashSet<(usize, usize)> = HashSet::new();
        let mut num_states = 0;

        let mut seen: HashMap<Vec<bool>, usize> = HashMap::new();

        let loop_idx = loop {
            let state: Vec<bool> = deps
                .iter()
                .map(|name| modules.get(name).unwrap().state)
                .collect();
            if let Some(loop_idx) = seen.get(&state) {
                break *loop_idx;
            } else {
                seen.insert(state, num_states);
            }

            let high_times = prop_signal_2(&mut modules, monitor);
            for subtime in high_times {
                highs.insert((num_states, subtime));
            }
            num_states += 1;
        };

        Self {
            num_states,
            highs,
            loop_idx,
        }
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        (a, b) = (b, a);
    }

    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn solution_2(input: &str) -> usize {
    let modules = parse_modules(input);

    let mut lengths: Vec<usize> = Vec::new();
    let to_rx = modules
        .values()
        .find(|m| m.outs.contains(&"rx".to_string()))
        .unwrap();
    for name in &to_rx.ins {
        let high_loop = HighLoop::new(name, modules.clone());
        assert_eq!(1, high_loop.highs.len());
        lengths.push(high_loop.num_states);
    }

    // Answer is lcm of loop lengths
    lengths.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
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
