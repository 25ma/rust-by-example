

/**
 *  From 和 Into 
 * 
 * From 和 Into 两个trait 是内部相关联的，实际上这是他们实现的一部分，如果我们能够从类型B 得到
 * 类型A, 那么很容易相信我们也能把类型B转换成类型A
 * 
 * 
 * From trait 允许一种类型定义 怎么根据另一种类型生成自己， 因此它提供了一种类型转换的 简单机制
 * 在标准库中有无数 From 的实现， 规定原生类型及其他常见类型的转换功能
 * 
 * 比如 可以很容易的把str 转换成 String 
 */


// 也可以为我们自己的类型定义转换机制

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item}
    }
}

 fn main() {
    // from 例子
    let num = Number::from(30);
    println!("My number is {:?}", num);   

    // into 例子
    let int = 5;
    let num :Number = int.into();
    println!("My number is {:?}", num);
 }