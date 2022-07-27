/**
 * while 循环
 * 
 * while 关键字可以用作当型循环（当条件满足时循环）
 * 
 * 当我们用while 循环写一个臭名昭著的FiZzzBuzz 程序
 */

 fn main() {
    // 计数器变量
    let mut n = 1;

    // 当n 小于101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器加1
        n += 1;
    }
 }