// 如下属性用于隐藏对未使用代码的警告
#![allow(dead_code)]
// 一个枚举内部可以是一个单元结构体，元组结构体或者普通的结构体
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),

    Click { x: i64, y: i64}
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),

        WebEvent::KeyPress(key) => println!("key: {}", key),
        WebEvent::Paste(s) => println!("paste\"{}\". ", s),

        WebEvent::Click{x, y} => {
            println!("clicked at x=: {}, y={}" , x, y);
        }
    }
}

// 类型别名
#[derive(Debug)]
enum VeryVerboseEnumofThingsToDoWithNumbers{
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumofThingsToDoWithNumbers;



fn main() {

    let perssed = WebEvent::KeyPress('x');

    let _pasted = WebEvent::Paste("my test".to_owned());

    let click = WebEvent::Click{x: 3, y: 5};

    let load = WebEvent::PageLoad;

    let unload = WebEvent::PageUnload;

    inspect(perssed);

    inspect(_pasted);

    inspect(click);

    inspect(load);

    inspect(unload);


    // 类型别名, Operations 就是相当于 `VeryVerboseEnumofThingsToDoWithNumbers`

    let x = Operations::Add;

    println!("x: {:?}", x);

}