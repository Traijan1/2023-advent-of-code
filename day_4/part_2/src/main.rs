use std::io::Error;

fn main() {
    let input = parse_input().unwrap();
    let mut scratchcards: Vec<(usize, usize)> = vec![];

    input
        .split('\n')
        .enumerate()
        .map(|(index, line)| (index, line.replace(&format!("Card {}: ", index + 1), "")))
        .for_each(|(index, line)| {
            let splitted = line.split(" | ").collect::<Vec<&str>>();
            let numbers = get_numbers(splitted[0]);
            let winning = get_numbers(splitted[1]);

            let mut scratchcards_won: usize = 0;

            numbers.iter().for_each(|num| {
                winning.iter().for_each(|win| {
                    if num == win {
                        scratchcards_won += 1;
                    }
                })
            });

            scratchcards.push((index, scratchcards_won));
        });

    let mut result: Vec<(usize, usize, usize)> = vec![];

    for i in 0..scratchcards.len() {
        result.push((i, 1, 0));
    }

    for (card_index, cards_won_count) in scratchcards.iter() {
        for i in 1..=*cards_won_count {
            for _ in 0..result[*card_index].2 + 1 {
                result[card_index + i].2 += 1;
            }
        }
    }

    println!(
        "Sum: {}",
        result
            .iter()
            .map(|val| val.1 + val.2)
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>()
    );
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
