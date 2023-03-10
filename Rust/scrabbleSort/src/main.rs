use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() {
    
    let file_result = File::open("wordlist.txt");
    let word_list = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let reader = BufReader::new(word_list);

    for attempted_line in reader.lines() {
        let successful_line = match attempted_line{
            Ok(line) => line,
            Err(e) => break,
        };
        let points = successful_line.chars().fold(0, |cur, nxt| cur + get_character_value(nxt));
        println!("{}  {}",successful_line, points);
    }
}





fn get_character_value(character: char) -> i32 {
    let uppercase_character = character.to_uppercase().collect::<String>().chars().nth(0).unwrap();

    match uppercase_character {
        'A'|'E'|'I'|'O'|'U'|'L'|'N'|'S'|'T'|'R' => return 1,
        'D'|'G' => return 2,
        'B'|'C'|'M'|'P' => return 3,
        'F'|'H'|'V'|'W'|'Y' => return 4,
        'K' => return 5,
        'J'|'X' => return 8,
        'Q'|'Z' => return 10,
        _ => return 0

    }
}