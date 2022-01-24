use macad::measure;

use std::thread::sleep;
use std::time::Duration;

fn main(){
    measure!(sleep(Duration::from_millis(200)));

    println!("result: {}", measure!(fact(324*128*559*59*79)));

}

fn fact(mut n: u64) -> u64{
    let mut i = 2;
    while i <= n>>1{
        if n % i == 0{
            n /= i;
            // println!("i: {i}, n: {n}");
            i = 2;
        }
        i+=1;
    }
    n
}