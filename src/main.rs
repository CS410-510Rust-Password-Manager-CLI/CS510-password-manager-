use chrono::prelude::*;
use std::env;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;


/// Parse the given string as a `u32`.
fn parsenum(s: &str) -> u32 {
    let n: u32 = s.parse().unwrap();
    u32::from(n)
}

///generates a random alphanumeric string, with a length of the passed in integer
/// random character code from: (https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html)
fn genpass(length: u32) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();
        rand_string
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() >= 1, "arguments needed"); //make sure an argument is passed in
    let length: u32 = parsenum(&args[0]);


    let generated_pass = genpass(length);
    println!("{}", generated_pass);
    println!("{}", generated_pass.len());
    println!("{}", Utc::now().date().naive_utc());
}
