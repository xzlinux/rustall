#![allow(dead_code)]
enum Status {
    Rich,
    Poor,
}
enum Work{
    Civilian,
    Soldier,
}
fn test1(){
    use Status::{Poor,Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;
    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }
    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

enum Number {
    Zero,
    One,
    Two,
}
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
fn test2(){
    println!("zero is {}",Number::Zero as i32);
    println!("one is {}",Number::One as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}",Color::Blue as i32);
}
use List::*;
enum List {
    Cons(u32,Box<List>),
    Nil,
}
impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self,elem: u32) -> List {
        Cons(elem,Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Cons(_,ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head,ref tail) => {
                format!("{},{}",head,tail.stringify())
            },
            Nil => {
                format!("Nil")
            }
        }
    }
}
fn test3(){
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);
    println!("{}",list.stringify());

    println!("linked list has length: {}",list.len());

}
static mut LANGUAGE :&'static str="Rust";
const THRESHOLD: i32 = 10;
fn is_big(n: i32) -> bool {
    n > THRESHOLD
}
fn test4(){
    let n = 16;
    unsafe{
        println!("This is {}",LANGUAGE);
        LANGUAGE="c++";
        println!("This is {}",LANGUAGE);
    }
    
    println!("The threshold is {}",THRESHOLD);
    println!("{} is {}",n,if is_big(n){"big"} else {"small"});
    
}
fn test5(){
    let long_lived_binding = 1;
    println!("out {}",long_lived_binding);
    {
        println!("in {}",long_lived_binding);
        let long_lived_binding=2;
        println!("{}",long_lived_binding);
        let long_lived_binding_in=100;
    }
    
    let long_lived_binding = 3;
    println!("{}",long_lived_binding);
}
fn test6(){
    let a_binding;
    {
        let x = 2;
        a_binding= x * x;
    }
    println!("a binding: {}", a_binding);
    let another_binding=30;
    println!("another binding: {}",another_binding);

}
fn test7(){
    let x = 1u8;
    let y = 1u64;
    let z = 3f32;
    let i = 1.0;
    println!("size of `i` in bytes: {}",std::mem::size_of_val(&i));
    println!("size of `x` in bytes: {}",std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}",std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}",std::mem::size_of_val(&z));
}
fn test8(){
    let elem = 5u8;
    let mut vec = Vec::new();

    vec.push('a');
    vec.push('b');
    vec.push(2 as char);

    println!("{:?}",vec);

}
fn test9(){
    type Nano_Second = u64;
    type Inch = u64;
  
    type u64_t=u64;
    let nanoseconds: Nano_Second = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    println!("{} nanoseconds + {} inches = {} unit?",
    nanoseconds,
    inches,
    nanoseconds + inches);
}
fn test10(){
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{:p}",&my_str);
    println!("{:p}",&my_string);
}
use std::convert::From;
#[derive(Debug)]
struct Numberx {
    value: i32,
}
impl From<i32> for Numberx {
    fn from(item: i32) -> Self {
        Numberx{value: item}
    }
}
fn test11(){
    let int = 5;
    let num: Numberx = int.into();
    println!("My Number is {:?}",num);
}
use std::convert::TryFrom;
use std::convert::TryInto;
#[derive(Debug,PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber{
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error>{
        if value % 2 ==0{
            Ok(EvenNumber(value))
        }else {
            Err(())
        }
    }
}
fn test12(){
    assert_eq!(EvenNumber::try_from(8),Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5),Err(()));

    let result: Result<EvenNumber,()>=8i32.try_into();
    assert_eq!(result,Ok(EvenNumber(8)));
    let result: Result<EvenNumber,()>=5i32.try_into();
    assert_eq!(result,Err(()));


}

use std::string::ToString;
struct Circle {
    radius: i32 
}
impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}",self.radius)
    }

}
fn test13(){
    let circlue=Circle{radius: 6};
    println!("{}",circlue.to_string());
}

fn test14(){
    let parsed: i64 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i64>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}",sum);


}
fn main() {
    test14();
}
