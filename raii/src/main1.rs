fn destroy_box(c: Box<i32>){
    println!("Destroying a box that contains {}",c);
}

fn main() {
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}",x,y);
    let a = Box::new(5i32);
    println!("a contains:{}",a);
    let b = a;
    //println!("{}",a);
    destroy_box(b);
    //println!("b is : {}",b);

    let immutable_box = Box::new(5u32);
    println!("immutable_box contains{}",immutable_box);

   // *immutable_box=100;
    println!("immutable_box {}",immutable_box);

    let mut mutable_box = immutable_box;
    println!("immutable_box contains {}",mutable_box);

    *mutable_box = 4;
    println!("mutable_box now contains {}",mutable_box);

}
