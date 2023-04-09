/// This macro is used to allow a module to be compiled only if a feature is enabled.
/// # Example
/// ```rust,no_run
/// titaniumutils::allow!(feature = "memory", "This module requires the memory feature to be enabled");
/// ```
#[macro_export]
macro_rules! allow {
    ($metadata:meta, $msg:expr) => {
        #[doc(hidden)]
        #[cfg(not($metadata))]
        compile_error!($msg);
    };
}

/// This macro is used to declare a module with a mandatory feature.
/// # Example
/// ```rust,no_run
/// titaniumutils::declare_module!(
///    ///Memory utilities
///    "memory" > memory <= titaniummemory
/// );
#[macro_export]
macro_rules! declare_module {
    ($(#[$attr:meta])* $feature:literal > $name:ident <= $lib:ident) => {
        $(#[$attr])*
        #[cfg(feature = $feature)]
        pub mod $name {
            pub use $lib::*;
        }
    };
}

/// This macro is used to deny a module to be compiled only if a feature is disabled.
/// # Example
/// ```rust,no_run
/// titaniumutils::deny!(all(feature = "web", feature = "desktop"), "You cannot compile this module with both the web and desktop features enabled");
#[macro_export]
macro_rules! deny {
    ($metadata:meta, $msg:expr) => {
        #[doc(hidden)]
        #[cfg($metadata)]
        compile_error!($msg);
    };
}