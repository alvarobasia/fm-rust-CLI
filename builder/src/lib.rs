use std::env;
use create_file;
use exceptions::Error;

#[derive(Debug, PartialEq)]
pub enum Command{
    Create(String)
} 

pub fn builder<'a>(args: Vec<String>) -> Result<Command, Error<'a>> {


    match args.get(1){
        Some(arg) => if arg == "create" {
            create_file::create_file(args.get(2))?;
            Ok(Command::Create(arg.to_string()))
        }else{
           Err(Error::new("INVALID COMMAND. "))
        },
        None => Err(Error::new("FEW ARGUMENTS IN THE COMMAND. "))
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_works_corretly() {
        let args = vec!["test".to_string(), "create".to_string(), "file".to_string()];
        assert_eq!(builder(args).unwrap(), Command::Create("create".to_string()));
    }

    #[test]
    #[should_panic]
    fn builder_panic_with_few_args() {
        let args = vec!["test".to_string(), "create".to_string()];
        builder(args).unwrap();
    }

    #[test]
    #[should_panic]
    fn builder_panic_with_incorrect_args() {
        let args = vec!["test".to_string(), "creat".to_string()];
        builder(args).unwrap();
    }
}
