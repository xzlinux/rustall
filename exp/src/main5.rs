
fn main(){
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    println!("2 in vec1: {}",vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}",vec2.into_iter().any(|x| x ==2));

    let vec3=vec![1,2,3];
    let vec4=vec![4,5,6];

    let mut iter = vec3.iter();
    let mut into_iter=vec4.into_iter();
    println!("Find 2 in vec1: {:?}",iter.find(|&&x| x == 2));
    println!("Find 2 in vec1: {:?}",into_iter.find(|&x| x == 2));


}