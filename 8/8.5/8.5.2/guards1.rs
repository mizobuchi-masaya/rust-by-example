#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    //let temperature = Temperature::Celsius(35);
    //let temperature = Temperature::Celsius(30);
    //let temperature = Temperature::Fahrenheit(90);
    let temperature = Temperature::Fahrenheit(86);
    // ^ TODO try different values for 'temperature'

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The 'if condition' part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is equals to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{} F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t)  => println!("{}f is equal to or below 86 Fahrenheit", t),
    }
}
