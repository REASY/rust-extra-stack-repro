use std::hint::black_box;

use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(Box<ErrorKind>);

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("IllegalFibonacciInputError: {0}")]
    IllegalFibonacciInputError(String),
    #[error("VeryLargeError:")]
    VeryLargeError([i32; 1024])
}


#[no_mangle]
pub fn fib2(n: u32) -> Result<u64, ErrorKind> {
    match n {
        0 => Err(ErrorKind::IllegalFibonacciInputError("zero is not a right argument to Fibonacci!".to_string())),
        1 | 2 => Ok(1),
        3 => Ok(2),
        _ => Ok(fib2(n - 1).unwrap() + fib2(n - 2).unwrap()),
    }
}


fn main() {
    use std::mem::size_of;
    println!("Size of Result<i32, Error>: {}", size_of::<Result<i32, Error>>());
    println!("Size of Result<i32, ErrorKind>: {}", size_of::<Result<i32, ErrorKind>>());

    let r = fib2(black_box(20)).unwrap();
    println!("r: {}", r);
}