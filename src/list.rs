#[macro_export]
macro_rules! list {
    ($($entry:expr)=>*) => {
        {
            let mut result = std::collections::LinkedList::new();
            $(
                result.push_back($entry);
            )*
            // println!("flat list: {:?}", result);
            result
        }
    };
    ( $( $($entry:expr)=>* );* ) => {
        {
            let mut result = std::collections::LinkedList::new();
            $(
                let mut sublist = std::collections::LinkedList::new();
                $(
                    sublist.push_back($entry);
                )*
                result.push_back(sublist);
            )*
            // println!("nested list: {:?}", result);
            result
        }
    };
}

/// this one uses slices and should be more efficient
#[macro_export]
macro_rules! list2 {
    ($($entry:expr)=>*) => {
        {
            std::collections::LinkedList::from_iter([$($entry),*])
        }
    };
    ( $( $($entry:expr)=>* );* ) => {
        {
            std::collections::LinkedList::from_iter([
            $(
                std::collections::LinkedList::from_iter([$($entry),*])
            ),*
            ])
        }
    };
}
