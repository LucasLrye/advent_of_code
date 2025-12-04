use std::fs;
use std::path::PathBuf;

fn main() {
    println!("Hello, 2025 Day 2!");
    let input = parse(PathBuf::from("./2025/02/src/text.txt"));
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
    let mut ret = 0;
    let id_range = input.split(',');
    for id in id_range {
        let number = id.trim().split_once('-').unwrap();
        // println!("number {:?}", number);
        let high_id = number.1.parse::<u64>().unwrap();
        let low_id = number.0.parse::<u64>().unwrap();
        let mut id = low_id;
        for _ in low_id..high_id + 1 {
            // println!("id {:?}", id);
            let id_str = format!("{}", id);
            let (left, right) = id_str.split_at(id_str.len() / 2);
            // println!("left {:?}, right {:?}", left, right);
            if left == right {
                ret += id;
                // println!("id {id}");
            }
            id += 1;
        }
    }
    ret
}

fn part_two(input: &str) -> u64 {
    let mut ret = 0;
    let id_range = input.split(',');
    for id in id_range {
        let number = id.trim().split_once('-').unwrap();
        // println!("number {:?}", number);
        let high_id = number.1.parse::<u64>().unwrap();
        let low_id = number.0.parse::<u64>().unwrap();
        let mut id = low_id;
        for _ in low_id..=high_id {
            if is_invalid(id) {
                ret += id;
            }
            id += 1;
        }
    }
    ret
}

fn is_invalid(id: u64) -> bool {
    // println!("id {:?}", id);
    let id_str = format!("{}", id);
    let id_str = id_str.as_bytes();

    for i in 1..=id_str.len() / 2 {
        let mut chunks = id_str.chunks(i);
        // println!("chnukns {:?}", chunks);
        let chunk = chunks.next().unwrap();
        if chunks.len() > 0 && chunks.all(|n| n == chunk) {
            return true;
        }
    }
    false
}

#[cfg(test)]
#[test]
fn test_2025_02_1() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
        .to_owned();
    let val = part_one(&input);
    assert_eq!(val, 1227775554);
    let val = part_two(&input);
    assert_eq!(val, 4174379265);
}
