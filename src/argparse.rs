use std::io::{self, Error};
struct GrCnf {
    
}
pub fn parse(args: &Vec<String>) -> Result<String, io::Error> {
    let not_enough_args: &'static str = "Not enough arguments!";
    if args.len() == 1 {
        return Err(Error::new(io::ErrorKind::Other, not_enough_args))
    } else {
    }
    Ok(String::from(" j"))
}
