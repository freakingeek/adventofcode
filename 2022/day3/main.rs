use std::fs;

fn letter_to_score(letter: char) -> i32 {
    match letter {
        'a'..='z' => (letter as u8 - b'a' + 1) as i32,
        'A'..='Z' => (letter as u8 - b'A' + 27) as i32,
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't load file!");
    let splitted_input = input.lines();

    let sum = splitted_input.fold(0, |sum, val| {
        let mid = val.len() / 2;
        let (left, right) = val.split_at(mid);

        let filtered: Vec<char> = right
            .chars()
            .filter(|letter| left.contains(*letter))
            .collect();

        return sum + letter_to_score(filtered[0]);
    });

    println!("Sum: {}", sum);
}
