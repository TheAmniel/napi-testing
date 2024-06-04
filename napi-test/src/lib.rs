#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn fibonacci(n: i32) -> i32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}