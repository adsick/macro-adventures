#[macro_export]
macro_rules! sum {
    () => {
        Default::default()
    };
    ($a:expr) => {
        $a
    };

    ($a:expr, $($tail:expr),+ $(,)?) => { //note trailing comma
        $a + sum!($($tail),*)
    };
    // (add $a:expr, $($tail:expr),+) => {
    //     $a + sum!($($tail),*)
    // };
}