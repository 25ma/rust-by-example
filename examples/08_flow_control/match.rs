/**
 * match 匹配
 * 
 * rust 通过match 关键字来提供模式匹配， 和 C语言的 switch 用法类似， 第一个匹配分支会被比对，并且多有可能的值都必须覆盖
 *
 */


 fn main() {
    let number = 13;

    println!("Tell me about {}", number);


    match number {
        1 => println!("One"),
        2|3|5|11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
 }