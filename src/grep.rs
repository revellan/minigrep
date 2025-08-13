use std::{fs, io};

pub fn stdin(search_query: &str) {
    let mut line: String = String::new();
    loop {
        line.clear();
        if io::stdin()
            .read_line(&mut line)
            .expect("Stdin should be readable!")
            == 0
        {
            break;
        }
        if line.contains(search_query) {
            print!("{}", line)
        }
    }
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
