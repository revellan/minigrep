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
    pub fn new<T: Iterator<Item = String>>(args: T) -> Conf {
        let mut c: u8 = 0;
        let mut search_query: Option<String> = None;
        let mut file: Option<String> = None;
        for i in args {
            match c {
                0 => (),
                1 => search_query = Some(i),
                2 => file = Some(i),
                _ => {
                    eprintln!("{}\n{}", TOO_MANY_ARGS, HELP_MSG);
                    process::exit(1);
                }
            }
            c += 1;
        }
        if search_query == None {
            eprintln!("{}\n{}", NO_ARGS, HELP_MSG);
            process::exit(1);
        } else if file == None {
            Conf::FromStdin(search_query.expect("'search_query' can't be 'None' at this Point!!!"))
        } else {
            Conf::FromFile(FileArgs {
                search_query: search_query
                    .expect("'search_query' can't be 'None' at this Point!!!"),
                file: file.expect("'file' can't be 'None' at this Point!!!"),
            })
        }
    }
}
pub fn printhelp() {
    eprintln!("{}", HELP_MSG)
}
