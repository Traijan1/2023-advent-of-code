use std::{collections::HashMap, io::Error};

fn main() {
    let mut digit = String::new();
    let mut numbers: Vec<i32> = vec![];
    let mut last_char = ' ';

    let spoken_to_letters = HashMap::from([
        ("eight", "8"),
        ("five", "5"),
        ("four", "4"),
        ("nine", "9"),
        ("one", "1"),
        ("seven", "7"),
        ("six", "6"),
        ("three", "3"),
        ("two", "2"),
    ]);

    let input = replace_spoken_to_digit(&spoken_to_letters, parse_input().unwrap());

    input.chars().enumerate().for_each(|(index, c)| {
        if is_one_digit(c) {
            if digit.len() == 0 {
                last_char = c;
                digit.push(c);
                println!("{c} = {digit}");
            } else {
                last_char = c;
            }
        }

        if c == '\n' || input.len() == index + 1 {
            digit.push(last_char);
            println!("Digit: {digit}");
            numbers.push(digit.parse().unwrap());

            digit.clear();
        }
    });

    println!("{:?}", numbers);
    println!("Sum is >{}<", numbers.iter().sum::<i32>());
}

fn somewhere_with(input: String, expected: String, start: u32) -> bool {
    let chars = input.chars().skip(start as usize).take(expected.len());
    let string = chars.collect::<String>();

    return expected == string;
}

fn is_one_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

fn replace_spoken_to_digit(map: &HashMap<&str, &str>, input: String) -> String {
    let vec: Vec<&str> = input.split("\n").collect();
    let mut new_vec: Vec<String> = vec![];
    let mut string = String::new();

    for str in vec {
        for (index, c) in str.to_string().chars().enumerate() {
            for key in map.keys() {
                let key_string = key.to_string();
                let mut chars = key_string.chars().peekable();

                if *chars.peek().unwrap() == c {
                    if somewhere_with(str.to_string(), key.to_string(), index as u32) {
                        string.push_str(map.get(key).unwrap());
                    }
                }
            }

            string.push(c);
        }

        new_vec.push(string.clone());
        string.clear();
    }

    new_vec.join("\n")
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
