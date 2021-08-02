use std::io;

/*
 * Basic Rust Program which determines the balancing of brackets
 *
 * Written for learning purposes only
 *
 * I am bad at rust
 */

#[cfg(test)]
mod test {
    use crate::is_balanced;
    #[test]
    fn same_nesting() {
        assert_eq!(true, is_balanced("(())".to_string()));
    }
    #[test]
    fn odd_nesting() {
        assert_eq!(false, is_balanced("(()))()".to_string()));
    }
    #[test]
    fn no_close_nesting() {
        assert_eq!(false, is_balanced("(((((".to_string()));
    }
    #[test]
    fn multi_bracket() {
        assert_eq!(true, is_balanced("([]{[]}({}))".to_string()));
    }
    #[test]
    fn mismatch() {
        assert_eq!(false, is_balanced("(]".to_string()));
    }
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("{}", is_balanced(input));
        }
        Err(error) => println!("error: {}. Did not recieve a valid line!", error),
    }
}

fn is_balanced(input: String) -> bool {
    let mut stack: Vec<char> = vec![];
    let mut is_balanced_bool: bool = true;
    for i in 0..input.len() {
        match input.chars().nth(i).unwrap() {
            '(' => stack.push('('),
            '{' => stack.push('{'),
            '[' => stack.push('['),
            ')' => {
                if stack.len() > 0 && stack[stack.len() - 1] == '(' {
                    stack.pop();
                } else {
                    is_balanced_bool = false;
                    break;
                }
            }
            '}' => {
                if stack.len() > 0 && stack[stack.len() - 1] == '{' {
                    stack.pop();
                } else {
                    is_balanced_bool = false;
                    break;
                }
            }
            ']' => {
                if stack.len() > 0 && stack[stack.len() - 1] == '[' {
                    stack.pop();
                } else {
                    is_balanced_bool = false;
                    break;
                }
            }
            _ => (),
        }
    }
    if is_balanced_bool && stack.len() > 0 {
        is_balanced_bool = false;
    }
    return is_balanced_bool;
}
