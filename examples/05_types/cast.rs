#![allow(overflowing_literals)]

/**
 * 类型转换
 * 
 * 
 * rust 不提供原生类型的隐式类型转换，但是可以使用as 关键字进行显式类型转换
 * 
 * 整形之间的转换大体遵循C语言的惯例 
 */

// 不显示类型转换产生的溢出警告

 fn main() {
    let decimal = 65.4321_f32; 

    // 错误 ， 不提供隐式转换
    // let integer :u8 = decimal;

    // 正确
    let integer = decimal as u8;

    println!("{}", integer);

    let character = integer as char;

    println!("{}", character);

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    /*
    *  当把任何类型转换为无符号类型T 时， 会不断加上或者减去 （std::T::MAX+1）
    *  直到值位于新类型T的范围内
    */

    // 1000 已经在u16 的范围内
    println!("1000 as a u16 is:{}", 1000 as u16);

    println!("1000 as a u8 is: {}", 1000 as u8);

     // -1 + 256 = 255
     println!("  -1 as a u8 is : {}", (-1i8) as u8);

     // 对正数，这就和取模一样。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
    // 如果 MSB 是 1，则该值为负” 是一样的。

    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 重复之前的例子
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);

 }