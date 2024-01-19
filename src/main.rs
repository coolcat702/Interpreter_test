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
        debug(&contents);
    }
    let tape: Vec<bool> = get_input("Input: ")
        .chars()
        .map(|c: char| c == '1')
        .collect();
    let (res, pos, time) = run(&tape, &contents, do_debug);
    print_state(res, pos, time);
}

fn get_input(message: &str) -> String {
    println!("{}", message);
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Unable to read input");
    return input.trim().to_string();
}

fn print_state(state: Vec<bool>, pos: i32, time: i32) {
    let mut res: String = ""
        .to_string();
    for i in 0..state
        .len() {
        res += state[i]
            .to_string()
            .as_str();
    }
    println!("\nOutput: {}\n", res);
    println!("Cursor at: {}", pos);
    println!("Took {} iterations", time);
}
    
fn debug(instructions: &Vec<String>) {
    println!("Program: ");
    let mut errors: i32 = 0;
    let mut paths: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    instructions
        .into_iter()
        .for_each(|line: &String| {
            println!("{}", line);
    });
    let mut visited_states: std::collections::HashSet<i32> = std::collections::HashSet::new();
    let mut jumped_states: std::collections::HashSet<i32> = std::collections::HashSet::new();
    let all_states: std::collections::HashSet<i32> = (1..instructions.len() as i32).collect();
    println!();
    for (idx, state) in instructions
        .iter()
        .enumerate() {
        for path in state.split(" ") {
            paths.insert(path.to_string(), idx as i32);
        }
        for path in paths.keys() {
            for char in path.chars() {
                if !is_valid(&char) {
                    println!("Invalid character {} at line {}", char, paths[path]);
                    errors += 1;
                }
            }
            let split_path = remove_excess(&path)
                .split(&remove_excess(&path))
                .map(|s: &str| s.to_string())
                .collect::<Vec<_>>();
            if &remove_excess(&path).trim() != &path.trim() {
                println!(
                    "Unreachable characters {:?} in line {}",
                    split_path, paths[path]
                );
                errors += 1;
            }
            visited_states.insert(paths[path] + 1);
            jumped_states.insert(
                path.chars()
                    .filter(|x: &char| x.is_digit(10))
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
            );
        }
    }
    let unused_states: std::collections::HashSet<i32> =
        all_states.difference(&visited_states).cloned().collect();
    let invalid_states: std::collections::HashSet<i32> =
        jumped_states.difference(&all_states).cloned().collect();
    let states_str: String = all_states
        .iter()
        .map(|state: &i32| state.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("States: {}", states_str);
    for unused_state in unused_states {
        println!("Unused state {}", &unused_state+1);
    }
    for invalid_state in invalid_states {
        println!("Invalid jump to uninitialized state {}", &invalid_state+1);
    }
    println!("Debugging finished with {} errors found.\n", errors);
}

fn run(tape: &Vec<bool>, instructions: &Vec<String>, debug: bool) -> (Vec<bool>, i32, i32) {
    let mut tape: Vec<bool> = tape.clone();
    let mut pos: i32 = 0;
    let mut idx: i32 = 1;
    let mut time: i32 = 0;
    while instructions[idx as usize-1] != "END" {
        if pos < 0 {
            pos += 1;
            break;
        }
        if pos >= tape.len() as i32 {
            pos -= 1;
            break;
        }
        time += 1;
        let val: bool = tape[pos as usize];
        let state: Vec<String> = instructions[idx as usize-1]
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
                idx = jump;
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
                    tape[pos as usize] = !val;
                }
                '{' => {
                    pos = 0;
                }
                '}' => {
                    pos = tape.len() as i32 - 1;
                }
                '/' => {
                    tape[pos as usize] = false
                }
                '\\' => {
                    tape[pos as usize] = true
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
    return is_digit(c) || c == &'<' || c == &'>' || c == &'!' || c == &'{' || c == &'}' || c == &'/' || c == &'\\';
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
