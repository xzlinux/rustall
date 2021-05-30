struct Empty;
struct Null;
trait DoubleDrop<T>{
    fn double_drop(self,_: T);
}
impl<T,U:std::fmt::Display> DoubleDrop<T> for U {
    fn double_drop(self,_: T){
        println!("{}",self)
    }
}
fn main(){
    let empty = Empty;
    let null = Null;
   // empty.double_drop(null);
    "abc".double_drop(null);

}