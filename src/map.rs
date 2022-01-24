#[macro_export]
macro_rules! map {
    ( $( $key:expr => $value:expr);* ) => {
        {
        let mut temp_hash_map = HashMap::new(); //with_size(?)
        $(
            temp_hash_map.insert($key, $value);
        )*
        temp_hash_map
    }
    };
}
