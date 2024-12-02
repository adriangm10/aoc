use std::{collections::HashMap, fs, io, str::FromStr, usize};

#[derive(Debug)]
enum Operation {
    Greater((char, usize)),
    Less((char, usize)),
    None,
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<(Operation, &'a str)>, //operation and destination
}

#[derive(Debug)]
struct ParseRuleError;

impl<'a> Workflow<'a> {
    fn from_str(s: &'a str) -> Result<Self, ParseRuleError> {
        let (name, rules) = s.split_once("{").ok_or(ParseRuleError)?;
        let rules: Vec<_> = rules
            .split(",")
            .map(|rule| match rule.split_once(":") {
                Some((rule, dest)) => {
                    let cat = rule.chars().next().unwrap();
                    match &rule[1..2] {
                        ">" => (Operation::Greater((cat, rule[2..].parse().unwrap())), dest),
                        "<" => (Operation::Less((cat, rule[2..].parse().unwrap())), dest),
                        c => unreachable!("unknown rule {c}"),
                    }
                }
                None => (Operation::None, rule.trim_end_matches("}")),
            })
            .collect();

        Ok(Self { name, rules })
    }

    fn walk_part(&self, p: &Part) -> &str {
        for (rule, dest) in &self.rules {
            match rule {
                Operation::Greater((c, n)) => {
                    if p.get(c).unwrap() > *n {
                        return dest;
                    }
                }
                Operation::Less((c, n)) => {
                    if p.get(c).unwrap() < *n {
                        return dest;
                    }
                }
                Operation::None => return dest,
            }
        }
        ""
    }

    fn forward_range(&self, range: RangePart) -> Vec<(&str, RangePart)> {
        let mut ranges = vec![];

        let mut acc_range = range.clone();
        let mut rej_range = range;

        for (rule, dest) in &self.rules {
            match rule {
                Operation::Greater((c, n)) => {
                    let range = acc_range.get(c).unwrap();
                    if range.1 > *n {
                        acc_range.set_range(c, (*n + 1, range.1));
                        rej_range.set_range(c, (range.0, *n));
                    }
                }
                Operation::Less((c, n)) => {
                    let range = acc_range.get(c).unwrap();
                    if range.0 < *n {
                        acc_range.set_range(c, (range.0, *n - 1));
                        rej_range.set_range(c, (*n, range.1));
                    }
                }
                Operation::None => (),
            }
            ranges.push((*dest, acc_range));
            acc_range = rej_range.clone();
        }

        return ranges;
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug)]
struct ParsePartError;

impl FromStr for Part {
    type Err = ParsePartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("{").trim_end_matches("}");
        let mut p = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for cat in s.split(",") {
            let (cat, n) = cat.split_once("=").ok_or(ParsePartError)?;
            match cat {
                "x" => p.x = n.parse().map_err(|_| ParsePartError)?,
                "m" => p.m = n.parse().map_err(|_| ParsePartError)?,
                "a" => p.a = n.parse().map_err(|_| ParsePartError)?,
                "s" => p.s = n.parse().map_err(|_| ParsePartError)?,
                c => unreachable!("unknown category {c}"),
            };
        }

        Ok(p)
    }
}

impl Part {
    fn get(&self, cat: &char) -> Option<usize> {
        match cat {
            'x' => Some(self.x),
            'm' => Some(self.m),
            'a' => Some(self.a),
            's' => Some(self.s),
            _ => None,
        }
    }

    fn value(&self) -> usize {
        self.x + self.m + self.s + self.a
    }
}

#[derive(Debug, Clone)]
struct RangePart {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl RangePart {
    fn new(min: usize, max: usize) -> Self {
        RangePart {
            x: (min, max),
            m: (min, max),
            a: (min, max),
            s: (min, max),
        }
    }

    fn combinations(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn set_range(&mut self, cat: &char, range: (usize, usize)) {
        match cat {
            'x' => self.x = range,
            'm' => self.m = range,
            'a' => self.a = range,
            's' => self.s = range,
            _ => (),
        }
    }

    fn get(&self, cat: &char) -> Option<(usize, usize)> {
        match cat {
            'x' => Some(self.x),
            'm' => Some(self.m),
            'a' => Some(self.a),
            's' => Some(self.s),
            _ => None,
        }
    }
}

fn part1(wkflows: &str, parts: &str) -> usize {
    let wkflows: HashMap<&str, Workflow> = wkflows
        .lines()
        .map(|line| {
            let wkflow = Workflow::from_str(line).unwrap();
            (wkflow.name, wkflow)
        })
        .collect();

    let parts: Vec<_> = parts
        .lines()
        .map(|line| Part::from_str(line).unwrap())
        .collect();

    let mut total = 0;
    for part in parts {
        let mut state = "in";
        while state != "R" && state != "A" {
            let wkflow = wkflows.get(state).unwrap();
            state = wkflow.walk_part(&part);
        }

        match state {
            "A" => total += part.value(),
            "R" => (),
            _ => unreachable!(),
        }
    }
    return total;
}

fn part2(wkflows: &str) -> usize {
    let wkflows: HashMap<&str, Workflow> = wkflows
        .lines()
        .map(|line| {
            let wkflow = Workflow::from_str(line).unwrap();
            (wkflow.name, wkflow)
        })
        .collect();

    let mut stack = vec![("in", RangePart::new(1, 4000))];
    let mut total = 0;

    while let Some((wkflow, range)) = stack.pop() {
        let wkflow = wkflows.get(wkflow).unwrap();

        for (dest, range) in wkflow.forward_range(range) {
            match dest {
                "A" => total += range.combinations(),
                "R" => (),
                _ => stack.push((dest, range)),
            }
        }
    }

    total
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/19_input.txt")?;
    let (wkflows, parts) = input.split_once("\n\n").ok_or(io::ErrorKind::Other)?;
    let total1 = part1(wkflows, parts);
    let total2 = part2(wkflows);
    println!("part1: {total1}, part2: {total2}");
    Ok(())
}
