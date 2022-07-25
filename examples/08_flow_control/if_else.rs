/**
 * if-else 分支判断和其他语言类似，不同的是，Rust 语言中的布尔判断条件不必使用小括号包裹，且每个条件后面都跟着一个代码块。 if-else条件选择
 * 是一个表达式，并且所有分支都必须返回相同的类型
 */

 fn main() {
    let n = -1;
    
    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 { 
            println!(", and is a small number, increase then-fold");
            10 * n
        } else {
            println!(", and is a big number, half the number");

            n ^ 2
        };
    
    println!("{} -> {}", n, big_n);
 }