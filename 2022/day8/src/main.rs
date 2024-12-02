use std::fs::File;
use std::io::prelude::*;

pub fn is_visible(matrix: &Vec<&str>, pos_x: usize, pos_y: usize) -> bool {
    let height = matrix[pos_y].chars().nth(pos_x).unwrap().to_digit(10).unwrap();
    println!("{}", matrix[pos_y]);
    let mut visible_x = true;
    let mut visible_y = true; 
    for i in 0..pos_x {
        if matrix[pos_y].chars().nth(i).unwrap().to_digit(10).unwrap() >= height {
            for j in pos_x+1..matrix[pos_y].len() {
                if matrix[pos_y].chars().nth(j).unwrap().to_digit(10).unwrap() >= height {
                    visible_x = false;
                    break;
                } 
            }
            break;
        }
    }
    if visible_x { return true; }
    for i in 0..pos_y {
        if matrix[i].chars().nth(pos_x).unwrap().to_digit(10).unwrap() >= height {
            for j in pos_y+1..matrix.len() {
                if matrix[j].chars().nth(pos_x).unwrap().to_digit(10).unwrap() >= height {
                    visible_y = false;
                    break;
                }
            }
            break;
        }
    }
    return visible_y;
}

pub fn scenic_score(matrix: &Vec<&str>, pos_x: usize, pos_y: usize) -> usize{
    let height = matrix[pos_y].chars().nth(pos_x).unwrap().to_digit(10).unwrap();
    let width = matrix[0].len();
    let mut score1: usize = 0;
    let mut score2: usize = 0;
    let mut score: usize;
    for i in (0..pos_y).rev() {
        score1 += 1;
        if matrix[i].chars().nth(pos_x).unwrap().to_digit(10).unwrap() >= height {
            break;
        }
    }
    for j in pos_y+1..matrix.len() {
        score2 += 1;
        if matrix[j].chars().nth(pos_x).unwrap().to_digit(10).unwrap() >= height {
            break;
        }
    }
    score = score1 * score2;
    score1 = 0;
    score2 = 0;
    for i in (0..pos_x).rev() {
        score1 += 1;
        if matrix[pos_y].chars().nth(i).unwrap().to_digit(10).unwrap() >= height {
            break;
        }
    }
    for j in pos_x+1..width {
        score2 += 1;
        if matrix[pos_y].chars().nth(j).unwrap().to_digit(10).unwrap() >= height {
            break;
        }
    }
    score *= score1 * score2;
    return score;
}

fn main() {
    let mut f = File::open("input.txt").expect("Couldn't open the file");
    let mut lines = String::new();
    f.read_to_string(&mut lines).expect("Couldn't read the file");
    let lines: Vec<&str> = lines.lines().collect();
    let width = lines[0].len();
    // let mut visibles = lines.len() * width;
    let mut max = usize::MIN;
    let mut score: usize;

    for line in 1..lines.len()-1 {
        for char in 1..width-1 {
            // parte 1
            // if !is_visible(&lines, char, line) {
            //     visibles -= 1;
            // }
            // parte 2
            score = scenic_score(&lines, char, line);
            if score > max {
                max = score;
            }
        }
    }
    println!("highest scenic score: {}", max);
}
