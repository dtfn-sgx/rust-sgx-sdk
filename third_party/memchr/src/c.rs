// This module defines safe wrappers around memchr (POSIX) and memrchr (GNU
// extension).

#![allow(dead_code)]

extern crate sgx_libc as libc;

// use self::libc::{c_int, c_void, size_t};
use self::libc::{size_t};

pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    let p = unsafe {
        libc::memchr(
            haystack.as_ptr() as *const u8,
            needle as u8,
            haystack.len() as size_t,
        )
    };
    if p.is_null() {
        None
    } else {
        Some(p as usize - (haystack.as_ptr() as usize))
    }
}

// memrchr is a GNU extension. We know it's available on Linux, so start there.
#[cfg(target_os = "linux")]
pub fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    // GNU's memrchr() will - unlike memchr() - error if haystack is empty.
    if haystack.is_empty() {
        return None;
    }
    let p = unsafe {
        libc::memrchr(
            haystack.as_ptr() as *const u8,
            needle as u8,
            haystack.len() as size_t,
        )
    };
    if p.is_null() {
        None
    } else {
        Some(p as usize - (haystack.as_ptr() as usize))
    }
}
