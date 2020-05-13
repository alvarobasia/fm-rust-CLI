use std::fs::File;
use exceptions::Error;

pub fn create_file<'a, 'b>(filename: Option<String>) -> Result<File, Error<'b> >{
    
    if let None = filename {
        return Err(Error::new("NO FILE NAME PASSED. "));
    }


    let mut _result = File::create(filename.unwrap());

    match _result {
        Ok(file) => return Ok(file),
        Err(_) => return Err(Error::new("IT IS NOT POSSIBLE TO CREATE THE FILE. ")) 
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
