use std::{
    collections::{BinaryHeap, HashSet},
    fs, io,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn next_moves(&self) -> &[Direction] {
        match self {
            Direction::North => &[Direction::North, Direction::West, Direction::East],
            Direction::South => &[Direction::South, Direction::West, Direction::East],
            Direction::West => &[Direction::West, Direction::North, Direction::South],
            Direction::East => &[Direction::East, Direction::North, Direction::South],
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Node {
    pos: (usize, usize),
    cost: usize,
    steps: u8,
    dir: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn possible_neighbours(&self, grid: &Vec<Vec<u8>>, min_steps: u8, max_steps: u8) -> Vec<Node> {
        let mut neighs = vec![];
        for dir in self.dir.next_moves() {
            if *dir != self.dir && self.steps < min_steps {
                continue;
            }

            if *dir == self.dir && self.steps == max_steps {
                continue;
            }

            match dir {
                Direction::North => {
                    if self.pos.0 == 0 {
                        continue;
                    }

                    neighs.push(Node {
                        pos: (self.pos.0 - 1, self.pos.1),
                        cost: self.cost + grid[self.pos.0 - 1][self.pos.1] as usize,
                        steps: if *dir == self.dir { self.steps + 1 } else { 1 },
                        dir: dir.clone(),
                    });
                }
                Direction::South => {
                    if self.pos.0 == grid.len() - 1 {
                        continue;
                    }

                    neighs.push(Node {
                        pos: (self.pos.0 + 1, self.pos.1),
                        cost: self.cost + grid[self.pos.0 + 1][self.pos.1] as usize,
                        steps: if *dir == self.dir { self.steps + 1 } else { 1 },
                        dir: dir.clone(),
                    });
                }
                Direction::West => {
                    if self.pos.1 == 0 {
                        continue;
                    }

                    neighs.push(Node {
                        pos: (self.pos.0, self.pos.1 - 1),
                        cost: self.cost + grid[self.pos.0][self.pos.1 - 1] as usize,
                        steps: if *dir == self.dir { self.steps + 1 } else { 1 },
                        dir: dir.clone(),
                    });
                }
                Direction::East => {
                    if self.pos.1 == grid.len() - 1 {
                        continue;
                    }

                    neighs.push(Node {
                        pos: (self.pos.0, self.pos.1 + 1),
                        cost: self.cost + grid[self.pos.0][self.pos.1 + 1] as usize,
                        steps: if *dir == self.dir { self.steps + 1 } else { 1 },
                        dir: dir.clone(),
                    });
                }
            }
        }

        return neighs;
    }
}

fn dijkstra(grid: &Vec<Vec<u8>>, min_steps: u8, max_steps: u8) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        pos: (0, 1),
        cost: grid[0][1] as usize,
        steps: 1,
        dir: Direction::East,
    });
    heap.push(Node {
        pos: (1, 0),
        cost: grid[1][0] as usize,
        steps: 1,
        dir: Direction::South,
    });

    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let mut visited = HashSet::new();

    while let Some(node) = heap.pop() {
        if node.pos == (grid_height - 1, grid_width - 1) && node.steps >= min_steps {
            return Some(node.cost);
        }

        if visited.contains(&(node.pos, node.steps, node.dir.clone())) {
            continue;
        }

        visited.insert((node.pos, node.steps, node.dir.clone()));

        for n in node.possible_neighbours(grid, min_steps, max_steps) {
            heap.push(n);
        }
    }

    None
}

fn parse_input(input: String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|c| c as u8)
                .collect()
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/17_test.txt")?;
    let grid = parse_input(input);
    let res1 = dijkstra(&grid, 1, 3).unwrap();
    let res2 = dijkstra(&grid, 4, 10).unwrap();
    println!("part 1: {res1}");
    println!("part 2: {res2}");
    Ok(())
}
