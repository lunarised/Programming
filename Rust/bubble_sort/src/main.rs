#[cfg(test)]
mod test{
    use crate::bubble_sort;
    #[test]
    fn reverse_test(){
        let test_vector = vec!(5,4,3,2,1);
        let ans_vector = vec!(1,2,3,4,5);
        assert_eq!(ans_vector, bubble_sort(test_vector));
    }
    #[test]
    fn no_sort_test(){
        let test_vector = vec!(1,2,3,4,5);
        let ans_vector = vec!(1,2,3,4,5); 
        assert_eq!(ans_vector, bubble_sort(test_vector));
    }
    #[test]
    fn negative_test(){
        let test_vector = vec!(-10, 20, 10, -20, 30, 40, 50, 0, -30);
        let ans_vector = vec!(-30, -20, -10, 0, 10, 20, 30, 40, 50); 
        assert_eq!(ans_vector, bubble_sort(test_vector));
    }
}
use std::io;
fn main() {
    let mut input = String::new();
    let output: Vec<i32>;
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let nums = input.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<i32>>();    
            output = bubble_sort(nums);
            println!("{:?}", output);
        }
        Err(error) => println!("error: {}. Did not recieve a valid line!", error),
    }
}
fn bubble_sort(input_vector: Vec<i32>) -> Vec<i32>{
    let mut processing_vector: Vec<i32> = input_vector;
    let mut hold;
    let length :usize = processing_vector.len() as usize;
    for i in (0..length-1).rev(){
        for j in i..length-1{
            if processing_vector[j]>processing_vector[j+1]{
                hold = processing_vector[j];
                processing_vector[j] = processing_vector[j+1];
                processing_vector[j+1] = hold;
            }
        }
    }
    return processing_vector;
}