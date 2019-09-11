fn main(){
    fizzbuzz(1000);
}
fn fizzbuzz(_max: i32){
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
