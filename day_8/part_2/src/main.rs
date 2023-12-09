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

    let mut current_paths: Vec<&Path> = paths
        .iter()
        .filter(|path| path.current.contains('A'))
        .collect();

    let mut steps_taken = 0;

    'outer: loop {
        for step in &steps {
            let mut vec: Vec<&Path> = vec![];

            steps_taken += 1;

            for current_path in &current_paths {
                let next_direction = match step {
                    'R' => &current_path.right,
                    'L' => &current_path.left,
                    _ => unimplemented!(),
                };

                let path = &paths
                    .iter()
                    .find(|path| path.current == *next_direction)
                    .unwrap();

                vec.push(path);
            }

            current_paths = vec;

            let result = current_paths.iter().all(|path| path.current.contains('Z'));

            if result {
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
