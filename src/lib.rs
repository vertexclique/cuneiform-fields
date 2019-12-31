//! # Cuneiform Fields: Field level cache optimizations for Rust (no_std).
//!
//!
//! This crate provides cache line size fitting optimizations to fields in structs.
//! It is no_std and can be used in any no_std environment.
//!
//! This crate aligns fields with `#[repr(align(COHERENCE_LINE_SIZE))]` to decrease the time between prefetch signals for data.
//! `COHERENCE_LINE_SIZE` can be detected or decided based on the architecture by `cuneiform` itself.
//!
//! ```toml
//! [dependencies]
//! cuneiform_fields = "0.1"
//! ```
//!
//! ## Examples
//!
//! #### Hermetic aligned fields
//! Align by hermetic cache line size detection mentioned in [cuneiform readme](https://github.com/vertexclique/cuneiform#----):
//! ```rust
//! use cuneiform_fields::prelude::*;
//!
//! pub struct Hermetic {
//!     data: HermeticPadding<u8>,
//!     data_2: u16,
//! }
//! ```
//! In the example above `data` will be aligned by hermetic alignment but field `data_2` isn't going to be alignment optimized.
//!
//! #### Architecture aligned fields
//! Align by cache line size detected by current Rust compiler architecture.
//! If architecture isn't detected in known architectures it will fall back to default alignment:
//! ```rust
//! use cuneiform_fields::prelude::*;
//!
//! pub struct ArchSpecific {
//!     data: ArchPadding<u8>,
//!     data_2: u16,
//! }
//! ```
//! In the example above `data` will be aligned by architecture alignment but field `data_2` isn't going to be alignment optimized.

#![doc(
    html_logo_url = "https://github.com/vertexclique/cuneiform-fields/raw/master/img/cuneiform-logo.png"
)]
// Force missing implementations
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![no_std]

pub mod arch;
pub mod hermetic;

pub mod prelude {
    //!
    //! Prelude that exports all field wrapping cache alignment types
    pub use crate::arch::*;
    pub use crate::hermetic::*;
}
