use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<u8>>,
    is_solved: bool,
    uncalled_numbers: HashSet<i32>,
    last_number: Option<i32>,
    // (i32, i32) represents (hor, ver) positions of a value
    positions: HashMap<i32, (usize, usize)>,
    index: i32,
}

impl Board {
    fn new(s: &str) -> Board {
        let current_parsed_string: Vec<&str> = s.split("\n").collect();
        let mut uncalled_numbers: HashSet<i32> = HashSet::new();
        let mut map: HashMap<i32, (usize, usize)> = HashMap::new();
        let mut board: Vec<Vec<u8>> = Vec::new();
        for (i, value) in current_parsed_string.iter().enumerate() {
            let parsed: Vec<&str> = value.split_whitespace().collect();
            board.push(Vec::new());
            if parsed.is_empty() {
                break;
            }
            for (j, v) in parsed.iter().enumerate() {
                uncalled_numbers.insert(v.parse().unwrap());
                map.insert(v.parse().unwrap(), (i,j));
                board[i].push(0);
            }
        }
        return Board {
            board: board,
            is_solved: false,
            uncalled_numbers: uncalled_numbers,
            last_number: None,
            positions: map,
            index: 0,
        }
    }

    fn current_index(&mut self, index: i32) {
        self.index = index;
    }

    fn calculate(&self) -> i32 {
        let sum = self.uncalled_numbers.iter().fold(0, |acc, x| acc + x);
        return sum * self.last_number.unwrap();
    }

    fn check(&mut self, v: i32) -> bool {
        if self.is_solved {
            return false
        }
        match self.positions.get(&v) { 
            Some(&coords) => self.solve(v, coords),
            None => false
        }
    }

    fn solve(&mut self, v: i32, coords: (usize, usize)) -> bool {
        self.board[coords.0][coords.1] = 1;
        self.uncalled_numbers.remove(&v);
        return self.verify_board(v, coords.0, coords.1)
    }

    fn verify_board(&mut self, current_number: i32, x: usize, y: usize) -> bool {
        let solved = self.validate_win_hori(x)|| self.validate_win_vert(y);
        if solved {
            self.last_number = Some(current_number);
            self.is_solved = solved;
        }
        return solved;
    }

    fn validate_win_hori(&mut self, x: usize) -> bool {
        let mut won = true;
        for i in 0..5 {
            won = won && !(self.board[x][i as usize] == 0);
        }
        return won;
    }

    fn validate_win_vert(&mut self, x: usize) -> bool {
        let mut won = true;
        for i in 0..5 {
            won = won && !(self.board[i as usize][x] == 0);
        }
        return won;
    }

}


fn main() {
    let current_env: Vec<String> = env::args().collect();
    let filename = &current_env[1];
    let file = fs::read_to_string(filename).expect("requires the file to run");
    let mut data: Vec<&str> = file.split("\n\n").collect();
    let input: &str = data.remove(0);
    let inputs: Vec<i32> = input.split(",").map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<Board> = data.iter().map(|&x| Board::new(x)).collect();
    let mut index = 0;
    for v in &inputs {
        for board in &mut boards {
            if board.is_solved {
                continue;
            }
            if board.check(*v) {
                index += 1;
                board.current_index(index);
            }
        }
    }
    boards.sort_by(|x, y| x.index.cmp(&y.index));
    for board in boards {
        if board.is_solved {
            println!("board value: {}, {}, {}", board.calculate(), board.is_solved, board.index);
        }
    }
}