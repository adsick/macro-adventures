#[macro_export]
macro_rules! measure {
    ($operation:expr) => {
        {
        let instant = std::time::Instant::now();
        let result = {$operation};
        let millis = instant.elapsed().as_millis();
        println!("operation {} took {}ms", stringify!($operation), millis);
        result
    }
    };
}