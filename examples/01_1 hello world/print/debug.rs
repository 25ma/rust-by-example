// #[derive(Debug)]
// struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);

// fn main() {
//     println!("Now {:?} will print!", Structure(3));
//     println!("Now {:?} will print!", Deep(Structure(7)));
// }

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};

    println!("{:#?}", peter);
}