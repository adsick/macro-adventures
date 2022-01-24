#[macro_export]
macro_rules! rev {
    ($i:tt) => {
        $i
    };
    ($i:tt, $($rest:tt),+) =>{
            rev!($($rest),+);
            $i;
    };
    // ($i:ident $($tail:ident)+) =>{
    //     rev!($($tail)+) $i
    // }
}