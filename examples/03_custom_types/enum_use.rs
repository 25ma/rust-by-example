
// 使用use 申明的话，就不用写出名称的完整路径了

#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}


fn main() {

    use Status::{Poor, Rich};

    let status = Rich;

    match status {
        Poor => println!("The poor have on money ..."),
        Rich => println!("The rich have lots of money!"),
    }

    use Work::*;
    let work = Civilian;

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers work!"),
    }
}