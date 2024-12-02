use std::{collections::HashMap, fs, io};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Symbol {
    x: usize,
    y: usize,
    symb: char,
}

#[derive(Debug)]
struct Num {
    x: i32,
    y: i32,
    w: i32,
    num: u32,
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/3_input.txt")?;

    let mut sum = 0;
    let mut gear_ratio = 0;
    let lines: Vec<_> = input.lines().collect();
    let mut num: Num = Num {
        x: 0,
        y: 0,
        w: 0,
        num: 0,
    };
    let mut symbol: Option<Symbol> = None;
    let mut symbols: HashMap<Symbol, Vec<u32>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, chr) in line.char_indices() {
            //println!("i: {}, j: {}, char: {}", i, j, chr);
            if chr == '.' {
                if let Some(symb) = symbol.as_ref() {
                    if num.num != 0 && check_num_symbol(&num, symb) {
                        //println!("summing num: {}", num.num);
                        sum += num.num;
                        if symb.symb == '*' {
                            if symbols.contains_key(symb) {
                                symbols.get_mut(symb).unwrap().push(num.num);
                            } else {
                                symbols.insert(symb.clone(), vec![num.num]);
                            }
                        }
                        num.num = 0;
                    } else {
                        symbol = check_above_under(&lines, i, j);
                        //println!("num: {:?}, symbol {:?}", num, symbol);
                        if let Some(symb) = symbol.as_ref() {
                            if num.num != 0 && check_num_symbol(&num, symb) {
                                //println!("summing num: {}", num.num);
                                sum += num.num;
                                if symb.symb == '*' {
                                    if symbols.contains_key(symb) {
                                        symbols.get_mut(symb).unwrap().push(num.num);
                                    } else {
                                        symbols.insert(symb.clone(), vec![num.num]);
                                    }
                                }
                                num.num = 0;
                            }
                        }
                    }
                } else {
                    symbol = check_above_under(&lines, i, j);
                    //println!("num: {:?}, symbol {:?}", num, symbol);
                    if let Some(symb) = symbol.as_ref() {
                        if num.num != 0 && check_num_symbol(&num, symb) {
                            //println!("summing num: {}", num.num);
                            sum += num.num;
                            if symb.symb == '*' {
                                if symbols.contains_key(symb) {
                                    symbols.get_mut(symb).unwrap().push(num.num);
                                } else {
                                    symbols.insert(symb.clone(), vec![num.num]);
                                }
                            }
                            num.num = 0;
                        }
                    } else {
                        num.num = 0;
                    }
                }

                continue;
            }

            match chr.to_digit(10) {
                Some(d) => {
                    if num.num == 0 {
                        num.x = j as i32;
                        num.y = i as i32;
                        num.w = 0;
                    }

                    num.num = num.num * 10 + d;

                    num.w += 1;

                    if let Some(check) = check_above_under(&lines, i, j) {
                        symbol = Some(check);
                    }

                    if let Some(symb) = symbol.as_ref() {
                        if j == line.len() - 1 && check_num_symbol(&num, symb) {
                            //println!("summing num: {}", num.num);
                            sum += num.num;
                            if symb.symb == '*' {
                                if symbols.contains_key(symb) {
                                    symbols.get_mut(symb).unwrap().push(num.num);
                                } else {
                                    symbols.insert(symb.clone(), vec![num.num]);
                                }
                            }
                            num.num = 0;
                            symbol = None;
                        }
                    }
                }
                None => {
                    symbol = Some(Symbol {
                        x: j,
                        y: i,
                        symb: chr,
                    });
                    let symb = symbol.as_ref().unwrap();
                    if num.num != 0 && check_num_symbol(&num, symb) {
                        //println!("summing num: {}", num.num);
                        sum += num.num;
                        if symb.symb == '*' {
                            if symbols.contains_key(symb) {
                                symbols.get_mut(symb).unwrap().push(num.num);
                            } else {
                                symbols.insert(symb.clone(), vec![num.num]);
                            }
                        }
                        num.num = 0;
                    }
                    if j == line.len() - 1 {
                        symbol = None;
                    }
                }
            }
        }
    }

    if let Some(symb) = symbol.as_ref() {
        //println!("num: {:?}, symb: {:?}", num, symb);
        if num.num != 0 && check_num_symbol(&num, symb) {
            sum += num.num;
        }
    }

    for value in symbols.values() {
        if value.len() == 2 {
            gear_ratio += value[0] * value[1];
        }
    }

    println!("sum = {}, gear ratio = {}", sum, gear_ratio);

    Ok(())
}

fn check_above_under(lines: &Vec<&str>, i: usize, j: usize) -> Option<Symbol> {
    if i > 0 {
        let c = lines[i - 1].chars().nth(j).unwrap();
        if &lines[i - 1][j..j + 1] != "." && !c.is_digit(10) {
            return Some(Symbol {
                x: j,
                y: i - 1,
                symb: c,
            });
        }
    }

    if i < lines.len() - 1 {
        let c = lines[i + 1].chars().nth(j).unwrap();
        if &lines[i + 1][j..j + 1] != "." && !c.is_digit(10) {
            return Some(Symbol {
                x: j,
                y: i + 1,
                symb: c,
            });
        }
    }

    None
}

fn check_num_symbol(num: &Num, symb: &Symbol) -> bool {
    if num.y.abs_diff(symb.y as i32) <= 1 {
        if num.x + num.w >= (symb.x as i32) && num.x - 1 <= (symb.x as i32) {
            return true;
        }
    }
    return false;
}
