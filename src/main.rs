use std::{
    io::{Write, stdout}, 
    process::exit
};

fn main() {
    println!("Hello!! This is the calculator program.");
    println!("Please input numerical expressions following prompts.");
    println!("\tExit the program\t-> input \"exit\"");
    println!("\tView the manual \t-> input \"help\"");
    while calculator() {}
    println!("Goodbye!!");
}

fn calculator() -> bool {
    print!("calc > ");
    stdout().flush().unwrap();

    let str = read_cmd();
    if str == "exit" {
        return false;
    }
    if str == "help" {
        show_help();
        return true;
    }

    match do_calc(str) {
        Some(result) => println!("result: {}", result),
        None => println!("Error. Please review the formula to make sure it is correct.")
    };

    return true;
}

fn read_cmd() -> String {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).ok();
    return str.trim().to_string();
}

fn show_help() {
    println!("Basic arithmetic");
    println!("    Addition                -> (example) 3 + 5  (return 8)");
    println!("    Subtracion              -> (example) 10 - 4 (return 6)");
    println!("    Multiplication          -> (example) 7 * 2  (return 14)");
    println!("    Division                -> (example) 15 / 6 (return 2)");
    println!("    Remainder               -> (example) 9 % 4  (return 1)");
    println!("Other examples");
    println!("    (example) 3 + 5 * 9     -> (return 48)");
    println!("    (example) (3 + 5) * 9   -> (return 72)");
    println!("    (example) (1 + 14) / 3  -> (return 5)");
    println!("Other commands");
    println!("    Exit the program        -> input \"exit\"");
    println!("    View the manual         -> input \"help\"");
}

fn do_calc(str: String) -> Option<i32> {
    let vec_str = str.chars().collect::<Vec<_>>();
    let mut num_stack: Vec<i32> = Vec::new();
    let mut op_stack = Vec::new();
    let len = str.len();

    let mut num_str = String::new();
    for i in 0..len {
        let c = vec_str[i];

        if c.is_ascii_whitespace() {
            continue;
        }

        if c.is_ascii_digit() {
            num_str.push(c);
        } else {
            if !num_str.is_empty() {
                let mut num: i32 = num_str.parse().unwrap();

                if !op_stack.is_empty() {
                    let op = op_stack.last().unwrap();
                    if *op == '*' || *op == '/' || *op == '%' {
                        let num2 = num_stack.pop().unwrap();
                        if *op == '*' {
                            num = num2 * num;
                        } else if *op == '/' {
                            num = num2 / num;
                        } else {
                            num = num2 % num;
                        }
                        op_stack.pop();
                    }
                }
                
                num_stack.push(num);
                num_str.clear();
            }

            if is_operator(c) {
                op_stack.push(c);
            } else if c == '(' {
                op_stack.push(c);
            } else if c == ')' {
                if !sub_calc(&mut num_stack, &mut op_stack) {
                    println!("Internal Error is occurred. This program has exited.");
                    exit(1);
                }
            } else {
                return None;
            }
        }
    }

    if !num_str.is_empty() {
        let num: i32 = num_str.parse().unwrap();
        num_stack.push(num);
    }

    sub_calc(&mut num_stack, &mut op_stack);

    return Some(num_stack.pop().unwrap());
}

fn sub_calc(num_stack: &mut Vec<i32>, op_stack: &mut Vec<char>) -> bool {
    while !op_stack.is_empty() {
        let op = op_stack.pop().unwrap();
        if op == '(' {
            break;
        }

        let num = num_stack.pop().unwrap();
        let num2 = num_stack.pop().unwrap();

        if op == '+' {
            num_stack.push(num2 + num);
        } else if op == '-' {
            num_stack.push(num2 - num);
        } else if op == '*' {
            num_stack.push(num2 * num);
        } else if op == '/' {
            num_stack.push(num2 / num);
        } else if op == '%' {
            num_stack.push(num2 % num);
        } else {
            return false;
        }
    }

    return true;
}

fn is_operator(op: char) -> bool {
    op == '+' || op == '-' || op == '*' || op == '/' || op == '%'
}