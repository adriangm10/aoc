use std::{io, fs};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/14_input.txt")?;
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    //println!("{:#?}", input.lines().collect::<Vec<&str>>());
    //let rotated = rotate_right(&input.lines().map(|line| line.to_string()).collect());
    //println!("{:#?}", rotated);
    //println!("{:#?}", tilt(&rotated));
    //let res = cycle(&input.lines().map(|line| line.to_string()).collect());
    //println!("{:#?}", res);
    //let res = cycle(&res);
    //println!("{:#?}", res);
    //let res = cycle(&res);
    //println!("{:#?}", res);
    Ok(())
}

fn part1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    let mut res = 0;
    for i in 0..lines[0].len() {
        let mut top = 0;
        res += input.lines().enumerate().map(|(j, line)|{
            match &line[i..i+1] {
                "#" => {
                    top = j + 1;
                    0
                },
                "." => 0,
                "O" => {
                    let charge = lines.len() - top;
                    top += 1;
                    charge
                },
                _ => unreachable!("unknown char"),
            }
        }).sum::<usize>();
        //println!("peso columna {i} = {column_charge}");
    }

    return res;
}

fn tilt(input: &Vec<String>) -> Vec<String> {
    let mut res = input.clone();
    for i in 0..res[0].len() {
        let mut top = 0;
        for j in 0..res.len() {
            match &res[j][i..i+1] {
                "#" => top = j + 1,
                "." => (),
                "O" => {
                    res[j].replace_range(i..i+1, ".");
                    res[top].replace_range(i..i+1, "O");
                    top += 1;
                },
                _ => unreachable!("unknown char"),
            };
        }
    }

    return res;
}

fn rotate_right(input: &Vec<String>) -> Vec<String> {
    let mut res = vec!["".to_string(); input[0].len()];
    for i in 0..input[0].len() {
        for j in 0..input.len() {
            //res[i][j..j+1] = &input[j][input[0].len() - i];
            let ind = input.len() - j - 1;
            res[i].push_str(&input[ind][i..i+1]);
            //println!("{:?}", res);
        }
    }

    return res;
}

fn cycle(input: &Vec<String>) -> Vec<String> {
    let mut res = input.clone();
    for _ in 0..4 {
        res = tilt(&res);
        res = rotate_right(&res);
    }

    return res;
}

fn north_load(input: &Vec<String>) -> usize {
    let mut res = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            res += match &input[i][j..j+1] {
                "#" | "." => 0,
                "O" => input.len() - i,
                _ => unreachable!("unknown char"),
            }
        }
    }
    return res;
}

fn part2(input: &str) -> usize {
    let mut visited: Vec<Vec<String>> = Vec::new();
    let mut res: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let mut ind = 1_000_000_000;

    for i in 0..1_000_000_000 {
        if let Some(first) = visited.iter().position(|x| *x == res) {
            let c = i - first;
            ind = (1_000_000_000 - first) % c + first;
            break;
        } else {
            visited.push(res.clone());
        }

        res = cycle(&res);
    }


    let res = &visited[ind];

    //for i in 0..visited.len() {
    //    println!("el ciclo {i} tiene peso {}", north_load(&visited[i]));
    //}

    north_load(&res)
}
