use std::io;

use std::io::prelude::*;
fn main(){
    print!("Input the FizzBuzz Cap: ");
    io::stdout().flush();
    let mut inputText = String::new();
    io::stdin()
        .read_line(&mut inputText)
        .expect("failed to read from stdin");

    let trimmed = inputText.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => fizzbuzz(i),
        Err(..) => println!("Given input: {} was not an integer", trimmed),
}
}
fn fizzbuzz(_max: isize){
    for x in 0.._max {
        if x%5 == 0{
            if x%3 == 0{
                println!("{}", "FizzBuzz");
            }else{
                println!("{}", "Buzz");
		}
        }else if x%3 == 0{
                println!("{}", "Fizz");
        }else{
                println!("{}", x);
        }
    }
    
}
