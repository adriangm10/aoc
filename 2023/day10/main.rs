use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/10_input.txt")?;
    let map: Vec<_> = input.lines().collect();
    let start = start_pos(&map).unwrap();
    println!("{:?}", start);
    let (length, points) = cycle_len_area(&map, start);
    println!(
        "cycle length = {length}, farthest point from start = {}, points = {points}",
        length / 2
    );
    Ok(())
}

fn start_pos(map: &Vec<&str>) -> Option<(usize, usize)> {
    let pos = map
        .iter()
        .flat_map(|line| line.chars())
        .position(|c| c == 'S')?;
    Some((pos / map.len(), pos % map[0].len()))
}

fn cycle_len_area(map: &Vec<&str>, start: (usize, usize)) -> (usize, f64) {
    if start.0 > 0 {
        let c = &map[start.0 - 1][start.1..start.1 + 1];
        if c == "F" || c == "7" || c == "|" {
            let (is, len, points) = cycle_len_aux(map, start, (start.0 - 1, start.1));
            //println!();
            if is {
                return (len, points);
            }
        }
    }

    if start.0 < map.len() - 1 {
        let c = &map[start.0 + 1][start.1..start.1 + 1];
        if c == "|" || c == "J" || c == "L" {
            let (is, len, points) = cycle_len_aux(map, start, (start.0 + 1, start.1));
            //println!();
            if is {
                return (len, points);
            }
        }
    }

    if start.1 > 0 {
        let c = &map[start.0][start.1 - 1..start.1];
        if c == "-" || c == "L" || c == "F" {
            let (is, len, points) = cycle_len_aux(map, start, (start.0, start.1 - 1));
            //println!();
            if is {
                return (len, points);
            }
        }
    }

    if start.1 < map[start.0].len() {
        let c = &map[start.0][start.1 + 1..start.1 + 2];
        if c == "-" || c == "7" || c == "J" {
            let (is, len, points) = cycle_len_aux(map, start, (start.0, start.1 + 1));
            //println!();
            if is {
                return (len, points);
            }
        }
    }

    return (0, 0.0);
}

fn cycle_len_aux(
    map: &Vec<&str>,
    start: (usize, usize),
    (mut y, mut x): (usize, usize),
) -> (bool, usize, f64) {
    let mut prev = start;
    //println!("visited nodes: {:?}", visited);
    let mut len = 1;
    let mut vertices = vec![];
    let mut is_cycle = false;
    let first_pipe = (y, x);
    let mut last_pipe: (usize, usize) = (0, 0);

    loop {
        let c = &map[y][x..x + 1];
        len += 1;

        if (c == "-" || c == "7" || c == "J") && x > 0 {
            if (y, x - 1) == start && (y, x - 1) != prev {
                is_cycle = true;
                last_pipe = (y, x);
                break;
            }
            let c1 = &map[y][x - 1..x];
            if (c1 == "-" || c1 == "F" || c1 == "L") && (y, x - 1) != prev {
                println!("going to {c1} {:?} from {c} ({y}, {x})", (y, x - 1));
                prev = (y, x);
                if c != "-" {
                    vertices.push((y, x));
                }
                x -= 1;
                continue;
            }
        }

        if (c == "-" || c == "L" || c == "F") && x < map[y].len() {
            if (y, x + 1) == start && (y, x + 1) != prev {
                is_cycle = true;
                last_pipe = (y, x);
                break;
            }
            let c1 = &map[y][x + 1..x + 2];
            if (c1 == "-" || c1 == "J" || c1 == "7") && (y, x + 1) != prev {
                println!("going to {c1} {:?} from {c} ({y}, {x})", (y, x + 1));
                prev = (y, x);
                if c != "-" {
                    vertices.push((y, x));
                }
                x += 1;
                continue;
            }
        }

        if (c == "L" || c == "|" || c == "J") && y > 0 {
            if (y - 1, x) == start && (y - 1, x) != prev {
                is_cycle = true;
                last_pipe = (y, x);
                break;
            }
            let c1 = &map[y - 1][x..x + 1];
            if (c1 == "|" || c1 == "7" || c1 == "F") && (y - 1, x) != prev {
                println!("going to {c1} {:?} from {c} ({y}, {x})", (y - 1, x));
                prev = (y, x);
                if c != "|" {
                    vertices.push((y, x));
                }
                y -= 1;
                continue;
            }
        }

        if (c == "|" || c == "F" || c == "7") && y + 1 < map.len() {
            if (y + 1, x) == start && (y + 1, x) != prev {
                is_cycle = true;
                last_pipe = (y, x);
                break;
            }
            let c1 = &map[y + 1][x..x + 1];
            if (c1 == "|" || c1 == "J" || c1 == "L") && (y + 1, x) != prev {
                println!("going to {c1} {:?} from {c} ({y}, {x})", (y + 1, x));
                prev = (y, x);
                if c != "|" {
                    vertices.push((y, x));
                }
                y += 1;
                continue;
            }
        }

        break;
    }

    if !is_cycle {
        return (false, len, 0.0);
    }

    if first_pipe.0.abs_diff(last_pipe.0) == 1 {
        vertices.push(start);
    }

    println!("vertices: {:?}", vertices);

    let area = polygon_area(vertices);
    let points = area - (len as f64 / 2.0) + 1.0;
    return (true, len, points);
}

fn polygon_area(vertices: Vec<(usize, usize)>) -> f64 {
    let mut area = 0;
    for i in 0..vertices.len() {
        area += if i < vertices.len() - 1 {
            println!("determinant {:?}, {:?} = {}", vertices[i], vertices[i + 1], determinant(vertices[i], vertices[i + 1]));
            determinant(vertices[i], vertices[i + 1])
        } else {
            println!("determinant {:?}, {:?} = {}", vertices[i], vertices[0], determinant(vertices[i], vertices[0]));
            determinant(vertices[i], vertices[0])
        };
    }
    return area as f64 / 2.0;
}

fn determinant(p1: (usize, usize), p2: (usize, usize)) -> isize {
    return p1.0 as isize * p2.1 as isize - p2.0 as isize * p1.1 as isize;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn prueba1() {
        let vertices = vec![(1, 3), (2, 7), (4, 4), (6, 8), (7, 1)];
        assert!(polygon_area(vertices) == -20.5);
    }

    #[test]
    pub fn prueba2() {
        let vertices = vec![(6, 1), (1, 3), (2, 7), (4, 4), (5, 8)];
        assert!(polygon_area(vertices) == -16.5);
    }
}
