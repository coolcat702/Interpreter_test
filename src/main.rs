fn main() {
    let filename: String = get_input("Filename: ");
    let mut contents: Vec<String> = std::fs::read_to_string(filename)
        .unwrap_or_else(|_err: std::io::Error| {
            eprintln!("Error: Invalid filename");
            std::process::exit(1);
        })
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect::<Vec<_>>();
    let do_debug: bool = get_input("Debug: ")
        .to_lowercase()
        .contains("n");
    contents.iter_mut().for_each(|line: &mut String| {
        *line = line.split("//")
            .collect::<Vec<_>>()
                [0]
            .to_string();
    });
    if do_debug {
        println!("\nProgram: ");
        for line in &contents {
            println!("{}", line);
        }
        println!("");
    }
    let tape: Vec<bool> = get_input("Input: ")
        .chars()
        .map(|c: char| c == '1')
        .collect();
}

fn get_input(message: &str) -> String {
    println!("{}", message);
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Unable to read input");
    return input.trim().to_string();
}

fn debug() {
    println!("Program: ");
    let mut errors: i32 = 0;
    let mut paths: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let instructions = vec![String::new()];
    for line in &instructions {
        println!("{}", line);
    }
    println!();
    for (idx, state) in instructions.iter().enumerate() {
        for path in state.split(" ") {
            paths.insert(path.to_string(), idx as i32);
        }
        let mut visited_states: std::collections::HashSet<i32> = std::collections::HashSet::new();
        let mut jumped_states: std::collections::HashSet<i32> = std::collections::HashSet::new();
        for path in paths.keys() {
            for char in path.chars() {
                if is_valid(&char) {
                    continue;
                }
                println!("Invalid character {} in line {}", char, paths[&path]);
                errors += 1;
            }
            let split_path = remove_excess(path)
                .split(remove_excess(&path))
                .map(|s: &str| s.to_string())
                .collect::<Vec<_>>();
            if remove_excess(&path) != path {
                println!(
                    "Unreachable characters {:?} in line {}. Consider removing them",
                    split_path, paths[&path]
                );
                errors += 1;
            }
        }
    }
}




fn run(tape: &Vec<bool>, instructions: &Vec<String>, debug: bool) -> (Vec<bool>, i32, i32) {
    let mut tape: Vec<bool> = tape.clone();
    let mut pos: usize = 0;
    let mut idx: usize = 1;
    let mut time: i32 = 0;
    while instructions[idx - 1] != "END" {
        let instruction: &str = &instructions[idx - 1];
        if pos < 0{
            pos += 1;
            break
        }
        if pos >= tape.len() {
            pos -= 1;
            break
        }
        time += 1;
        let val: bool = tape[pos];
        let state: Vec<String> = instructions[idx-1]
            .split(" ")
            .map(|s: &str| s
                .to_string())
            .collect::<Vec<_>>();
        let mut to_run: String = state[0].to_owned();
        if state.len() != 1 {
            to_run = state[val as usize].to_owned();
        }
        let mut jump: i32 = 0;
        if debug {
            println!("State: {} On path {} ({}) at {}", idx, val as i32, to_run, pos);
        }
        for char in to_run.chars() {
            if is_digit(&char) {
                jump *= 10;
                jump += char as i32;
            }
        }
        for char in to_run.chars() {
            if is_digit(&char) {
                idx = jump as usize;
                break
            }
            match char {
                '>' => {
                    pos += 1;
                }
                '<' => {
                    pos -= 1;
                }
                '!' => {
                    tape[pos] = !val;
                }
                '{' => {
                    pos = 0;
                }
                '}' => {
                    pos = tape.len() - 1;
                }
                '/' => {
                    tape[pos] = false
                }
                '\\' => {
                    tape[pos] = true
                }
                _ => {
                    continue;
                }

            }
        }
    }

    return (tape, pos as i32, time);
}

fn is_digit(c: &char) -> bool {
    return c >= &'0' && c <= &'9';
}

fn is_valid(c: &char) -> bool {
    return is_digit(&c) || c == &'<' || c == &'>' || c == &'!' || c == &'{' || c == &'}' || c == &'/' || c == &'\\';
}

fn remove_excess(s: &String) -> String {
    let mut res: String = "".to_string();
    let mut found: bool = false;
    for char in s.chars() {
        if is_digit(&char) {
            res += char.to_string().as_str();
            found = true;
        } else if found {
            break
        } else {
            res += char.to_string().as_str();
        }
    }

    return res;
}
