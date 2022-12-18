use util;
use TODO::run2;

fn main() {
    let lines = util::read_lines(util::input_file_path())
        .expect("Could not parse input file lines")
        .map(|l| l.expect("Could not parse line"));
    run2(lines);
}
