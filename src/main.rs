use std::io::{self, Write};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-fibonacci/include/fibonacci.h");

        fn fibonacci(n: i32) -> Vec<i32>;
    }
}

fn main() {
    let mut n = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n.trim().parse()
        .expect("Please type a number!");


    let sequence = ffi::fibonacci(n);
    println!("{:?}", sequence);
}