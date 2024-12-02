use std::{collections::VecDeque, fs, io, str::FromStr, usize};

#[derive(Debug, PartialEq, Eq)]
struct RangeValue {
    dest: usize,
    src: usize,
    len: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    ranges: Vec<RangeValue>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError(String);

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s
            .split_once("\n")
            .ok_or(ParseMapError("[ERROR] missing definition".to_string()))?;
        let ranges: Vec<_> = s
            .lines()
            .map(|line| {
                let range: Vec<_> = line
                    .split_terminator(" ")
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
                RangeValue {
                    dest: range[0],
                    src: range[1],
                    len: range[2],
                }
            })
            .collect();

        Ok(Map { ranges })
    }
}

fn walk_src(map: &Map, src: usize) -> usize {
    for range in &map.ranges {
        if let Some(n) = src.checked_sub(range.src) {
            if n < range.len {
                return range.dest + n;
            }
        }
    }
    src
}

fn walk_range(map: &Map, range: (usize, usize)) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut ranges_to_check = VecDeque::from([range]);

    for range in &map.ranges {
        let mut new_to_check = VecDeque::new();
        while let Some((mut start, mut len)) = ranges_to_check.pop_back() {
            //println!("checking range: ({start}, {len})");
            if start >= range.src && start < range.src + range.len {
                let dest_from = range.dest + start - range.src;
                if start + len <= range.src + range.len {
                    //println!("the range is a subrange of ({}, {})", range.src, range.len);
                    ranges.push((dest_from, len));
                } else {
                    let dest_len = range.src + range.len - start;
                    start = range.src + range.len;
                    len -= dest_len;
                    ranges.push((dest_from, dest_len));
                    new_to_check.push_front((start, len));
                    //println!("the range is inside but from > range.src ({}, {})", range.src, range.len);
                }
            } else if start < range.src && start + len > range.src {
                let dest_from = range.dest;
                if start + len <= range.src + range.len {
                    let dest_len = start + len - range.src;
                    len -= dest_len;
                    ranges.push((dest_from, dest_len));
                    new_to_check.push_front((start, len));
                    //println!("the range is inside but from <= range.src ({}, {})", range.src, range.len);
                } else {
                    let dest_len = range.len;
                    ranges.push((dest_from, dest_len));
                    let len1 = range.src - start;
                    let from2 = range.src + range.len;
                    let len2 = start + len - (range.src + range.len);
                    new_to_check.push_front((start, len1));
                    new_to_check.push_front((from2, len2));
                    //println!("({}, {}) is inside the range", range.src, range.len);
                }
            } else {
                //println!("the range is not in: ({}, {})", range.src, range.len);
                new_to_check.push_front((start, len));
            }
        }
        ranges_to_check = new_to_check;
    }

    ranges.extend(ranges_to_check.iter());
    return ranges;
}

fn get_seeds_maps(input: &str) -> Option<(Vec<usize>, Vec<Map>)> {
    let mut input_maps = input.split_terminator("\n\n");
    let (_, seeds) = input_maps.next()?.split_once(": ")?;

    let seeds: Vec<_> = seeds
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let maps: Vec<Map> = input_maps
        .filter_map(|map| Map::from_str(map).ok())
        .collect();

    Some((seeds, maps))
}

fn part1(input: &str) -> Option<usize> {
    let (mut seeds, maps) = get_seeds_maps(input)?;

    for map in maps {
        seeds = seeds
            .iter()
            .map(|seed| {
                //let dest = map_src(&map, *seed);
                //println!("{seed} -> {dest}");
                walk_src(&map, *seed)
            })
            .collect();
        //println!();
    }

    Some(*seeds.iter().min().unwrap())
}

fn part2(input: &str) -> Option<usize> {
    let (seeds, maps) = get_seeds_maps(input)?;

    let mut seed_ranges = Vec::new();

    for i in (0..seeds.len() - 1).step_by(2) {
        seed_ranges.push((seeds[i], seeds[i + 1]));
    }
    //println!("initial ranges: {:?}", seed_ranges);
    
    for map in maps {
        let mut new_ranges = Vec::new();
        for range in seed_ranges {
            new_ranges.append(&mut walk_range(&map, range));
        }
        seed_ranges = new_ranges;
        //println!("new ranges: {:?}", seed_ranges);
        //println!();
    }

    Some(seed_ranges.into_iter().fold(usize::MAX, |min, (from, _)| {
        if from < min { from } else { min }
    }))
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/5_input.txt")?;

    println!("part1: {}, part2: {}", part1(&input).unwrap(), part2(&input).unwrap());

    Ok(())
}
