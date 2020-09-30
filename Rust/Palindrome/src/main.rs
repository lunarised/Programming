use std::io;
/*
 * Basic Rust Program to determine if a given String is a Palindrome
 *
 * Written for learning purposes only
 *
 * I am bad at rust
 */
fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("{}", is_palindrome(input));
        }
        Err(error) => println!("error: {}. Did not recieve a valid line!", error),
    }

}

fn is_palindrome(input: String) -> bool{
let mut first: char;
let mut next: char;
for i in 0..(input.len()/2){
first = input.chars().nth(i).unwrap();
next = input.chars().nth(input.len()-i-2).unwrap();
if first != next{
    return false;
}
}
return true;
}
