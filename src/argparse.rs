use std::process;
//ERRORS
const NO_ARGS: &str = "Not enough Arguments!";
const TOO_MANY_ARGS: &str = "Too many Arguments!";
const HELP_MSG: &str = "Usage: minigrep [SEARCH_STRING] [FILE]
    If no file is given, stdin will be read instead!
    Examples:
        minigrep \"hello world\" hello_world.txt
        ^- This will output every line in hello_world.txt with that contains \"hello world\"
        
        echo -e \"test\\nhello world hello\" | minigrep \"hello world\"
        ^- This will output the second line \"hello world hello\", as it matches the search query";
pub struct FileArgs {
    pub search_query: String,
    pub file: String,
}
pub enum Conf {
    FromFile(FileArgs),
    FromStdin(String),
}
impl Conf {
    pub fn new<T: Iterator<Item = String>>(mut args: T) -> Conf {
        args.next();
        let search_query = match args.next() {
            Some(s) => s,
            None => {
                eprintln!("{}\n{}", NO_ARGS, HELP_MSG);
                process::exit(1);
            }
        };
        match args.next() {
            Some(s) if args.next().is_none() => Conf::FromFile(FileArgs {
                search_query,
                file: s,
            }),
            None => Conf::FromStdin(search_query),
            Some(_) => {
                eprintln!("{}\n{}", TOO_MANY_ARGS, HELP_MSG);
                process::exit(1);
            }
        }
    }
}
pub fn printhelp() {
    eprintln!("{}", HELP_MSG)
}
