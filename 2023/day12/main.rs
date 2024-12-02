use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/12_input.txt")?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            if let Some((springs, nums)) = line.split_once(" ") {
                let nums: Vec<_> = nums
                    .split(",")
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();

                let mut unfolded_springs = springs.repeat(5);
                for j in 1..5 {
                    unfolded_springs.insert(springs.len() * j + j - 1, '?');
                }
                let nums = nums.repeat(5);
                count(unfolded_springs.to_string(), nums)
                //println!("{i}: {c}");
            } else {
                0
            }
        })
        .sum()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            if let Some((springs, nums)) = line.split_once(" ") {
                let nums: Vec<_> = nums
                    .split(",")
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();

                let c = count(springs.to_string(), nums);
                println!("line {line} = {c} arrangements\n");
                c
            } else {
                0
            }
        })
        .sum()
}

#[memoize::memoize]
fn count(line: String, nums: Vec<usize>) -> usize {
    if line == "" {
        return if nums == [] { 1 } else { 0 };
    }

    if nums == [] {
        return if line.contains("#") { 0 } else { 1 };
    }

    let mut res = 0;
    let char = &line[..1];

    if char == "." || char == "?" {
        res += count(line[1..].to_string(), nums.clone());
    }

    if char == "#" || char == "?" {
        if line.len() >= nums[0]
            && !line[..nums[0]].contains(".")
            && (line.len() == nums[0] || &line[nums[0]..nums[0] + 1] != "#")
        {
            if line.len() == nums[0] {
                res += count("".to_string(), Vec::from(&nums[1..]));
            } else {
                res += count(line[nums[0] + 1..].to_string(), Vec::from(&nums[1..]));
            }
        }
    }

    return res;
}

// fn count(mut line: String, nums: Vec<usize>) -> usize {
//     line.insert(0, '.');
//     line.insert(0, '.');
//     let mut table = vec![vec![0usize; line.len()]; nums.len() + 1];
// 
//     table[0][0] = 1;
//     for i in 1..line.len() {
//         table[0][i] = match &line[i..i + 1] {
//             "#" => 0,
//             "?" | "." => table[0][i - 1],
//             _ => unreachable!("unknown char"),
//         }
//     }
// 
//     for i in 1..table.len() {
//         for j in 0..line.len() {
//             table[i][j] = match &line[j..j + 1] {
//                 "#" => {
//                     if j > nums[i - 1] && line[j - nums[i - 1] + 1..j].chars().all(|c| c == '?' || c == '#') {
//                         table[i - 1][j - nums[i - 1] - 1]
//                     } else {
//                         0
//                     }
//                 }
//                 "." => if j > 0 { table[i][j - 1] } else { 0 },
//                 "?" => {
//                     //if j > nums[i - 1] {
//                     //    let sum: usize = nums[0..i].iter().sum();
//                     //    if i != 1 && j > sum && !line[j - sum - 1..j].chars().all(|c| c == '?' || c == '#') {
//                     //        table[i - 1][j - nums[i - 1] - 1]
//                     //    } else {
//                     //        table[i][j - 1] + table[i - 1][j - 2]
//                     //    }
//                     //} else {
//                     //    0
//                     //}
//                     //if j > nums[i - 1] {
//                     //    if line[j - nums[i - 1] + 1..=j].chars().find(|c| *c == '#').is_some() {
//                     //        table[i - 1][j - nums[i - 1] - 1]
//                     //    } else {
//                     //        table[i][j - 1] + table[i - 1][j - 2]
//                     //    }
//                     //} else {
//                     //    0
//                     //}
//                 }
// 
//                 _ => unreachable!("unknown char"),
//             }
//         }
//     }
//     println!("{:?}", table);
//     *table.last().unwrap().last().unwrap()
// }
