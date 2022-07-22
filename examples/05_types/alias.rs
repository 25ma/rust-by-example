/**
 * 别名
 * 
 * 可以用type 给已有的类型取个新的名字， 类型的名字必须遵循驼峰命名法（类似：CamelCase）
 * 否则编译器将给出警告，原生类型是例外， 比如 usize f32 等等
 */

 // NanoSecond 是 u64的新名字


 type NanoSecond = u64;
  #[allow(dead_code)]
 type Inch = f64;

 // 通过这个属性屏蔽警告
 #[allow(non_camel_case_types)]
 type u64_t = u64;

fn main() {

    let nanseconds : NanoSecond = 5 as u64_t;
    let inches:Inch = 2 as u64_t;

    // 注意类型别名并不能提供额外的类型安全， 因为别名并不是新的类型
    println!("{} nanseconds + {} inches = {} unit?", nanseconds, inches, (nanseconds + inches));
}