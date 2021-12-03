use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let vec: Vec<&str> = contents.split("\n").collect();

    first_problem(vec.iter().map(|&x| int(x)).collect());
    second_problem(vec.iter().map(|&x| int(x)).collect());
}

fn first_problem(vector: Vec<i32>) {
    let value = sum_of_depth(vector);
    println!("value: {}", value);
}

fn second_problem(vector: Vec<i32>) {
    let new_vector = sum_of_threes(vector);
    let value = sum_of_depth(new_vector);
    println!("value: {}", value);
}

fn sum_of_depth(vector: Vec<i32>) -> i32 {
    let mut value = 0;
    let mut pre_value = -1;
    for item in vector {
        if pre_value == -1 {
            pre_value = item;
            continue;
        }
        if item > pre_value {
            value = value + 1;
        }
        pre_value = item;
    }
    return value;
}

fn sum_of_threes(vector: Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for (i, _) in vector.iter().enumerate() {
        let second_index = i + 1;
        let third_index = i + 2;
        if third_index >= vector.len() {
            break;
        }
        let value = vector[i] + vector[second_index] + vector[third_index];
        vec.push(value);
    }
    return vec;
}

fn int(item: &str) -> i32 {
    return item.parse::<i32>().unwrap();
}
