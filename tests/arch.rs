use cuneiform_fields::prelude::*;
use std::mem;

#[allow(dead_code)]

pub struct ArchSpecific {
    data: ArchPadding<u8>,
    data_2: u16,
}

fn main() {
    let h = ArchSpecific {
        data: ArchPadding::new(123_u8),
        data_2: 123,
    };

    #[cfg(not(target_os = "windows"))]
    assert_eq!(mem::size_of_val(&h.data), 128);
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    assert_eq!(mem::size_of_val(&h.data), 64);

    // Check other fields which are not optimized.

    #[cfg(not(target_os = "windows"))]
    assert_eq!(mem::size_of_val(&h.data_2), 2);
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    assert_eq!(mem::size_of_val(&h.data_2), 2);
}
