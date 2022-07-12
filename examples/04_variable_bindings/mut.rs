/*
可变变量

rust 变量默认是不可变的， 但是加上mut 修饰语后就可以改变
 */


 fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation:{}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation:{}", mutable_binding);

    // 但是此时如果将不可变变量 去进行+= 操作，等于进行可变变量的操作就会报错
    // cannot assign twice to immutable variable `_immutable_binding` 

    //_immutable_binding += 1;
 }