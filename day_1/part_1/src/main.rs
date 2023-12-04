use std::io::Error;

fn main() {
    let input = parse_input().unwrap();

    let mut digit = String::new();
    let mut numbers: Vec<i32> = vec![];
    let mut last_char = ' ';

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

fn is_one_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
