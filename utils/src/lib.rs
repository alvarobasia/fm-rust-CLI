
use std::env;

pub fn parser_args_to_vec(args: env::Args) -> Vec<String> {
    let mut result = Vec::new();
    args.for_each(|argument|{
        result.push(argument);
    });
    result
}