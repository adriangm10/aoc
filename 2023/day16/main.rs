use std::{collections::HashSet, fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/16_input.txt")?;
    let input = input.lines().collect();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    Ok(())
}

fn process_ray(map: &Vec<&str>, start_pos: (i32, i32), start_dir: (i32, i32)) -> usize {
    let mut cells: HashSet<(i32, i32)> = HashSet::new();
    let mut rays = vec![(start_dir, start_pos)];

    while !rays.is_empty() {
    //for _ in 0..10 {
        //println!("rays = {:?}", rays);
        let ray = rays.remove(0);
        let mut curr_dir = ray.0;
        let mut curr_pos = ray.1;
        //println!("inserting {:?}", curr_pos);
        //cells.insert(curr_pos);

        while curr_pos.0 >= 0
            && curr_pos.0 < map.len() as i32
            && curr_pos.1 >= 0
            && curr_pos.1 < map[0].len() as i32
        {
            //println!("pos: {:?}, dir: {:?}", curr_pos, curr_dir);

            match &map[curr_pos.0 as usize][curr_pos.1 as usize..(curr_pos.1 + 1) as usize] {
                "." => (),
                "\\" => {
                    if curr_dir.1 != 0 {
                        curr_dir.0 = if curr_dir.1 > 0 { 1 } else { -1 };
                        curr_dir.1 = 0;
                    } else {
                        curr_dir.1 = if curr_dir.0 > 0 { 1 } else { -1 };
                        curr_dir.0 = 0;
                    }
                }
                "|" => {
                    if curr_dir.0 == 0 {
                        // if curr_pos == (8, 7) {
                        //     println!("hola");
                        // }
                        if !cells.contains(&curr_pos) {
                            rays.push(((1, 0), (curr_pos.0 + 1, curr_pos.1)));
                            rays.push(((-1, 0), (curr_pos.0 - 1, curr_pos.1)));
                            cells.insert(curr_pos);
                        }
                        break;
                    }
                }
                "/" => {
                    if curr_dir.1 != 0 {
                        curr_dir.0 = if curr_dir.1 > 0 { -1 } else { 1 };
                        curr_dir.1 = 0;
                    } else {
                        curr_dir.1 = if curr_dir.0 > 0 { -1 } else { 1 };
                        curr_dir.0 = 0;
                    }
                }
                "-" => {
                    if curr_dir.1 == 0 {
                        if !cells.contains(&curr_pos) {
                            rays.push(((0, 1), (curr_pos.0, curr_pos.1 + 1)));
                            rays.push(((0, -1), (curr_pos.0, curr_pos.1 - 1)));
                            cells.insert(curr_pos);
                        }
                        break;
                    }
                }
                _ => unreachable!("unknown char"),
            };

            //println!("inserting: {:?}\n", curr_pos);
            cells.insert(curr_pos);
            curr_pos.0 += curr_dir.0;
            curr_pos.1 += curr_dir.1;
        }
    }

    return cells.len();
}

fn part1(input: &Vec<&str>) -> usize {
    process_ray(input, (0, 0), (0, 1))
}

fn part2(input: &Vec<&str>) -> usize {
    let mut max = 0;

    for i in 0..input.len() {
        let res = process_ray(input, (i as i32, 0), (0, 1));

        if max < res {
            max = res;
        }

        let res = process_ray(input, (i as i32, (input[i].len() - 1) as i32), (0, -1));

        if max < res {
            max = res;
        }
    }

    for i in 0..input[0].len() {
        let res = process_ray(input, (0, i as i32), (1, 0));

        if max < res {
            max = res;
        }

        let res = process_ray(input, ((input.len() - 1) as i32, i as i32), (-1, 0));

        if max < res {
            max = res;
        }
    }

    return max;
}
