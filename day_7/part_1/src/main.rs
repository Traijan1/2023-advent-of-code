use std::hash::Hash;
use std::{collections::HashMap, io::Error};

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
struct Hand<'a> {
    hand: &'a str,
    bid: usize,
}

impl<'a> Hand<'a> {
    fn get_hand_type_score(&self, b: &Hand, get_score: impl Fn(char) -> usize) -> i32 {
        let a_type = self.get_hand_type();
        let b_type = b.get_hand_type();

        if a_type > b_type {
            1
        } else if a_type < b_type {
            -1
        } else {
            for (a_card, b_card) in self.hand.chars().zip(b.hand.chars()) {
                if a_card == b_card {
                    continue;
                }

                let a_score = get_score(a_card);
                let b_score = get_score(b_card);

                if a_score > b_score {
                    return 1;
                } else {
                    return -1;
                }
            }

            0
        }
    }

    fn get_hand_type(&self) -> usize {
        let chars: Vec<char> = self.hand.chars().collect();
        let count_vector = get_count(&chars);

        if count_vector[0] == 5 {
            7
        } else if count_vector[0] == 4 {
            6
        } else if count_vector[0] == 3 && count_vector[1] == 2 {
            5
        } else if count_vector[0] == 3 && count_vector[1] == 1 {
            4
        } else if count_vector[0] == 2 && count_vector[1] == 2 {
            3
        } else if count_vector[0] == 2 && count_vector[1] == 1 {
            2
        } else if count_vector[0] == 1 {
            1
        } else {
            println!("Did not found scoring for '{}'", self.hand);
            0
        }
    }
}

fn get_count<T>(iter: &Vec<T>) -> Vec<usize>
where
    T: PartialEq + Eq + Hash + Copy,
{
    let mut map: HashMap<T, usize> = HashMap::new();

    for value in iter.iter() {
        if map.contains_key(&value) {
            continue;
        }

        let count = iter.iter().filter(|c| *c == value).count();
        map.insert(*value, count);
    }

    let mut vec: Vec<usize> = Vec::from_iter(map.values())
        .iter()
        .map(|val| **val)
        .collect();

    vec.sort();
    vec.reverse();

    vec
}

fn main() {
    let input = parse_input().unwrap();
    let mut hands = get_hands(&input);

    let card_score = vec![
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    let get_score = |char| card_score.iter().position(|val| *val == char).unwrap();
    hands.sort_by(|a, b| a.get_hand_type_score(b, get_score).partial_cmp(&0).unwrap());

    let mut total_winnings = 0;

    for (index, hand) in hands.iter().enumerate() {
        total_winnings += (index + 1) * hand.bid;
    }

    println!("Toal winnings: {}", total_winnings);
}

fn get_hands<'a>(input: &'a str) -> Vec<Hand<'a>> {
    let mut hands = vec![];

    input.lines().for_each(|line| {
        if let [hand, bid] = line.split(" ").collect::<Vec<&str>>()[..] {
            hands.push(Hand {
                hand,
                bid: bid.parse().unwrap(),
            })
        }
    });

    hands
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
