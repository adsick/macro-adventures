#[macro_export]
macro_rules! comb {
    (($($args:expr),*) > $f:ident) =>{
        $f($($args),*)
    };
    (($($args:expr),*) > $f:ident > $($tail:ident)>+ ) => {
    comb!(($f($($args),*)) > $($tail)>+)
    };
}

// мои прошлые попытки (не работает как должно)
#[macro_export]
macro_rules! comb_rev {
    (($($args:expr),*) > $f:ident) =>{
        $f($($args),*)
    };
    // (($($args:expr),*) > $f:ident $($tail:ident)+ ) => {
    //     $f(comb!(($($args),*) > $($tail)+))
    // };

    (($($args:expr),*) > $f:ident $($tail:ident)+) => {
        $f(comb!(($($args),*) > $($tail)+ | $f))
    };

    (($($args:expr),*) > $f:ident $($tail:ident)+ | $($reversed:ident);*) => {
        $f(comb!(($($args),*) > $($tail)+ | $($reversed);*; $f))
    };
    (($($args:expr),*) > $f:ident | $($reversed:ident);*) => {
        $f(comb!(($($args),*) > $($reversed);*; $f))
    };

    (($($args:expr),*) > $f:ident; $($tail:ident);+) => {
        $f(comb!(($($args),*) > $($tail)+))
    }
}