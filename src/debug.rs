/// Equivalent to println!, except does nothing if the "debug" feature is disabled.
#[macro_export]
macro_rules! debug {
    ($($args:tt),*) => {{
        #[cfg(feature = "debug")]
        println!($($args),*);
    }}
}
