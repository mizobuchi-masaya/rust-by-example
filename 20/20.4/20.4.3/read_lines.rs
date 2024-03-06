use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // file hosts.txt must exist in  the current path.
    if let Ok(lines) = read_lines("./hosts.txt") {
        // Consume the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

// The output is wrapped in a Result to allow mathcing on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
