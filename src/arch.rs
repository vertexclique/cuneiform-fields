//!
//! Architecture system uses default Rust supported platforms and architectures.
//!
//! Alignment values are not randomly chosen or incorporated directly.
//! Values are considered and incorporated inside with the mindset of preventing false sharing
//! or creating less warp points in exclusive caching.
//!
//! Please head to the documentation of [Cuneiform](https://docs.rs/cuneiform/0.1.0/src/cuneiform/slabs.rs.html#5) for more info.

use core::fmt;
use core::ops::{Deref, DerefMut};
use cuneiform::cuneiform;

///
/// Applies padding based on the architecture detected by cuneiform.
/// This struct allows us to have padding on only wrapped field.
///
/// Cache line size is determined by a few steps:
/// * It checks the current compiler architecture is already known by cuneiform.
///     * If known it applies that value as alignment for the field.
/// * If it is still not detected then fallback padding will be used.
///
#[cuneiform(hermetic = false)]
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct ArchPadding<T> {
    value: T,
}

unsafe impl<T: Send> Send for ArchPadding<T> {}
unsafe impl<T: Sync> Sync for ArchPadding<T> {}

impl<T> ArchPadding<T> {
    /// Applies padding based on the architecture detected by cuneiform.
    ///
    /// # Examples
    ///
    /// ```
    /// use cuneiform_fields::arch::ArchPadding;
    ///
    /// let padded = ArchPadding::new(1);
    /// ```
    pub fn new(t: T) -> ArchPadding<T> {
        ArchPadding::<T> { value: t }
    }

    /// Returns the inner value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cuneiform_fields::arch::ArchPadding;
    ///
    /// let padded = ArchPadding::new(7);
    /// let value = padded.into_inner();
    /// assert_eq!(value, 7);
    /// ```
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Deref for ArchPadding<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for ArchPadding<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: fmt::Debug> fmt::Debug for ArchPadding<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ArchPadding")
            .field("value", &self.value)
            .finish()
    }
}

impl<T> From<T> for ArchPadding<T> {
    fn from(t: T) -> Self {
        ArchPadding::new(t)
    }
}
