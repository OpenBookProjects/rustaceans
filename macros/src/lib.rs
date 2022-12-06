#[macro_export]
macro_rules! hashmap {
/*     () => {
        HashMap::new()
    };
 */
    ($($k:expr => $v:expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($k,$v);
            )*
        _map
        }
    };

    ($($k:expr => $v:expr,)*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($k,$v);
            )*
        _map
        }
    };
}
