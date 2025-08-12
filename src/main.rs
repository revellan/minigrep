use minigrep::argparse;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let res: GrCnf = match argparse::parse(&args) {
        Err(e) => panic!("{}", e),
        Ok(res) => res,
    }
}
