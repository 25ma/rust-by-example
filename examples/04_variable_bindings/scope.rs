/**
 *   作用域和屏蔽
 * 
 *  变量绑定有一定的作用域 ，它被限定只在一个代码块中生存，代码块是一个呗｛｝ 包围
 *  的语句集合，另外也允许变量遮蔽 variable shadowing 
 */

 fn main() {
    let long_lived_binding = 1; // 该变量作用域在main 函数中

    {
        let short_lived_binding = 2; // 该变量作用域在当前代码块中

        println!("inner short :{}", short_lived_binding);

        // 此绑定遮蔽了外面的绑定
        let long_lived_binding = 5_u32;

        println!("inner long :{}", long_lived_binding);
    }

    // 此时会报这个错误：  cannot find value `short_lived_binding` in this scope
    // println!("outer short:{}", short_lived_binding);

    // 前面代码块的 相同变量的生命周期已经结束，故当前变量的值依然是初始值 1；
    println!("outer long: {}", long_lived_binding);

    // 对变量long_lived_binding 进行了遮蔽操作
    let long_lived_binding = 'a';

    println!("outer long:{}", long_lived_binding);
 }