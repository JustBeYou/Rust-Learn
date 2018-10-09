use std::io;

fn idiot_fib(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        idiot_fib(n - 1) + idiot_fib(n - 2)
    }
}

fn main() {
    println!("Hello to this fibonacci calculator!");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed reading from STDIN");

    let mut number = 0;
    let trimmed = input_text.trim();
    match trimmed.parse::<u64>() {
         Ok(i) => number = i,
         Err(..) => println!("This is not an integer"),
    };

    if number != 0 {
        println!("Calling idiot fibonacci...");
        for i in 0..number {
            println!("Fib({:}) -> {:}", i, idiot_fib(i));
        }
    }
    println!("Bye")
}
