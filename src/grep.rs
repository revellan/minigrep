use std::{fs, io};

pub fn stdin(search_query: &str) {
    println!("{}", search_query);
}
pub fn file(search_query: &str, file: &str) -> Result<(), io::Error> {
    let file_contents = fs::read_to_string(file)?;
    for line in file_contents.lines() {
        if line.contains(search_query) {
            println!("{}", line);
        }
    }
    Ok(())
}
