fn main() {
    let input: Vec<i32> = vec![3, 2, 1, 0];
    let n = 2;
    println!("{:?}", flip_front(input, n));
}
fn flip_front(input_vector: Vec<i32>, flip_position: usize) -> Vec<i32> {
    let mut v1: Vec<i32> = input_vector[0..flip_position]
        .iter()
        .copied()
        .rev()
        .collect();
    let v2: Vec<i32> = input_vector[flip_position..input_vector.len()].to_vec();
    v1.extend(v2.iter().cloned());
    return v1;
}
