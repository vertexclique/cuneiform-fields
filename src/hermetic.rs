//!
//! Hermetic enables cuneiform to detect cache sizes from OSes which have API to fetch.
//!
//! Currently, hermetic argument works only Linux kernel 2.6.32 and above.
//! If system is different than supported systems it falls back to slabs.

use core::fmt;
use core::ops::{Deref, DerefMut};
use cuneiform::cuneiform;

///
/// Applies padding hermetically detected by cuneiform.
///
/// If OS exposes an API to detect coherence line size this padding
/// type is using that amount to align the field with padding.
///
#[cuneiform(hermetic = true)]
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct HermeticPadding<T> {
    value: T,
}

unsafe impl<T: Send> Send for HermeticPadding<T> {}
unsafe impl<T: Sync> Sync for HermeticPadding<T> {}

impl<T> HermeticPadding<T> {
    /// Applies padding hermetically detected by cuneiform.
    ///
    /// # Examples
    ///
    /// ```
    /// use cuneiform_fields::hermetic::HermeticPadding;
    ///
    /// let padded = HermeticPadding::new(1);
    /// ```
    pub fn new(t: T) -> HermeticPadding<T> {
        HermeticPadding::<T> { value: t }
    }

    /// Returns the inner value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cuneiform_fields::hermetic::HermeticPadding;
    ///
    /// let padded = HermeticPadding::new(7);
    /// let value = padded.into_inner();
    /// assert_eq!(value, 7);
    /// ```
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Deref for HermeticPadding<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for HermeticPadding<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: fmt::Debug> fmt::Debug for HermeticPadding<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HermeticPadding")
            .field("value", &self.value)
            .finish()
    }
}

impl<T> From<T> for HermeticPadding<T> {
    fn from(t: T) -> Self {
        HermeticPadding::new(t)
    }
}
