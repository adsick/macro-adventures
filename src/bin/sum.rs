use macad::sum;

fn main(){
    assert_eq!(0, sum!());

    assert_eq!(1, sum!(1));

    assert_eq!(4, sum!(2, 2));

    assert_eq!(14, sum!(3, 6, 5,));

    let s = "hello".to_string();

    println!("{}", sum!(s, " from sum macro"));

    // note: can't do smith like sum!(string, &str, &str); 

}