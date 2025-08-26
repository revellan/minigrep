#[cfg(test)]
use super::*;
#[test]
fn argparse_file() {
    use argparse::Conf;
    use argparse::FileArgs;
    let args: Vec<String> = ["minigrep", "searchstring", "file.txt"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let conf: Conf = Conf::new(&args);
    let file_args: FileArgs;
    match conf {
        Conf::FromStdin(_) => panic!("Argparse failed to parse Arguments correctly!!!"),
        Conf::FromFile(c) => file_args = c,
    }
    assert_eq!(file_args.file, "file.txt");
    assert_eq!(file_args.search_query, "searchstring");
}
#[test]
fn argparse_stdin() {
    use argparse::Conf;
    let args: Vec<String> = ["minigrep", "searchstring"].iter().map(|s| s.to_string()).collect();
    let conf = Conf::new(&args);
    let search_query: String;
    match conf {
        Conf::FromFile(_) => panic!("Argparse failed to parse Arguments correctly!!!"),
        Conf::FromStdin(s) => search_query = s.to_string(),
    }
    assert_eq!(search_query, "searchstring");
}
