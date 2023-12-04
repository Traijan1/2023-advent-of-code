use std::io::Error;

fn main() {
    let input = parse_input().unwrap();
    let mut points: Vec<usize> = vec![];

    input
        .split('\n')
        .enumerate()
        .map(|(index, line)| line.replace(&format!("Card {}: ", index + 1), ""))
        .for_each(|line| {
            let splitted = line.split(" | ").collect::<Vec<&str>>();
            let numbers = get_numbers(splitted[0]);
            let winning = get_numbers(splitted[1]);

            let mut point: usize = 0;

            numbers.iter().for_each(|num| {
                winning.iter().for_each(|win| {
                    if num == win {
                        if point == 0 {
                            point = 1;
                        } else {
                            point *= 2;
                        }
                    }
                })
            });

            points.push(point);
        });

    println!("Sum: {}", points.iter().sum::<usize>());
}

fn get_numbers(numbers: &str) -> Vec<usize> {
    let mut result = vec![];

    numbers.split(" ").for_each(|num| {
        if let Ok(number) = num.parse::<usize>() {
            result.push(number);
        }
    });

    result
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
