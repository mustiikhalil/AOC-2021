use std::env;
use std::fmt;
use std::fs;
use std::io::{Error, ErrorKind};

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl std::str::FromStr for Direction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item: Vec<&str> = s.split(" ").collect();
        return match item[0] {
            "forward" => Ok(Direction::Forward(item[1].parse()?)),
            "up" => Ok(Direction::Up(item[1].parse()?)),
            "down" => Ok(Direction::Down(item[1].parse()?)),
            _ => Err(Box::new(Error::new(ErrorKind::Other, "Couldnt parse!"))),
        };
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Direction::Forward(v) => write!(f, "Forward {}", v),
            Direction::Up(v) => write!(f, "Up {}", v),
            Direction::Down(v) => write!(f, "Down {}", v),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Directions")
            .field("x: ", &self.x)
            .field("y: ", &self.y)
            .field("aim: ", &self.aim)
            .finish()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let vec: Vec<&str> = contents.split("\n").collect();

    let i: Vec<Direction> = vec.iter().filter_map(|&x| x.parse().ok()).collect();
    let position = third_problem(i);
    let value = position.x * position.y;
    println!("position: {}", value)
}

fn third_problem(vector: Vec<Direction>) -> Position {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for v in vector {
        match v {
            Direction::Forward(count) => {
                x += count;
                y += aim * count;
            }
            Direction::Up(count) => aim -= count,
            Direction::Down(count) => aim += count,
        }
    }
    return Position {
        x: x,
        y: y,
        aim: aim,
    };
}
