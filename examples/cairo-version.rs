extern crate cairo_static_sys;

use std::ffi::CStr;

fn main() {
    let version = unsafe { CStr::from_ptr(cairo_static_sys::cairo_version_string()) };
    println!("{}", version.to_str().unwrap());
}
