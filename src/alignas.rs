//!
//! Types that have forced alignment size desired by the user.
//!
//! For the rough CXX equivalent of this code: <https://en.cppreference.com/w/cpp/language/alignas>

use core::fmt;
use core::ops::{Deref, DerefMut};

#[doc(hidden)]
macro_rules! impl_align_as {
    ($size:expr, $si:ident) => {
        ///
        /// Applies predetermined alignment size to an object.
        ///
        /// CXX equivalent is: <https://en.cppreference.com/w/cpp/language/alignas>
        #[repr(align($size))]
        pub struct $si<T> {
            value: T,
        }

        unsafe impl<T: Send> Send for $si<T> {}
        unsafe impl<T: Sync> Sync for $si<T> {}

        impl<T> $si<T> {
            /// Applies alignment with $size bytes to given data.
            ///
            /// # Examples
            ///
            /// ```
            /// use cuneiform_fields::alignas::AlignAs8;
            ///
            /// let padded = AlignAs8::new(1);
            /// ```
            pub fn new(t: T) -> $si<T> {
                $si::<T> { value: t }
            }

            /// Returns the inner value.
            ///
            /// # Examples
            ///
            /// ```
            /// use cuneiform_fields::alignas::AlignAs8;
            ///
            /// let padded = AlignAs8::new(7);
            /// let value = padded.into_inner();
            /// assert_eq!(value, 7);
            /// ```
            pub fn into_inner(self) -> T {
                self.value
            }
        }

        impl<T> Deref for $si<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.value
            }
        }

        impl<T> DerefMut for $si<T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.value
            }
        }

        impl<T: fmt::Debug> fmt::Debug for $si<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("$si").field("value", &self.value).finish()
            }
        }

        impl<T> From<T> for $si<T> {
            fn from(t: T) -> Self {
                $si::new(t)
            }
        }
    };
}

#[doc(hidden)]
macro_rules! generate_align_as {
    ($($n: expr => $si: ident),*) => {
        $(impl_align_as!{$n, $si})*
    };
}

// 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768]
generate_align_as! {
    2  => AlignAs2,
    4  => AlignAs4,
    8  => AlignAs8,
    16  => AlignAs16,
    32  => AlignAs32,
    64  => AlignAs64,
    128  => AlignAs128,
    256  => AlignAs256,
    512  => AlignAs512,
    1024  => AlignAs1024,
    2048  => AlignAs2048,
    4096  => AlignAs4096,
    8192  => AlignAs8192,
    16384  => AlignAs16384,
    32768  => AlignAs32768
}

mod alignas_tests {
    use crate::alignas::AlignAs32768;
    #[allow(dead_code)]
    struct Naber(AlignAs32768<u8>);

    #[test]
    fn test_alignas_32768() {
        assert_eq!(core::mem::align_of::<Naber>(), 32768);
    }
}
