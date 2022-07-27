/**
 * 元组
 * 
 * 元组可以在match 中结构
 * 
 *
 */

 fn main () {
    let triple = (0,-2, 3);


    println!("Tell me about {:?}", triple);

    // match 可以结构一个元组
    match triple {
        (0,y,z) => println!("First is `o`, `y` is {:?}, and `x` is {:?}", y ,z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
 }