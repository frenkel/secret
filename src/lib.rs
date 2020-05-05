#[macro_use]
extern crate glib;
extern crate glib_sys;

extern crate libc;
#[macro_use]
extern crate bitflags;

/// No-op.
macro_rules! skip_assert_initialized {
    () => ()
}

pub use auto::*;

mod auto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
