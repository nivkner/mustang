//! Utilities to check against C signatures, when enabled.

macro_rules! libc {
    ($e:expr) => {
        // TODO: Implement actually using libc. Right now this is just a
        // signature check.
        #[allow(unreachable_code)]
        if false {
            #[allow(unused_imports)]
            use crate::use_libc::*;
            // TODO: `dlopen` libc, `dlsym` the function, and call it...
            return $e;
        }
    };
}

#[cfg(feature = "threads")]
macro_rules! libc_type {
    ($name:ident, $libc:ident) => {
        #[cfg(test)]
        static_assertions::const_assert_eq!(
            core::mem::size_of::<$name>(),
            core::mem::size_of::<libc::$libc>()
        );
        #[cfg(test)]
        static_assertions::const_assert_eq!(
            core::mem::align_of::<$name>(),
            core::mem::align_of::<libc::$libc>()
        );
    };
}

pub(crate) fn same_ptr<T, U>(t: *const T) -> *const U {
    assert_eq!(core::mem::size_of::<T>(), core::mem::size_of::<U>());
    assert_eq!(core::mem::align_of::<T>(), core::mem::align_of::<U>());
    t.cast::<U>()
}

pub(crate) fn same_ptr_mut<T, U>(t: *mut T) -> *mut U {
    assert_eq!(core::mem::size_of::<T>(), core::mem::size_of::<U>());
    assert_eq!(core::mem::align_of::<T>(), core::mem::align_of::<U>());
    t.cast::<U>()
}
