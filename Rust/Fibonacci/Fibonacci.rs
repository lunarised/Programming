use std::io;
use std::io::prelude::*;

fn main(){
    let mut in_num:u64 = 0;
    print!("How Many Fibonacci Numbers do you want? ");
    io::stdout().flush();
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u64>(){
        Ok(i) => in_num = i,
        Err(..) => println!("Given input: {} was not an integer", trimmed),
    }
fibonacci(in_num,1,1);
}

fn fibonacci(_iter: u64, a: u64, b: u64){
    if _iter == 0{
        return
    }
    println!("{}", a);    
    fibonacci(_iter -1, b, a+b);

}
