use std::io;
use std::char::DecodeUtf16;
fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            input.pop();
            println!("{}", xor_cipher(input));
        }
        Err(error) => println!("error: {}. Did not recieve a valid line!", error),
    }
}
fn xor_cipher(input: String) -> String{
    for letter in input.chars(){
        let a: u32 = 7;
        println!("{}", !7);
    }
    return "OOF".to_string();
}