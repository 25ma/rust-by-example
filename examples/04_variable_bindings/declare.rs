/**
 * 变量先申明
 * 
 * 可以先申明变量绑定，后面才将他们初始化，但是这种做法很少用，因为这样可能导致使用未初始化的变量                                                                                                                                                                                                                                            
 */

 fn main() {

    // 声明一个变量 
    let a_binging;

    {
        let x = 2;
        // 初始化一个绑定
        a_binging = x*x;
    }

    println!("a binding: {}", a_binging);

    // 打开如下注释会直接报错，所以变量需要声明，然后还要进行初始化，否则会报错
    // consider giving `another_binding` a type

    // let another_binding;
    // println!("another binding: {}", another_binding);

 }