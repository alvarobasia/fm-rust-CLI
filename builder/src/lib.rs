use create_file;
use exceptions::Error;

#[derive(Debug, PartialEq)]
pub enum Command{
    Create(String)
} 

pub fn builder<'a>(args: &mut Vec<String>) -> Result<Command, Error<'a>> {

    if let None = args.get(1) {
        return Err(Error::new("FEW ARGUMENTS IN THE COMMAND. "))
    }

    match &args.get(1).unwrap()[..]{
        "create" => {
            create_file::create_file(&args.split_off(2))?;
            Ok(Command::Create("create".to_string()))
        },
         _ => Err(Error::new("INVALID COMMAND. ")),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_works_corretly() {
        let mut args = vec!["test".to_string(), "create".to_string(), "file".to_string()];
        assert_eq!(builder(&mut args).unwrap(), Command::Create("create".to_string()));
    }

    #[test]
    #[should_panic]
    fn builder_panic_with_few_args() {
        let mut args = vec!["test".to_string(), "create".to_string()];
        builder(&mut args).unwrap();
    }

    #[test]
    #[should_panic]
    fn builder_panic_with_incorrect_args() {
        let mut args = vec!["test".to_string(), "creat".to_string()];
        builder(&mut args).unwrap();
    }
}
