#[macro_export]
macro_rules! allow {
    ($metadata:meta, $msg:expr) => {
        #[doc(hidden)]
        #[cfg(not($metadata))]
        compile_error!($msg);
    };
}

#[macro_export]
macro_rules! deny {
    ($metadata:meta, $msg:expr) => {
        #[doc(hidden)]
        #[cfg($metadata)]
        compile_error!($msg);
    };
}