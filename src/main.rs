mod args;
mod functions;

use args::Args;
use clap::Parser;
use rand::random;
use functions::{
    ConvertToSets,
    Set,
};

fn main() {
    let args = Args::parse();
    let sets = args.chars.convert_to_sets();
    let mut password = String::new();
    for _ in 0..args.length {
        let set = &sets[random::<usize>() % sets.len()];
        password.push(
            Set::get_chars(set)
                .chars()
                .nth(random::<usize>() % Set::get_chars(set).len())
                .unwrap()
        );
    }
    println!("{password}");
}
