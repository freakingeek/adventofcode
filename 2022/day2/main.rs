use std::fs;

fn letter_to_score(letter: char) -> i32 {
    match letter {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}

fn game_status(user: char, computer: char) -> &'static str {
    match user {
        'A' => match computer {
            'X' => "draw",
            'Y' => "win",
            'Z' => "lose",
            _ => "",
        },
        'B' => match computer {
            'X' => "lose",
            'Y' => "draw",
            'Z' => "win",
            _ => "",
        },
        'C' => match computer {
            'X' => "win",
            'Y' => "lose",
            'Z' => "draw",
            _ => "",
        },
        _ => "",
    }
}

fn game_status_to_score(status: &str) -> i32 {
    match status {
        "win" => 6,
        "draw" => 3,
        "lose" => 0,
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File can't be loaded!");
    let splitted_input = input.split('\n');
    
    let mut num = 0;

    for value in splitted_input {
        let mut values = value.split(' ');
        if let (Some(user), Some(computer)) = (values.next(), values.next()) {
            let status = game_status(user.chars().next().unwrap(), computer.chars().next().unwrap());
            num += game_status_to_score(status) + letter_to_score(computer.chars().next().unwrap());
        }
    }

    println!("{}", num);
}