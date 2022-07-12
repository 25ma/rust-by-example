
    // rust 常量有两种  可以在任意作用域使用，包括全局作用域，他们都需要显式的类型申明
    // cosnt : 不可改变的值
    // static : 具有'static 生命周期的，可以是可变的变量

    /*
     * 有个特例就是"string" 字面量，可以不经改动就被赋给一个 `static` 变量， 因为它的类型标记 &'static str 就
     * 包含了所要求的生命周期 'static. 其他的引用类型都必须特地申明，使其拥有'static 生命周期， 这两种引用类型的差异似乎也无关
     * 紧要，因为无论如何，static 变量都的显式的申明
     */

static LANGUAGE : &'static str = "RUST";

const THRESHOLD: i32 = 10;

fn is_big(n:i32) -> bool {
    n > THRESHOLD
}


fn main() {

    let n = 6;

   // 在main函数的主函数中访问常量

   println!("This is {}", LANGUAGE);
   println!("The threshold is {}", THRESHOLD);
   println!("{} is {}", n, if is_big(n) {"big"} else {"small"});

    // 常量不可修改
//    THRESHOLD = 5;

}