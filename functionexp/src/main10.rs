use std::marker::PhantomData;
#[derive(PartialEq)]
struct PhantomTuple<A,B>(A,PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A,B>{first: A,phantom: PhantomData<B>}
fn main(){
    let _tuple1: PhantomTuple<char,f32> = PhantomTuple('Q',PhantomData);
    let _tuple2: PhantomTuple<char,f64> = PhantomTuple('Q',PhantomData);

    let _struct1: PhantomStruct<char,f32> = PhantomStruct{
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char,f64> = PhantomStruct{
        first: 'Q',
        phantom: PhantomData,
    };
}