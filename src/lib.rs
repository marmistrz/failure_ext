//! Convenience extension traits for the failure error-handling crate
//!
//! This crate makes it possible to call `.context(...)` on the following types
//! * `std::option::Option`
//! * `futures::Future` (if the `future_ext` feature is enabled)

#[cfg(feature = "future_ext")]
mod future_ext;
mod option_ext;

#[cfg(feature = "future_ext")]
pub use future_ext::FutureExt;
pub use option_ext::OptionExt;
