#[macro_export(local_inner_macros)]
macro_rules! log_on_dev(
    ($($arg:tt)*) => {
        #[cfg(feature = "verbose")]
        ::std::println!($($arg)*);
    }
);
