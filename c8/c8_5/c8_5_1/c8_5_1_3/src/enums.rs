// 'allow' required to silence warnings because only
// one variant i used.
#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name.
    Red,
    Blue,
    Green,
    // These likewise tie 'u32' tuples to diffrent names; color models.
    RGB(u32, u32, u32),
    HSB(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    //let color = Color::RGB(122, 17, 40);
    //let color = Color::HSB(122, 17, 40);
    //let color = Color::HSL(122, 17, 40);
    //let color = Color::CMY(122, 17, 40);
    let color = Color::CMYK(122, 17, 40, 179);
    // TODO ^ Try diffrent variants for 'color'
 
    println!("What color is it?");
    // An 'enum' can be destructured using a 'match'.
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, gren: {}, and blue: {}!", r, g, b),
        Color::HSB(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }
}
