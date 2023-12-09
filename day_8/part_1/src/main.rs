use std::io::Error;

#[derive(Debug, Clone)]
struct Path {
    current: String,
    right: String,
    left: String,
}

fn main() {
    let input = parse_input().unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let steps: Vec<char> = lines[0].chars().collect();
    let paths = get_paths(lines[2..].to_vec());

    let mut current_path: &Path = paths.iter().find(|path| path.current == "AAA").unwrap();

    let mut steps_taken = 0;

    'outer: loop {
        for step in &steps {
            steps_taken += 1;

            let next_direction = match step {
                'R' => &current_path.right,
                'L' => &current_path.left,
                _ => unimplemented!(),
            };

            current_path = &paths
                .iter()
                .find(|path| path.current == *next_direction)
                .unwrap();

            if current_path.current == "ZZZ" {
                break 'outer;
            }
        }
    }

    println!("Steps taken: {steps_taken}");
}
fn get_paths(lines: Vec<&str>) -> Vec<Path> {
    let mut paths = vec![];

    for line in lines {
        let string_line = line.replace("(", "").replace(")", "");

        if let [current, left_right] = string_line.split(" = ").collect::<Vec<&str>>()[..] {
            if let [left, right] = left_right.split(", ").collect::<Vec<&str>>()[..] {
                paths.push(Path {
                    current: current.to_string(),
                    right: right.to_string(),
                    left: left.to_string(),
                });
            }
        }
    }

    paths
}

fn parse_input() -> Result<String, Error> {
    std::fs::read_to_string("input.txt".to_string())
}
