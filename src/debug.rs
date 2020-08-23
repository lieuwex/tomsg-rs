/// Equivalent to println!, except does nothing if the "debug" feature is disabled.
macro_rules! debug {
    ($($args:tt),*) => {{
        #[cfg(feature = "debug")]
        println!($($args),*);
    }}
}
