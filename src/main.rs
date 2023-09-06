use std::io;

fn main() {
    let result: f64;
    let mut op: String = String::new();

    println!("Enter the first number: ");
    let x: f64 = input_parser();

    println!("Enter the second number: ");
    let y: f64 = input_parser();

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number associated with the desired operation: ");

    io::stdin().read_line(&mut op).expect("Invalid Input");
    let op: i32 = match op.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => result = x / y,
        _ => {
            println!("Invalid selection");
            return;
        }
    }

    println!("The result is: {}", result);
}

fn input_parser() -> f64 {
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("Invalid Input");
    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return f64::NAN;
        }
    };

    return x;
}