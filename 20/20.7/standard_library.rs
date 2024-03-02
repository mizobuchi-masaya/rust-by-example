use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The reset of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./arg arg1 arg2
    println!("I got {:?} aruments: {:?}.", args.len() - 1, &args[1..]);
}
