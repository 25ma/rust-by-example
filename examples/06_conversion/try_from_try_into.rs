/**
 * tryFrom and TryInto
 * 
 * 类似于 From 和Into, TryFrom 和TryInto 是类型转换的通用trait. 不同于From / Into的是，
 * TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，已返回值是 Result 型
 */

 use std::convert::TryFrom;
 use std::convert::TryInto;

 #[derive(Debug, PartialEq)]
 struct EvenNumber(i32);

 impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value:i32) ->Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
 }


 fn main (){

    // tryFrom 
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // tryInto
    
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    println!("result:{:?}", result);
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    println!("result:{:?}", result);
    assert_eq!(result, Err(()));


 }