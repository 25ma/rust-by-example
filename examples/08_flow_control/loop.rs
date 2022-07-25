
/**
 * loop 循环
 * 
 * rust 提供了 loop 关键字来表示一个无限循环
 * 
 * 可以使用break 语句在任何时候退出一个循环，还可以使用coutinue 跳过循环体的剩余部分并开始下一轮循环
 */
fn main() {
    let mut count = 0u32;

    println!("Let's count unitl infinity!");

    // 无限循环

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 此时只是跳出当前循环， 也就是if 
            continue;
        }

        println!("{}", count);


        if count == 5 {
            println!("ok, that's enough");

            // 此时是 终止循环
            break;
        }
    }
}