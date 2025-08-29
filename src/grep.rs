use super::argparse;
use std::{env, fs, io};

fn stdin(search_query: &str) {
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
fn file(search_query: &str, file: &str) -> Result<(), io::Error> {
    let file_contents = fs::read_to_string(file)?;
    for line in file_contents.lines() {
        if line.contains(search_query) {
            println!("{}", line);
        }
    }
    Ok(())
}
pub fn main() {
    match argparse::Conf::new(env::args()) {
        argparse::Conf::FromFile(file_args) => {
            let error: Option<io::Error> = match file(&file_args.search_query, &file_args.file) {
                Ok(_) => None,
                Err(e) => Some(e),
            };
            if let Some(e) = error {
                match e {
                    ref e if e.kind() == io::ErrorKind::NotFound => {
                        eprintln!("Could not find File \"{}\"", file_args.file);
                        std::process::exit(e.raw_os_error().unwrap());
                    }
                    e => {
                        eprintln!("{:?}", e.kind());
                        std::process::exit(e.raw_os_error().unwrap());
                    }
                }
            }
        }
        argparse::Conf::FromStdin(search_query) => {
            stdin(&search_query);
        }
    }
}
