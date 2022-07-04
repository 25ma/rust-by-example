
#![allow(dead_code)]

// Zero 的值为啥是0 ？ 因为enum 的隐式辨别值是从0 开始的，如果调整Zero 与One 的位置，则One 的值为0 
enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {

    // enum 可以转成整型
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets  are #{:06x}", Color::Green as i32);

}