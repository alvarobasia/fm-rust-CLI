use std::env;
use std::process;
use builder;


fn main() {
    let _args = builder::builder(env::args()).unwrap_or_else(|err| {
        eprint!("{}", err);
        process::exit(1);
    });
}
