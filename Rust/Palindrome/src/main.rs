use std::io;
/*
 * Basic Rust Program to determine if a given String is a Palindrome
 *
 * Written for learning purposes only
 *
 * I am bad at rust
 */
#[cfg(test)]
 mod test{
     use crate::is_palindrome;
    
    #[test]
    fn is_palindrome_good_even(){
        assert_eq!(true, is_palindrome("123321".to_string()));
    }
    #[test]
    fn is_palindrome_good_odd(){
        assert_eq!(true, is_palindrome("1234321".to_string()));
    }
    #[test]
    fn is_palindrome_bad_even(){
        assert_eq!(false, is_palindrome("1b3321".to_string()));
    }
    #[test]
    fn is_palindrome_bad_odd(){
        assert_eq!(false, is_palindrome("1b34321".to_string()));
    }


 }
fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            input.pop();
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
next = input.chars().nth(input.len()-i-1).unwrap();
if first != next{
    return false;
}
}
return true;
}
