use std::io::Error;

fn main() {
    let input = parse_input().unwrap();

    if let [time_line, distance_line] = input.split("\n").collect::<Vec<&str>>()[..] {
        let time = get_time(time_line.to_string());
        let distance_time = get_time(distance_line.to_string());

        let mut possible_wins = 0;

        for i in 2..time {
            let hold = time - i;
            let distance = hold * (time - hold);

            if distance > distance_time {
                possible_wins += 1;
            }
        }

        println!("Possible wins: {}", possible_wins);
    }
}

fn get_time(line: String) -> usize {
    let mut splitted: Vec<String> = line.split(" ").map(|val| val.to_string()).collect();
    splitted = splitted[1..].to_vec();
    splitted.retain(|val| !val.is_empty());

    splitted.join("").parse().unwrap()
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
