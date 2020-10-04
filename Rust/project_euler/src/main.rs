fn main() {
    println!("{}", euler_2(1,2, 4_000_000));
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

fn euler_2(start_1 :i64, start_2 :i64, cap :i64) -> i64
{
    let mut current_max = start_2.clone();
    
    let mut hold;
    let mut v1 = start_1;
    let mut v2 = start_2;
    let mut sum = 0;
    if v1 % 2 == 0{
        sum += v1
    }
    while current_max < cap{
        if v2 % 2 == 0{
            sum += v2
        }
        
        hold = v2;
        v2 = v1+v2;
        v1 = hold;

        current_max = v2;
    }
    return sum;
}