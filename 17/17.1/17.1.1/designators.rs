macro_rules! create_function {
    // This macro takes an arugment of designator 'ident' and
    // creates a function named '$func_name'.
    // The 'ident' designator is used for variale/fuction names.
    ($func_name:ident) => {
        fn $func_name() {
            // The 'stringify!' macro converts an 'ident' into a string.
            println!("You called {:?}()",
                stringify!($func_name));
        }
    };
}

// Create functions named 'foo' and 'bar' with the above macro.
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type '&xpr' and prints
    // it as a string along with the result.
    // The 'expr' designator is used for expression
    ($expression:expr) => {
        // 'stringify!' will convert the expression *as it is* into a tring.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
