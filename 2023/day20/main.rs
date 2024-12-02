use std::{io, fs, collections::{HashMap, VecDeque}};

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn opposit(&self) -> Pulse {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module<'a> {
    name: &'a str,
    dests: Vec<&'a str>,
    t: ModuleType,
    state: Pulse,
    mem: Vec<(&'a str, Pulse)>,
}

impl<'a> Module<'a> {
    fn new(name: &'a str, t: ModuleType, dests: Vec<&'a str>) -> Self {
        Module { name, dests, t, state: Pulse::Low, mem: Vec::new() }
    }

    fn from_str(s: &'a str) -> Self {
        match &s[0..1] {
            "%" => {
                let (name, dests) = s.split_once(" -> ").unwrap();
                let dests = dests.split(", ").collect();
                let name = &name[1..];
                Self::new(name, ModuleType::FlipFlop, dests)
            }
            "&" => {
                let (name, dests) = s.split_once(" -> ").unwrap();
                let dests = dests.split(", ").collect();
                let name = &name[1..];
                Self::new(name, ModuleType::Conjunction, dests)
            },
            "b" => {
                let (_, dests) = s.split_once(" -> ").unwrap();
                let dests = dests.split(", ").collect();
                Self::new("broadcaster", ModuleType::Broadcaster, dests)
            }
            _ => unreachable!("unknown module"),
        }
    }

    //dest, source, pulse
    fn transmit(&mut self, pulse: &Pulse, from: &str) -> Vec<(&str, &str, Pulse)> {
        let mut output = Vec::new();
        match self.t {
            ModuleType::FlipFlop => {
                if *pulse == Pulse::Low {
                    self.state = self.state.opposit();
                    for d in &self.dests {
                        output.push((*d, self.name, self.state.clone()));
                    }
                }
            }

            ModuleType::Conjunction => {
                for i in 0..self.mem.len() {
                    if self.mem[i].0 == from {
                        self.mem[i].1 = pulse.clone();
                    }
                }

                self.state = if self.mem.iter().all(|x| x.1 == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for d in &self.dests {
                    output.push((*d, self.name, self.state.clone()));
                }
            }

            ModuleType::Broadcaster => {
                for d in &self.dests {
                    output.push((*d, self.name, Pulse::Low));
                }
            }
        };
        return output;
    }
}

// low, high
fn pulse(modules: &mut HashMap<&str, Module>) -> (usize, usize) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut inputs = VecDeque::from([("broadcaster", "", Pulse::Low)]);

    while let Some((name, from, pulse)) = inputs.pop_back() {
        if let Some(module) = modules.get_mut(name) {
            let outs = module.transmit(&pulse, from);
            inputs.extend(outs);
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            };
        }
    }

    return (low_pulses, high_pulses);
}

fn part1(mut modules: HashMap<&str, Module>) -> usize {
    let mut low_pulses = 1;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        let pulses = pulse(&mut modules);
        low_pulses += pulses.0;
        high_pulses += pulses.1;
    }
    return low_pulses * high_pulses;
}

fn parse(input: &str) -> HashMap<&str, Module> {
    let mut modules: HashMap<&str, Module> = input.lines().map(|line| {
        let m = Module::from_str(line);
        (m.name, m)
    }).collect();

    let mods: Vec<_> = modules.values().cloned().collect();

    for module in modules.values_mut() {
        if module.t == ModuleType::Conjunction {
            for m in &mods {
                for dest in &m.dests {
                    if *dest == module.name {
                        module.mem.push((m.name, Pulse::Low));
                    }
                }
            }
        }
    }

    return modules;
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/20_test.txt")?;
    let mut modules = parse(&input);
    //let res1 = part1(modules);
    println!("{modules:?}");
    pulse(&mut modules);
    //println!("{res1}");
    Ok(())
}
