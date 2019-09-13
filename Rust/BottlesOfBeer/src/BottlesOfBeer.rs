use std::io;
use std::io::prelude::*;

fn main(){
    let mut in_num:u64 = 0;
    print!("How Many Bottles of Beer do you have? ");
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
beers(in_num);
}

fn beers(mut _iter: u64){
    
    if _iter <= 0{
        return
    }
    if _iter == 1{
        println!("{} Bottle of beer on the wall", _iter);
        println!("{} Bottle of beer!", _iter);
        println!("Take one down, and pass it around");
        _iter -= 1;
        println!("No more bottles of beer on the wall");
        return
    }
    else{
        println!("{} Bottles of beer on the wall", _iter);
        println!("{} Bottles of beer!", _iter);
        println!("Take one down, and pass it around");
        _iter -= 1;
        println!("{} Bottles of beer on the wall", _iter);
        println!("");
        beers(_iter);
    }
}
