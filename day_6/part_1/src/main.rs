use std::io::Error;

fn main() {
    let input = parse_input().unwrap();

    if let [time_line, distance_line] = input.split("\n").collect::<Vec<&str>>()[..] {
        let times = get_times(time_line.to_string());
        let distance_times = get_times(distance_line.to_string());

        let mut possible_wins: Vec<usize> = vec![];

        times.iter().enumerate().for_each(|(index, time)| {
            let mut possible_win = 0;

            for i in 2..*time {
                let hold = time - i;
                let distance = hold * (time - hold);

                if distance > distance_times[index] {
                    possible_win += 1;
                }
            }

            possible_wins.push(possible_win);
        });

        println!("{:?}", possible_wins);

        let mut multiplied = possible_wins[0];

        for possible in possible_wins.iter().skip(1) {
            multiplied *= possible;
        }

        println!("Possible wins multiplied: {}", multiplied);
    }
}

fn get_times(line: String) -> Vec<usize> {
    let mut splitted: Vec<String> = line.split(" ").map(|val| val.to_string()).collect();
    splitted = splitted[1..].to_vec();
    splitted.retain(|val| !val.is_empty());

    splitted
        .iter()
        .map(|val| val.parse::<usize>().unwrap())
        .collect()
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
