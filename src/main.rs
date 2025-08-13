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
                    ref e if e.kind() == io::ErrorKind::InvalidFilename => {
                        eprintln!("File not found Error");
                        std::process::exit(
                            e.raw_os_error()
                                .expect(&format!("Could not find File \"{}\"", file_args.file)),
                        );
                    }
                    e => {
                        panic!("{}", e.kind())
                    }
                }
            }
            //panic!("{}", e), //eprintln!("File \"{}\" not found!!!", file_args.file)
        }
        argparse::Conf::FromStdin(search_query) => {
            grep::stdin(search_query);
        }
    }
}
