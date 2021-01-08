#[macro_export]
macro_rules! hashmap {
    ( $($x:expr => $y:expr),+ $(,)?) => {
        {
            use ::std::collections::HashMap;

            let mut map = HashMap::new();

            $(
                map.insert($x, $y);
            )*
            
            map
        }
    };
    () => {
        {
            use ::std::collections::HashMap;

            HashMap::new()
        }
    }
}

