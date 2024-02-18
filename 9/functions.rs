// Unlike C/C++, there's no restriction on the order of function definitions
fn main() {
    // We can use this function here, and defeine it somewhere later
    fizzbuzz_to(100);
}

// Funtion that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the 'return' keyword is not necessaray here
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type '()'
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function retursn '()', the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
