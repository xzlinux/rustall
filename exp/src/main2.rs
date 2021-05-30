fn apply<F: FnOnce()>(f: F){
        f();
}
fn apply_to_3<F: Fn(i32)-> i32>(f: F) -> i32 {
        f(3)
    }
fn apply_2<F>(mut f: F) where 
    F: FnOnce(){
        f();
    }
fn main(){
    use std::mem;
    let greeting = "hello";
    //不可复制变量
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}.",greeting);
        farewell.push_str("!!!");
        println!("The I screamed {}.",farewell);
        println!("Now I can sleep. zzzz");
        mem::drop(farewell);
    };
    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}",apply_to_3(double));

    let mut x =7;
    let print = || {
        x=x+1;
        println!("{}",x+1);
    };
    apply_2(print);

}