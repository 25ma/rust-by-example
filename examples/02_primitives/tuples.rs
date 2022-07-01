
use std::fmt;

// 元组充当函数的返回

fn reverse(pair:(i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}


#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.0, self.1);
        write!(f, "\r\n");
        write!(f, "({} {})", self.2, self.3)
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。

    let pair = (1, true);

    println!("pair is {:?}", pair);


    // 将元组作为参数传递进函数，然后函数将元组充当函数的返回
    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));

    // 此时如果没有后面的逗号则当做一个字面量返回
    println!("just an integer: {:?}", (5u32));

    // 元组的值绑定到一个变量
    let tuple = (1, "hello", 4.5, true);

    // 又可以将元组类型的变量解构

    let(a,b,c,d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);


    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);


}