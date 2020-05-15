use std::env;
use std::process;
use builder;
use utils;

fn main() {
    let argumnets = utils::parser_args_to_vec(env::args());
    
    let _args = builder::builder(argumnets).unwrap_or_else(|err| {
        eprint!("{}", err);
        process::exit(1);
    });
}
