use std::env;
use std::fs;

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

impl Eq for Coords {}
impl PartialEq for Coords {

    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Coords {
    fn new(s: &str) -> Coords {
        let v: Vec<&str> = s.split(",").collect();
        return Coords {
            x: v[0].parse().unwrap(),
            y: v[1].parse().unwrap()
        }
    }
}

#[derive(Debug)]
struct Directions {
    from: Coords,
    to: Coords,
}

impl Eq for Directions {}

impl PartialEq for Directions {

    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Directions {
    fn new(s: &str) -> Directions {
        let values: Vec<&str> = s.split(" -> ").collect();
        return Directions {
            from: Coords::new(values[0]),
            to: Coords::new(values[1])
        }
    }
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<usize>>,
}

impl Board {
    fn new(directions: &Vec<Directions>) -> Board {
        let mut max: usize = 0;

        for direction in directions {
            max = max.max(direction.to.x).max(direction.from.x);
            max = max.max(direction.to.y).max(direction.from.y);
        }

        // Making sure we have max value + 1
        max = max + 1;
        return Board {
            board: vec![vec![0; max]; max],
        }
    }

    fn solve(&mut self, directions: &Vec<Directions>) {
        for direction in directions {
            if direction.to.x == direction.from.x {
                let stride = match direction.from.y < direction.to.y {
                    true => direction.from.y..=direction.to.y,
                    false => direction.to.y..=direction.from.y
                };
                for i in stride {
                    self.board[i as usize][direction.from.x as usize] += 1;
                }
            }
            if direction.to.y == direction.from.y {
                let stride = match direction.from.x < direction.to.x {
                    true => direction.from.x..=direction.to.x,
                    false => direction.to.x..=direction.from.x
                };
                for i in stride {
                    self.board[direction.from.y as usize][i as usize] += 1;
                }
            }
        }
    }

    fn print(self) {
        let mut count = 0;
        for i in self.board.into_iter() {
            for j in i {
                if j > 1 {
                    count += 1
                }
            }
        }
        println!("count: {}", count);
    }
}

fn main() {
    let filename: String = env::args().last().unwrap();
    let file = fs::read_to_string(filename).expect("Error: couldnt find file");
    let directions: Vec<Directions> = file.lines().map(|x| Directions::new(&x)).collect();
    let mut board = Board::new(&directions);
    board.solve(&directions);
    board.print();
}

