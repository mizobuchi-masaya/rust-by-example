fn main() {
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than Zero"),
        _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
    }
}
