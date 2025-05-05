/// Define a macro to wrap test functions with automatic site name print
#[macro_export]
macro_rules! site_test {
    ($site:expr, $name:ident, $body:block) => {
        #[test]
        fn $name() {
            println!("[{}] {} start!", $site, stringify!($name));
            $body
        }
    };
}
