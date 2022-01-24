#[macro_export]
macro_rules! mul {
    () => {
        1
    };
    ($v:expr) =>{
        $v
    };
    ($($vs:expr),+){
        $($vs)**
    }
}