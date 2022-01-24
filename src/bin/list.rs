use macad::list;
use macad::list2;
fn main(){

    // simple flat list
    let flat = list![1 => 2 => 3];
    // same but with slices under the hood
    let flat2 = list2![1 => 2 => 3];

    assert_eq!(flat, flat2);

    // nested list (using manual macro invocations)
    let flat_nested = list![list!["a" => "b" => "c"] => list!["d" => "e" => "f"]];
    // nested list (using nested repetitions)
    let nested = list!["a" => "b" => "c"; "d" => "e" => "f"];
    // more efficient implementation
    let nested2 = list2!["a" => "b" => "c"; "d" => "e" => "f"];

    // check that they are equal
    assert_eq!(flat_nested, nested);
    assert_eq!(nested, nested2);
}