extern crate rand;
use rand::{
    Rng,
    distributions::Alphanumeric
};
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

        let mut rng = thread_rng();
        let chars: String = iter::repeat(())
            .map(|_| rng.sample(Alphanumeric))
            .take(num)
            .collect();

        println!("{}", chars);
    } else {
        eprintln!("you need to put the amount of characters to generate");
    }
}