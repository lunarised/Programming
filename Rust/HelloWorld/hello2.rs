fn main(){
    println!("{}", hello("Henry"));
    println!("NERD");
}
fn hello(_name: &str) -> String{
    let h = "Hello ".to_string();
    let out = h + &_name;
    return out;

}
