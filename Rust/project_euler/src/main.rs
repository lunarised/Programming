fn main() {
    println!("{}", euler_1(1000));
}
fn euler_1(max_number :i64) -> i64{
    let mut sum = 0;
    for i in 0..max_number{
        if i%3 == 0 || i%5 == 0{
            sum += i;
        }

    }
    return sum

}