/**
 * 嵌套循环
 * 
 * 在处理嵌套循环的时候可以 使用break 和 continue 外层标签， 在这类情形中，循环必须用一些 'lable 
 * 标签来注明，并且标签必须传递给break / continue 语句
 */

fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner : loop {
            println!("Entered the inner loop");
            // break;
            break 'outer;
        }
    }
 }