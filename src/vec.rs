//naive vec macro implementation
#[macro_export]
macro_rules! vec_with_semicolon {
    ( $( $($entry:expr),* );* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                $(
                    temp_vec.push($entry);
                )*
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! my_vec {
    ( $($entry:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($entry);
                )*
            temp_vec
        }
    };
}