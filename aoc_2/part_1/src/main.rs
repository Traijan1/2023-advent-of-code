use std::{collections::HashMap, io::Error};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        for set in self.sets.clone() {
            if !set.is_possible(red, green, blue) {
                // println!("{:?}", set);
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug)]
struct Set {
    cubes: Vec<Cube>,
}

impl Set {
    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        for cube in self.cubes.clone() {
            if cube.color == Color::Red {
                let check = cube.count <= red;

                if !check {
                    return false;
                }
            }

            if cube.color == Color::Green {
                let check = cube.count <= green;

                if !check {
                    return false;
                }
            }

            if cube.color == Color::Blue {
                let check = cube.count <= blue;

                if !check {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Clone, Copy, Debug)]
struct Cube {
    color: Color,
    count: usize,
}

fn main() {
    let input = parse_input().unwrap();
    let games = get_games(input);

    let mut possible_games: Vec<Game> = vec![];

    for game in games {
        if game.is_possible(12, 13, 14) {
            possible_games.push(game);
        }
    }

    println!("{:?}", possible_games);

    let sum_of_id: usize = possible_games.iter().map(|game| game.id).sum();

    println!("Sum of possible games: {}", sum_of_id);
}

fn get_games(input: String) -> Vec<Game> {
    let mut games = vec![];
    let mut game_id = 1;

    for line in input.split("\n") {
        let without_game = line.replace(&format!("Game {}: ", game_id), "");
        let mut current_game = Game {
            id: game_id,
            sets: vec![],
        };

        for set in without_game.split("; ") {
            let mut current_set = Set { cubes: vec![] };

            for cube in set.split(", ") {
                let mut current_cube = Cube {
                    color: Color::Red,
                    count: 0,
                };

                let values: Vec<&str> = cube.split(" ").collect();

                current_cube.count = values[0].parse().unwrap();
                current_cube.color = get_color(cube);
                current_set.cubes.push(current_cube);
            }

            current_game.sets.push(current_set);
        }

        games.push(current_game);
        game_id += 1;
    }

    games
}

fn get_color(cube: &str) -> Color {
    if cube.ends_with("red") {
        return Color::Red;
    } else if cube.ends_with("blue") {
        return Color::Blue;
    } else if cube.ends_with("green") {
        return Color::Green;
    } else {
        return Color::Red;
    }
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
