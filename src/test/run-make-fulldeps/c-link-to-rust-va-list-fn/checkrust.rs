// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type = "staticlib"]
#![feature(c_variadic)]
#![feature(libc)]

extern crate libc;

use libc::{c_char, c_double, c_int, c_long, c_longlong};
use std::ffi::VaList;
use std::ffi::{CString, CStr};

macro_rules! continue_if {
    ($cond:expr) => {
        if !($cond) {
            return 0xff;
        }
    }
}

unsafe fn compare_c_str(ptr: *const c_char, val: &str) -> bool {
    let cstr0 = CStr::from_ptr(ptr);
    let cstr1 = CString::new(val).unwrap();
    &*cstr1 == cstr0
}

#[no_mangle]
pub unsafe extern "C" fn check_list_0(mut ap: VaList) -> usize {
    continue_if!(ap.arg::<c_longlong>() == 1);
    continue_if!(ap.arg::<c_int>() == 2);
    continue_if!(ap.arg::<c_longlong>() == 3);
    0
}

#[no_mangle]
pub unsafe extern "C" fn check_list_1(mut ap: VaList) -> usize {
    continue_if!(ap.arg::<c_int>() == -1);
    continue_if!(ap.arg::<c_char>() == 'A' as c_char);
    continue_if!(ap.arg::<c_char>() == '4' as c_char);
    continue_if!(ap.arg::<c_char>() == ';' as c_char);
    continue_if!(ap.arg::<c_int>() == 0x32);
    continue_if!(ap.arg::<c_int>() == 0x10000001);
    continue_if!(compare_c_str(ap.arg::<*const c_char>(), "Valid!"));
    0
}

#[no_mangle]
pub unsafe extern "C" fn check_list_2(mut ap: VaList) -> usize {
    continue_if!(ap.arg::<c_double>().floor() == 3.14f64.floor());
    continue_if!(ap.arg::<c_long>() == 12);
    continue_if!(ap.arg::<c_char>() == 'a' as c_char);
    continue_if!(ap.arg::<c_double>().floor() == 6.18f64.floor());
    continue_if!(compare_c_str(ap.arg::<*const c_char>(), "Hello"));
    continue_if!(ap.arg::<c_int>() == 42);
    continue_if!(compare_c_str(ap.arg::<*const c_char>(), "World"));
    0
}

#[no_mangle]
pub unsafe extern "C" fn check_list_copy_0(mut ap: VaList) -> usize {
    continue_if!(ap.arg::<c_double>().floor() == 6.28f64.floor());
    continue_if!(ap.arg::<c_int>() == 16);
    continue_if!(ap.arg::<c_char>() == 'A' as c_char);
    continue_if!(compare_c_str(ap.arg::<*const c_char>(), "Skip Me!"));
    ap.copy(|mut ap| {
        if compare_c_str(ap.arg::<*const c_char>(), "Correct") {
            0
        } else {
            0xff
        }
    })
}
