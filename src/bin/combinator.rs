use macad::comb;
fn main(){
    println!("2 + 2 is {}", comb!((2, 2) > add));
    
    let add_one = |n|n+1;
    let sub_one = |n|n-1;
    let mul_by_two = |n|2*n;

    println!("(5 + 1)*2 - 1 is {}", comb!((5) > add_one > mul_by_two > sub_one));
    
    //"vararg" support
    println!("((13 + 4) + 1)*2 is {}", comb!((13, 4) > add > add_one > mul_by_two));
}

fn add(a: i32, b: i32)->i32{
    a + b
}

fn sub(a: i32, b: i32)->i32{
    a - b
}