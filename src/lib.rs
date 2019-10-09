//! This library provides a convenient derive macro for the standard library's
//! [`std::error::Error`] trait.
//!
//! [`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
//!
//! <br>
//!
//! # Example
//!
//! ```rust
//! use thiserror::Error;
//!
//! #[derive(Error, Debug)]
//! pub enum DataStoreError {
//!     #[error("data store disconnected")]
//!     Disconnect(#[source] io::Error),
//!     #[error("the data for key `{0}` is not available")]
//!     Redaction(String),
//!     #[error("invalid header (expected {expected:?}, found {found:?})")]
//!     InvalidHeader {
//!         expected: String,
//!         found: String,
//!     },
//!     #[error("unknown data store error")]
//!     Unknown,
//! }
//! ```
//!
//! <br>
//!
//! # Details
//!
//! - Errors may be enums, structs with named fields, tuple structs, or unit
//!   structs.
//!
//! - A `Display` impl is generated for your error if you provide
//!   `#[error("...")]` messages on the struct or each variant of your enum, as
//!   shown above in the example.
//!
//!   The messages support a shorthand for interpolating fields from the error.
//!
//!     - `#[error("{var}")]` ⟶ `write!("{}", self.var)`
//!     - `#[error("{0}")]` ⟶ `write!("{}", self.0)`
//!     - `#[error("{var:?}")]` ⟶ `write!("{:?}", self.var)`
//!     - `#[error("{0:?}")]` ⟶ `write!("{:?}", self.0)`
//!
//!   You may alternatively write out the full format args yourself, using
//!   arbitrary expressions.
//!
//!   When providing your own format args, the shorthand does not kick in so you
//!   need to specify `.var` in the argument list to refer to named fields and
//!   `.0` to refer to tuple fields.
//!
//!   ```rust
//!   pub enum Error {
//!       #[error("invalid rdo_lookahead_frames {} (expected < {})", .0, i32::max_value())]
//!       InvalidLookahead(i32),
//!   }
//!   ```
//!
//! - The Error trait's `source()` method is implemented to return whichever
//!   field has a `#[source]` attribute, if any. This is for identifying the
//!   underlying lower level error that caused your error.
//!
//!   Any error type that implements `std::error::Error` or dereferences to `dyn
//!   std::error::Error` will work as a source.
//!
//!   ```rust
//!   #[derive(Error, Debug)]
//!   pub struct MyError {
//!       msg: String,
//!       #[source]
//!       source: anyhow::Error,
//!   }
//!   ```
//!
//! - The Error trait's `backtrace()` method is implemented to return whichever
//!   field has a type named `Backtrace`, if any.
//!
//!   ```rust
//!   use std::backtrace::Backtrace;
//!
//!   #[derive(Error, Debug)]
//!   pub struct MyError {
//!       msg: String,
//!       backtrace: Backtrace, // automatically detected
//!   }
//!   ```
//!
//! - See also the [`anyhow`] library for a convenient single error type to use
//!   in application code.
//!
//!   [`anyhow`]: https://github.com/dtolnay/anyhow

mod aserror;

pub use thiserror_impl::*;

// Not public API.
#[doc(hidden)]
pub mod private {
    pub use crate::aserror::AsDynError;
}
