/**
 *  for 循环
 * 
 * for 与区间
 * 
 * for in 结构可以遍历一个 Iterator 。创建迭代器的一个最简单的方法是使用区间标记
 * a..b. 这会生成从a 到 b（不含此值），步长为1 的一系列值
 * 
 */


 fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }


    // for 与 迭代器
    
    // for in 结构能以几种方式与Iterator 互动，在迭代器 trait 一节将会谈到，如果没有特别指定，
    // for 循环会对给出的集合应用 into_iter 函数，把它转换成一个迭代器， 这并不是 把集合变成迭代器的唯一方法，其他的方法有 iter 和 iter_mut 函数
    // 这三个函数以不同的方式返回集合中的数据

    // iter - 在每次迭代中借用集合中的一个元素，这样集合本身不会被改变，循环之后仍可以使用

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }


    // iter_mut - 可变的借用集合中的每个元素，从而允许集合被就地修改

    let mut iter_mut_names = vec!["Bob", "Frank", "Ferris"];

    for name in iter_mut_names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is rustacean among us!",
            _ => "Hello",
        }
    }

    println!("iter_mut_names:{:?}", iter_mut_names);

 }