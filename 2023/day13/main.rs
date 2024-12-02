use std::{fs, io};

#[derive(Debug)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/13_test.txt")?;
    let patterns: Vec<_> = input.split("\n\n").collect();
    let res = patterns.iter().fold(0, |acc, pat| {
        match reflect(pat) {
            Reflection::Horizontal(r) => acc + 100 * r,
            Reflection::Vertical(r) => acc + r,
        }
    });

    println!("{res}");
    Ok(())
}

fn reflect(pat: &str) -> Reflection {
    let lines: Vec<&str> = pat.lines().collect();
    let mut i = 0;
    let mut reflected;
    let width = lines[0].len();

    loop {
        reflected = true;
        let mut l = i + 1;
        let mut r = i;

        while l > 0 && r < width - 1 {
            l -= 1;
            r += 1;
            if !lines.iter().all(|line| line[l..l+1] == line[r..r+1]) {
                reflected = false;
                break;
            }
        }

        if reflected {
            return Reflection::Vertical(i + 1);
        }

        i += 1;

        reflected = true;
        l = i + 1;
        r = i;

        while l > 0 && r < lines.len() - 1 {
            l -= 1;
            r += 1;
            if !lines[l].chars().zip(lines[r].chars()).all(|(c1, c2)| c1 == c2) {
                reflected = false;
                break;
            }
        }

        if reflected {
            return Reflection::Horizontal(i + 1);
        }
    }
}


