/**
 * ToString 和 FromStr
 * 
 * ToString 
 * 
 * 要把任何类型转换成 String, 只需要实现那个类型的ToString trait。 然而不要直接这么做，您应该实现fmt::Display trait
 * 
 * 它会自动提供 ToString, 并且还可以用来打印类型，就像print! 一节中讨论的那样
 * 
 */

//  use std::string::ToString;

 struct Circle {
    radius: i32,
 }


 impl Circle {
    fn get_radius(&self) -> i32 {
        self.radius
        // format!("Circle of radius:{:?}", self.radius)
    }
    fn to_string(&self) -> String {
        format!("Circle of radius:{:?}", self.radius)
    }
 }

 fn main() {

    // toString
    let circle = Circle{radius: 6};
    println!("{}", circle.get_radius());

    println!("redius: {}", circle.to_string());

    // 解析字符串
    // 我们经常需要将字符串转成数字，完成这项工作的标准手段是用parse 函数，我们得提供要转换到的类型
    // 这可以通过不使用类型推断，或者用 涡轮鱼 语法（turbo fish）实现
    // 只要对目标类型实现了 FromStr trait 就可以用 parse 把字符串转换成目标类型， 标准库中已经给无数种
    // 类型实现了 FromStr. 如果要转换到用户定义的类型， 只要手动实现FromStr就行

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;

    println!("Sum:{:?}", sum);
 }