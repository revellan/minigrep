use minigrep::argparse;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let _res: argparse::Conf = argparse::Conf::new(&args);
}
