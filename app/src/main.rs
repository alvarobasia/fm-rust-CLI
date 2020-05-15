use std::env;
use std::process;
use builder;
use utils;
use colored::*;

fn main() {
    println!("\t\t\t{} {}", "FILE".red(), "MANAGER".blue());
    let mut argumnets = utils::parser_args_to_vec(env::args());
    
    let _args = builder::builder(&mut argumnets).unwrap_or_else(|err| {
        println!("type => fn help");
        eprint!("{}", err);
        process::exit(1);
    });
}
