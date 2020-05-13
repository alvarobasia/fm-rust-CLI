use std::env;
use create_file;
use exceptions::Error;

#[derive(Debug)]
pub enum Command{
    Create(String)
} 


pub fn builder<'a>(mut args: env::Args) -> Result<Command, Error<'a>> {
    args.next();

    match args.next() {
        Some(arg) => if arg == "create" {
            create_file::create_file(args.next())?;
            Ok(Command::Create(arg))
        }else{
           Err(Error::new("INVALID COMMAND. "))
        },
        None => Err(Error::new("FEW ARGUMENTS IN THE COMMAND. "))
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
