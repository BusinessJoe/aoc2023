use std::{
    collections::{HashMap, VecDeque},
    io::{self, Read},
};

#[derive(Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conj { ins: Vec<String> },
}

#[derive(Clone)]
struct Module {
    t: ModuleType,
    name: String,
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
            (
                name_token[1..].to_string(),
                ModuleType::Conj { ins: vec![] },
            )
        } else {
            ("broadcaster".to_string(), ModuleType::Broadcaster)
        };

        Self {
            t,
            name,
            outs,
            state: false,
        }
    }
}

fn prop_signal(modules: &mut HashMap<String, Module>) -> (u32, u32, bool) {
    let mut high_count = 0;
    let mut low_count = 0;

    let mut rx_low = false;

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
                ModuleType::Conj { ref ins } => {
                    let all_ins = ins
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
        } else {
            // rx!
            if !incoming {
                println!("rx");
                rx_low = true;
            }
        }
    }

    (high_count, low_count, rx_low)
}

pub fn solution_1(input: &str) -> u32 {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(Module::parse)
        .map(|m| (m.name.clone(), m))
        .collect();

    // Populate ins of Conj modules
    for m in modules.clone().values() {
        for name in &m.outs {
            if let Some(Module {
                t: ModuleType::Conj { ins },
                ..
            }) = &mut modules.get_mut(name)
            {
                ins.push(m.name.clone());
            }
        }
    }

    let mut high = 0;
    let mut low = 0;

    for _ in 0..1000 {
        let (h, l, _) = prop_signal(&mut modules);
        high += h;
        low += l;
    }

    dbg!(high, low);
    high * low
}

pub fn solution_2(input: &str) -> usize {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(Module::parse)
        .map(|m| (m.name.clone(), m))
        .collect();

    // Populate ins of Conj modules
    for m in modules.clone().values() {
        for name in &m.outs {
            if let Some(Module {
                t: ModuleType::Conj { ins },
                ..
            }) = &mut modules.get_mut(name)
            {
                ins.push(m.name.clone());
            }
        }
    }

    let mut i = 1;
    loop {
        let (_, _, rx_low) = prop_signal(&mut modules);
        if rx_low {
            break;
        }
        i += 1;
    }

    i
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
