extern crate rand;

use rand::Rng;
use std::{
    error::Error,
    env,
    iter,
};

use rand::thread_rng;

fn main() {
    let mut args = env::args();
    args.next();
    if let Some(arg) = args.next() {
        let num = match arg.trim().parse::<usize>() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("{} is not a valid number", arg);
                eprintln!("Error: {}", e.description());
                return
            }
        };

        let chars = random_chars(num);

        println!("{}", chars);
    } else {
        eprintln!("you need to put the amount of characters to generate");
    }
}

fn random_chars(num: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|_| (0x20u8 + (rng.gen::<f32>() * 96.0) as u8) as char)
        .take(num)
        .collect()
}