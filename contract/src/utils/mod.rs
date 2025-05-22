use core::fmt::Debug;
use gstd::{ext, format};

pub const ONE_TOKEN: u128 = 1e12 as u128; // 1_000_000_000_000

pub fn panicking<T, E: Debug, F: FnOnce() -> Result<T, E>>(f: F) -> T {
    match f() {
        Ok(v) => v,
        Err(e) => panic(e),
    }
}

pub fn panic(err: impl Debug) -> ! {
    ext::panic(format!("{err:?}"))
}
