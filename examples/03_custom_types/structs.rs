// rust 结构体分为三种

//1 . 元组结构体, 其就是具名的元组

struct Pair(i32, f32);


// 2. 经典的C语言风格结构体

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

// 3. 单元结构体，不带字段 在泛型中很有用

struct Unit;

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_left: Point,
}
fn main() {
    // 简单的方法初始化字段，并创建结构体
    let name = String::from("peter");
    let age = 27;
    let peter = Person {
        name,
        age
    };

    // 以debug 的方式打印结构体
    println!("{:?}", peter);

    // 实例化结构体
    let point: Point = Point{
        x: 10.3, y:0.4,
    };

    // 访问结构体的字段
    println!("point coordinates: ({}, {})", point.x, point.y);


    // 使用结构体更新语法，创建新的point 
    // 这样可以用到之前的 point 字段
    let bottom_right = Point{ x:5.2, ..point};

    // 打印新的point
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用let 来解构结构体

    let Point{x: left_edge, y:top_edge} = point;

    println!("point coordinates: ({}, {})",left_edge, top_edge);

    // 实例化一个单元结构体

    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.2);

    // 访问元组结构体 的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

}