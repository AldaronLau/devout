//! A simple cross-platform logging library
//!
//! # Getting Started
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.devout]
//! version = "1.0.0"
//! # Optional integration with the log crate
//! features = ["log+0.4"]
//!
//! # Optional integration with the log crate
//! [dependencies.log-0_4]
//! package = "log"
//! version = "0.4"
//! optional = true
//! ```
//!
//! ```
#![doc = include_str!("../examples/example.rs")]
//! ```

#![no_std]
#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg"
)]
#![deny(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

#[cfg(feature = "std")]
extern crate std;

mod log;
mod log_level;
mod logger;
mod macros;
mod target;

#[cfg(feature = "log+0.4")]
pub use self::log::version_0_4::*;
pub use self::{
    log_level::LogLevel,
    logger::{logger, Log},
    target::Target,
};

/*
/// A tag to identify a log.
#[derive(Copy, Clone, Debug)]
pub struct Tag(Option<&'static str>);

impl Tag {
    /// Create a new tag by passing a textual identifier.
    #[inline(always)]
    pub const fn new(ident: &'static str) -> Self {
        Tag(Some(ident))
    }

    /// Hide logs using this tag.
    #[inline(always)]
    pub const fn hide(self) -> Self {
        Tag(None)
    }

    /// Choose whether or not to show this tag based on a boolean value.
    /// `true` is show, and `false` is hide.
    #[inline(always)]
    pub const fn show(self, log: bool) -> Self {
        if log {
            self
        } else {
            self.hide()
        }
    }

    /// Returns true if logs using this tag are shown.
    #[inline(always)]
    pub const fn is_shown(self) -> bool {
        self.0.is_some()
    }

    /// Get tag as optional identifier.
    #[inline(always)]
    const fn as_option(&self) -> Option<&'static str> {
        self.0
    }

    /// Print out a log message with this tag.  Prefer `log!()` instead.
    #[inline(always)]
    pub fn log(&self, args: core::fmt::Arguments<'_>) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("[{target}] {args}"));
    }
}*/
