use std::env;
use std::fs;

#[derive(Debug)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(s: &str) -> Coords {
        let v: Vec<&str> = s.split(",").collect();
        println!("s: {:?}", v);
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
    board: Vec<Vec<i32>>,
}

impl Board {
    fn new(directions: &Vec<Directions>) -> Board {
        let mut max_y = 0;
        let mut max_x = 0;

        for direction in directions {
            max_x = std::cmp::max(max_x, direction.to.x);
            max_x = std::cmp::max(max_x, direction.from.x);
            max_y = std::cmp::max(max_y, direction.to.y);
            max_y = std::cmp::max(max_y, direction.from.y);
        }
        let mut new_vec: Vec<Vec<i32>> = Vec::new();

        let grid = std::cmp::max(max_x + 1, max_y + 1);
        for i in 0..grid {
            new_vec.push(Vec::new());
            for _ in 0..grid {
                new_vec[i as usize].push(0);
            }
        }
        return Board {
            board: new_vec,
        }
    }

    fn solve(&mut self, directions: &Vec<Directions>) {
        for direction in directions {
            println!("direction: {:?}", direction);
            if direction.to.x == direction.from.x {
                let stride = if direction.from.y < direction.to.y {
                    direction.from.y..direction.to.y+1
                } else {
                    direction.to.y..direction.from.y+1
                };
                for i in stride {
                    self.board[i as usize][direction.from.x as usize] += 1;
                }
            }
            if direction.to.y == direction.from.y {
                let stride = if direction.from.x < direction.to.x {
                    direction.from.x..direction.to.x+1
                } else {
                    direction.to.x..direction.from.x+1
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
    let env: Vec<String> = env::args().collect();
    let filename = &env[1];
    let file = fs::read_to_string(filename).expect("Error: couldnt find file");
    let directions: Vec<Directions> = file.split("\n").map(|x| Directions::new(&x)).collect();
    let boxed = Box::new(directions);
    let mut board = Board::new(&boxed);
    board.solve(&boxed);
    board.print();
}

