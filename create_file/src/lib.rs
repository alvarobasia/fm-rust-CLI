use std::fs::File;
use exceptions::Error;
use regex::Regex;

pub fn create_file<'a, 'b>(filename: &Vec<String>) -> Result<File, Error<'b> >{
    let file_as_string;

    if filename.len() == 0 {
        return Err(Error::new("NO FILE NAME PASSED. "));
    }else{
        file_as_string = filename.get(0).unwrap();
    };

    let re = Regex::new(r"^.+\..+$").unwrap();

    let mut _result;

    if re.is_match(&file_as_string[..]){
        if filename.len() == 1 || filename[1] != "--u"{
            _result = File::create(filename.get(0).unwrap());
        }else{
            let split = filename[0].split(".");
            let filename: Vec<&str> = split.collect();
            _result = File::create(filename[0]);
        }
    }else if filename.len() == 2 && filename[1] == "--u"{
        _result = File::create(filename.get(0).unwrap());
    }else {
        let format = format!("{}{}", filename.get(0).unwrap().to_owned() , ".txt".to_owned());
        _result = File::create(format);
    }

    match _result {
        Ok(file) => return Ok(file),
        Err(_) => return Err(Error::new("IT IS NOT POSSIBLE TO CREATE THE FILE. ")) 
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_file_correctly_with_extension() {
        assert_eq!(
            match create_file(&vec!["test.txt".to_string()]){
                Ok(_) => "Ok",
                Err(_) => "Err"
            },
            "Ok"
        )
    }   
    #[test]
    fn create_file_correctly_without_extension() {
        assert_eq!(
            match create_file(&vec!["test".to_string()]){
                Ok(_) => "Ok",
                Err(_) => "Err"
            },
            "Ok"
        )
    } 
    #[test]
    fn create_file_correctly_without_extension_untrack() {
        assert_eq!(
            match create_file(&vec!["test".to_string(), "--u".to_string()]){
                Ok(_) => "Ok",
                Err(_) => "Err"
            },
            "Ok"
        )
    } 
    #[test]
    fn create_file_correctly_with_extension_untrack() {
        assert_eq!(
            match create_file(&vec!["test.txt".to_string(), "--u".to_string()]){
                Ok(_) => "Ok",
                Err(_) => "Err"
            },
            "Ok"
        )
    } 
    #[test]
    #[should_panic]
    fn create_file_panic() {
           create_file(&vec![]).unwrap();
    } 
}
