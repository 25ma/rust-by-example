/**
 * 冻结
 * 
 * 当数据相同的名称不变地址时，它还会冻结， 在不可变绑定超出作用域之前，无法修改已冻结的数据：
 */


 fn main() {
    let mut _mutable_integer = 7i32;
    {
        // 被不可变的 _mutable_integer 遮蔽
        let _mutable_integer = _mutable_integer;

        // 此时再当前作用域通过不可变变量去修改其值，在本作用域被冻结

        //_mutable_integer = 50; //cannot assign twice to immutable variable
    } // _mutable_integer 离开作用域

    // 此时是main 函数下的 可变变量，当然修改是可以正常运行的
    _mutable_integer = 3;
    println!("_mutable_integer:{}",_mutable_integer);
 }