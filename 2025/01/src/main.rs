use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, 2025 Day 1!");
    let input = parse(PathBuf::from("./2025/01/src/text.txt"));
    let val1 = part_one(&input);
    println!("part1 is {:?}", val1);
    let val2 = part_two(&input);
    println!("part2 is {:?}", val2);
}

fn parse(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}

fn part_one(input: &str) -> u64 {
    let mut zero = 0;
    let mut dial: i64 = 50;
    for line in input.lines() {
        if line.is_empty() {
            return zero;
        }
        // println!("line : {}", line);
        let operator: char = line.chars().next().unwrap();
        // println!("operator {}", operator);
        let line_number: String = line.chars().skip(1).take(line.len() - 1).collect();
        // println!("line number : {}", line_number);
        let number = line_number.parse::<i64>().unwrap();
        if operator == 'L' {
            dial -= number;
        } else {
            dial += number;
        }
        dial = dial.rem_euclid(100);
        // println!("dial {}", dial);
        if dial == 0 {
            zero += 1;
        }
    }
    zero
}

fn part_two(input: &str) -> u64 {
    let mut zero = 0;
    let mut dial: i64 = 50;
    for line in input.lines() {
        if line.is_empty() {
            return zero;
        }
        // println!("line : {}", line);
        let operator: char = line.chars().next().unwrap();
        // println!("operator {}", operator);
        let line_number: String = line.chars().skip(1).take(line.len() - 1).collect();
        // println!("line number : {}", line_number);
        let number = line_number.parse::<i64>().unwrap();
        for _ in 0..number {
            if operator == 'L' {
                dial -= 1;
            } else {
                dial += 1;
            }
            dial = dial.rem_euclid(100);
            if dial == 0 {
                zero += 1;
            }
        }
        // println!("dial {}", dial);
    }
    zero
}

#[cfg(test)]
#[test]
fn test_2025_01_1() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
    .to_owned();
    let val = part_one(&input);
    assert_eq!(val, 3);
}
