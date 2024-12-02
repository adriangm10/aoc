use std::{fs::File, io::Read};

fn main(){
    let mut f = File::open("input.txt").expect("Couldn't open the file");
    let mut guide: String = String::new();
    f.read_to_string(&mut guide).expect("Couldn't read the file");
    let guide: Vec<&str> = guide.split("\n").collect();
    let mut line: Vec<&str>;
    let mut res: Vec<u32> = Vec::new();
    for i in guide{
        line = i.split(" ").collect();
        if line.len() == 2 {
            res.push(match (line[0], line[1]) {
                ("A", "X") => 3 + 0,
                ("A", "Y") => 1 + 3,
                ("A", "Z") => 2 + 6,
                ("B", "X") => 1 + 0,
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,
                ("C", "X") => 2 + 0,
                ("C", "Y") => 3 + 3,
                ("C", "Z") => 1 + 6,
                (_, _) => 0,
            });
        }
    }
    let score: u32 = res.iter().sum();
    println!("Your final score is: {}", score);
}
