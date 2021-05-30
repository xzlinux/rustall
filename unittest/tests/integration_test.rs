extern crate unittest;
mod common;
#[test]
fn test_add(){
    common::setup();
    assert_eq!(unittest::add(3,2),5);
}