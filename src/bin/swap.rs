use macad::swap;
fn main(){
    let cat = "Oscar";
    let dog = "Baloo";
    println!("before swap: ");
    println!("cat: {}", cat);
    println!("dog: {}", dog);
    swap!(cat, dog);
    println!("after swap: ");
    println!("cat: {}", cat);
    println!("dog: {}", dog);
}