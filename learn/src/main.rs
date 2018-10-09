use std::io;
use std::collections::HashMap;

fn idiot_fib(n: u64) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        idiot_fib(n - 1) + idiot_fib(n - 2)
    }
}

fn initialize_map() -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 1);
    map
}

fn smart_fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    if !map.contains_key(&n) {
        let new_value = smart_fib(n - 1, map) + smart_fib(n - 2, map);
        map.insert(n, new_value);   
    }

    map[&n]
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

        println!("Calling smart fibonacci...");
        let mut map = initialize_map();

        for i in 0..number {
            println!("Fib({:}) -> {:}", i, smart_fib(i, &mut map));
        }
    }
    println!("Bye")
}
