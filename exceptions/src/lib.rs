use std::fmt;
use std::convert;
use std::io;

#[derive(Debug)]
pub struct Error<'a>{
    name : &'a str,
    err: Option<io::Error>
}

impl fmt::Display for Error<'_> {
    fn fmt(&self,  f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.name)
    }
}

impl convert::From<io::Error> for Error<'_> {
    fn from(error: io::Error) -> Self {
        Self {name: "error" , err : Some(error) }
    }
}

impl<'a> Error<'a> {
    pub fn new(name: &'a str) -> Self {
        Error {
            name,
            err: None
        }
    }

    pub fn print_error(&self){
        eprint!("{}", &self)
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
