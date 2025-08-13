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
pub struct FileArgs<'a> {
    pub search_query: &'a str,
    pub file: &'a str,
}
pub enum Conf<'a> {
    FromFile(FileArgs<'a>),
    FromStdin(&'a str),
}
impl<'a> Conf<'a> {
    pub fn new(args: &'a Vec<String>) -> Conf<'a> {
        match args.len() {
            0 | 1 => {
                panic!("{}\n{}", NO_ARGS, HELP_MSG)
            }
            2 => Conf::FromStdin(&args[1]),
            3 => Conf::FromFile(FileArgs {
                search_query: &args[1],
                file: &args[2],
            }),
            _ => {
                eprintln!("{}\n{}", TOO_MANY_ARGS, HELP_MSG);
                process::exit(1);
            }
        }
    }
}
