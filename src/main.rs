use minigrep::{argparse, grep};
use std::{env, io};
fn main() {
    let args: Vec<String> = env::args().collect();
    match argparse::Conf::new(&args) {
        argparse::Conf::FromFile(file_args) => {
            let error: Option<io::Error> = match grep::file(file_args.search_query, file_args.file)
            {
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
            grep::stdin(search_query);
        }
    }
}
