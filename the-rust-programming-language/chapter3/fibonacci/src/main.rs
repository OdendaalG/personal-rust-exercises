use std::io;

fn main() {
    println!("Enter your fibonacci number:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Enter a number please."),
    };

    let result: u32 = fibonacci(n);

    println!("Fibonacci for N:{n} is '{result}'");
}

fn fibonacci(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
