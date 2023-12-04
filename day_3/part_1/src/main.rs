use std::{io::Error, str::Chars};

#[derive(Debug, Clone, Copy)]
struct Number {
    value: usize,
    place: (i32, i32),
    line: i32,
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    value: char,
    place: i32,
    line: i32,
}

fn main() {
    let input = parse_input().unwrap();
    let mut chars = input.chars();
    let mut skip = 0;
    let mut current_number = Number {
        value: 0,
        place: (0, 0),
        line: 0,
    };
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let mut current_line = 1i32;
    let mut current_index = 0i32;

    for (index, char) in chars.clone().enumerate() {
        current_index += 1;

        if char == '\n' {
            current_line += 1;
            current_index = 0;
        }

        if skip != 0 {
            skip -= 1;
            continue;
        }

        if is_digit(char) {
            (skip, current_number) = get_number(chars.clone(), index, current_line, current_index);
            numbers.push(current_number);
        } else if char != '.' && char != '\n' {
            symbols.push(Symbol {
                value: char,
                place: (current_index - 1),
                line: current_line,
            });
        }
    }

    let mut numbers_next_to_symbol: Vec<Number> = vec![];

    for symbol in symbols {
        for number in numbers.clone() {
            for place in number.place.0 - 1..=number.place.1 + 1 {
                if symbol.line - 1 == number.line && symbol.place == place {
                    numbers_next_to_symbol.push(number);
                    break;
                } else if symbol.line + 1 == number.line && symbol.place == place {
                    numbers_next_to_symbol.push(number);
                    break;
                }
            }

            if symbol.line == number.line && symbol.place == number.place.0 - 1 {
                numbers_next_to_symbol.push(number);
            } else if symbol.line == number.line && symbol.place == number.place.1 + 1 {
                numbers_next_to_symbol.push(number);
            }
        }
    }

    println!(
        "{:?}",
        numbers_next_to_symbol
            .iter()
            .map(|number| number.value)
            .sum::<usize>()
    );
}

fn get_number(chars: Chars, skip: usize, current_line: i32, current_index: i32) -> (usize, Number) {
    let mut string = String::new();

    for char in chars.skip(skip) {
        if is_digit(char) {
            string.push(char);
        } else {
            break;
        }
    }

    let number = Number {
        value: string.parse().unwrap(),
        place: (current_index - 1, current_index + (string.len() as i32) - 2),
        line: current_line,
    };

    (string.len() - 1, number)
}

fn is_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
