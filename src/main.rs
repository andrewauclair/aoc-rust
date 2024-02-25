use std::fs::read_to_string;

fn extract_calibration_value(str: String) -> u32 {
    let mut first = ' ';
    let mut last = ' ';

    for ch in str.chars() {
        if ch.is_numeric() {
            if first == ' ' {
                first = ch;
            }
            last = ch;
        }
    }

    let result = format!("{}{}", first, last);

    return result.parse::<u32>().unwrap();
}

fn main() {
    let mut result = Vec::new();

    for line in read_to_string("Input/2023/day_1.txt").unwrap().lines() {
        result.push(line.to_string());
    }

    let mut total = 0;

    for line in result {
        total += extract_calibration_value(line);
    }

    println!("Answer: {total}");
}
