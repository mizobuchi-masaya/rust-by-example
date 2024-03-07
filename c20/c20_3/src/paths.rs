use std::path::Path;

fn main() {
    // Create a 'Path' from an '&'static str'
    let path = Path::new(".");

    // The 'display' method returns a 'Display' style structure
    let _display = path.display();
    println!("{}", _display);

    // "Join' merges a path with anbyte container using the OS speific
    // separator, and returns a 'Pathbuf'
    let mut new_path = path.join("a").join("b");
    println!("{:?}", new_path.to_str());

    // 'push' extends the 'PathBuf' with a '&Path'
    new_path.push("c");
    new_path.push("myfile.tar.gz");
    println!("{:?}", new_path.to_str());

    // 'set_file_name' updates the file name of the 'PathBuf'
    new_path.set_file_name("package.tgz");
    println!("{:?}", new_path.to_str());

    // Convert the 'PathBuf' into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
