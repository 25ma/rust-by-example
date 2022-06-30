use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f,"[")?;

        for (k, v) in vec.iter().enumerate() {
            if k != 0 {
                write!(f,", ")?;
            }
            write!(f,"{}: {}", k,v)?;
        }
        write!(f,"]")
    }
}

fn main() {
    let v = List(vec![1,2,3,4,5]);
    println!("{}", v);
}