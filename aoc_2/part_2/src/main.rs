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
                return false;
            }
        }

        true
    }

    fn power_of_game(&self) -> usize {
        let mut least_red = 0;
        let mut least_green = 0;
        let mut least_blue = 0;

        for set in self.sets.clone() {
            for cube in set.cubes {
                if cube.color == Color::Red && cube.count > least_red {
                    least_red = cube.count;
                }

                if cube.color == Color::Green && cube.count > least_green {
                    least_green = cube.count;
                }

                if cube.color == Color::Blue && cube.count > least_blue {
                    least_blue = cube.count;
                }
            }
        }

        println!("{} {} {}", least_red, least_green, least_blue);

        least_blue * least_green * least_red
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

    let mut sum_of_power = 0;

    for game in games {
        sum_of_power += game.power_of_game();
    }

    println!("Power of games is: {}", sum_of_power);
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
