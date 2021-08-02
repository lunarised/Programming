use std::env;
use std::fs;
use std::fs::File;
use std::str::SplitWhitespace;
fn main() {
    println!("{:?}", find_length_difference_with_same_value());
}
fn letter_value_sum(input_string: &str) -> u32 {
    let mut sum: u32 = 0;
    let tidy_string = input_string.to_lowercase();
    for character in tidy_string.chars() {
        sum += character as u32 - 96;
    }
    return sum;
}

fn find_word_with_set_value(letter_value: u32) -> Vec<String> {
    let mut vec = Vec::new();
    let contents = get_words_list();
    let word_list = contents.split_whitespace();
    for word in word_list {
        if (letter_value_sum(word) == letter_value) {
            vec.push(word.to_string());
        }
    }
    return vec;
}

fn get_words_list() -> String {
    let filename = "enable1.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents;
}

fn find_length_difference_with_same_value() -> Vec<String> {
    let mut vec = Vec::new();
    let contents = get_words_list();
    let word_list = contents.split_whitespace();
    for word in word_list {
        let contents2 = contents.clone();
        let word_list2 = contents2.split_whitespace();
        for word2 in word_list2 {
            println!("{}", word2);
            if word.len() + 11 == word2.len() || word.len() == word2.len() + 11 {
                if letter_value_sum(word) == letter_value_sum(word2) {
                    vec.push(word.to_string());
                    vec.push(word2.to_string());
                    println!("{} and {}", word, word2);
                }
            }
        }
    }
    return vec;
}
