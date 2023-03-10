use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("{:?}", find_length_difference_with_same_value());
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn letter_value_sum(input_string: &str) -> u32 {
    input_string.chars().map(|c| c as u32 - 96).sum()
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
    let mut values = Vec::new();
    let mut buckets = Vec::new();
    let contents = get_words_list();
    let word_list: Vec<&str> = contents.split_whitespace().collect();

    let max_value = word_list.iter().map(|word| word.len()).max().unwrap();

    for _i in 0..max_value {
        buckets.push(Vec::new());
    }
    for word in &word_list {
        buckets[word.len() - 1].push(word);
    }
    let mut scores = std::collections::HashMap::new();
    for word in &word_list {
        scores.insert(word, letter_value_sum(&word));
    }

    for bucket in 0..max_value - 12 {
        println!("{}", bucket);
        for word in &buckets[bucket] {
            for word2 in &buckets[bucket + 11] {
                if scores.get(word) == scores.get(word2) {
                    values.push(word.to_string());
                    values.push(word2.to_string());
                }
            }
        }
    }
    return values;
}
