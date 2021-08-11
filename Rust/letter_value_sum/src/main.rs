use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    println!("{:?}", find_length_difference_with_same_value());
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
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


fn find_length_difference_with_same_value() -> Vec<String>{
    let mut values = Vec::new();
   let mut buckets = Vec::new();
    let contents = get_words_list();
    let word_list = contents.split_whitespace();
    let mut max_value: usize = 0;
    for word in word_list{
        if word.len() > max_value{
            max_value = word.len();
        }
    }
    for _i in 0..max_value{
        let tmp_handle = Vec::new();
        buckets.push(tmp_handle)

    }
    let pass2 = get_words_list();
    let word_list2 = pass2.split_whitespace();
    for word in word_list2{
        buckets[word.len()-1].push(word);
    }
    for bucket in 0..max_value-12{
        println!("{}", bucket);
        for word in &buckets[bucket]{
            for word2 in &buckets[bucket+11]{
                if letter_value_sum(word) == letter_value_sum(word2){
                    values.push(word.to_string());
                    values.push(word2.to_string());
                }
            }
        }
    }
    return values;
}