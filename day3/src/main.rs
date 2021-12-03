use std::env;
use std::fs;

#[derive(Debug)]
struct Varient {
    seen_bit_0: i32,
    seen_bit_1: i32,
}

impl Varient {
    fn new() -> Varient {
        Varient {
            seen_bit_0: 0,
            seen_bit_1: 0,
        }
    }
}

fn main() {
    let env: Vec<String> = env::args().collect();

    let filename = &env[1];
    let file = fs::read_to_string(filename).expect("Expects a file");

    let values: Vec<&str> = file.split("\n").collect();

    first_example(values);
    println!("Second problem");
    second_problem();
}

fn first_example(values: Vec<&str>) {
    let v: usize = values[0].len();

    let mut counts: Vec<Varient> = Vec::new();
    for _ in 0..v {
        counts.push(Varient::new());
    }

    for value in values {
        let bits: Vec<&str> = value.split("").filter(|&x| x != "").collect();
        for (index, str) in bits.iter().enumerate() {
            if *str == "" {
                continue;
            }
            let int: u8 = str.parse().unwrap();
            if int == 0 {
                counts[index].seen_bit_0 += 1;
            } else {
                counts[index].seen_bit_1 += 1;
            }
        }
    }

    let mut max_value = String::new();
    let mut min_value = String::new();

    for value in counts {
        if value.seen_bit_0 > value.seen_bit_1 {
            max_value += "0";
            min_value += "1";
        } else {
            max_value += "1";
            min_value += "0";
        }
    }
    let max = isize::from_str_radix(&max_value, 2).unwrap();
    let min = isize::from_str_radix(&min_value, 2).unwrap();
    println!("max: {}", max);
    println!("min: {}", min);
    println!("value: {}", (min * max));
}

fn second_problem() {
    let env: Vec<String> = env::args().collect();

    let filename = &env[1];
    let file = fs::read_to_string(filename).expect("Expects a file");
    let values: Vec<&str> = file.split("\n").collect();
    let max_value = get_min_max_value(values.iter().map(|&x| x).collect(), false);
    let min_value = get_min_max_value(values, true);

    println!("max: {}", max_value);
    println!("min: {}", min_value);
    let max = isize::from_str_radix(&max_value, 2).unwrap();
    let min = isize::from_str_radix(&min_value, 2).unwrap();
    println!("value: {}", (min * max));
}

fn get_min_max_value(vec: Vec<&str>, min: bool) -> &str {
    let mut vector: Vec<&str> = vec;
    let mut bit_position: usize = 0;
    while vector.iter().count() > 1 {
        vector = helper_func(vector, bit_position, min);
        bit_position += 1;
    }
    return vector[0];
}

fn helper_func(vec: Vec<&str>, bit_position: usize, min: bool) -> Vec<&str> {
    let zero = 48;
    let one = 49;
    println!("vec: {:?}", vec);
    let mut max_seen: Vec<&str> = Vec::new();
    let mut min_seen: Vec<&str> = Vec::new();
    let mut max_bit_0 = 0;
    let mut max_bit_1 = 0;
    for item in vec.iter() {
        let bytes = item.as_bytes();
        if bytes.len() <= 0 {
            continue;
        }
        let byte: u8 = bytes[bit_position];
        if byte == one {
            max_bit_1 += 1
        } else {
            max_bit_0 += 1
        }
    }
    for item in vec.iter() {
        let bytes = item.as_bytes();
        if bytes.len() <= 0 {
            continue;
        }
        if bytes[bit_position] == one && !min && max_bit_1 >= max_bit_0 {
            max_seen.push(item);
        } else if bytes[bit_position] == one && max_bit_1 < max_bit_0 {
            min_seen.push(item)
        } else if bytes[bit_position] == zero && min && max_bit_0 <= max_bit_1 {
            min_seen.push(item);
        } else if bytes[bit_position] == zero && max_bit_1 < max_bit_0 {
            max_seen.push(item);
        }
    }
    if min {
        return min_seen;
    }
    return max_seen;
}
