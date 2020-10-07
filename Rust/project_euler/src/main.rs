fn main() {
    println!("{}", euler_5(10));
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


/* Broken
fn euler_5(max_in: i64) -> i64{
    let max: usize = max_in as usize;
    let mut trace: [usize; 11] = [0; 11];
    let mut sum = 0;
    for n in 1..max+1{
        println!("{}", n);
        if trace[n] == 0{
            sum += n;
            let mut i = 1;
            while (i*(n+1) < max+1){
                trace[i*n] = 1;
                i += 1;
                println!("{}", i);
            }
        }

    }
    return sum as i64;
}
*/