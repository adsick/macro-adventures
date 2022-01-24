use macad::*;

use std::collections::HashMap;
use std::collections::LinkedList;
fn main() {
    // RA signals an error but there is no one!
    // let v = vec![1, 2, 3, 4, 5, 6];

    // let x = 0;

    // let v = my_vec![1, 2, 3, 4, 5, 6];
    // let v = vec_with_semicolon![1, 2, 3, 4, 5, 6; 7, 8, 9];
    
    
    // let m = map!(4 => "a"; 6 => "b");
    // println!("{:?}", m);

    let l = list![1 => 2 => 3];
    println!("{:?}", l);

    // reverse 
    // let a = 1;
    // let b = 2;
    // let c = 3;
    // rev!(a, b, c);

    // fn print(s: &str){
    //     println!("{}", s)
    // }
    // doesn't work
    // rev!(print("world"), print("hello"));
}
