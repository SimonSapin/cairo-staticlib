#![no_std]

extern crate libc;
use libc::c_char;

#[link(name = "cairo", kind = "static")]
extern "C" {
    pub fn cairo_version_string() -> *const c_char;
}
