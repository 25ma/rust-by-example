
// 测试链表

use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}


impl List {
    fn new() ->List{
        Nil
    }

    fn prepend(self, elem:u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {

        // 必须对self 进行匹配，因为这个方法的行为取决于self 的取值种类， self 为&List, 而 *self 为List
        // 匹配一个具体的T类型要好过 其引用 &T

        match *self {
            // 不能得到tail 的所有权，因为tail 是借用的；
            // 因此使用一个tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    // 返回列表的字符串表示（该字符串是对分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }, 
            Nil => {
                format!("Nil")
            }
        }
    }  
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(20);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}