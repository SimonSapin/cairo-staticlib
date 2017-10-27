use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let cairo_subdir = "cairo-1.15.8";
    let disabled_features = r#"
        xlib xcb qt quartz win32 skia os2 beos drm gallium gl directfb gobject fc ft
    "#;

    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let cairo_dir = src_dir.join(cairo_subdir);
    let libs_dir = out_dir.join("src").join(".libs");

    run(Command::new(cairo_dir.join("configure"))
        .args(disabled_features.split_whitespace().map(|f| format!("--disable-{}", f)))
        .current_dir(&out_dir)
    );
    run(Command::new("make")
        .env("MAKEFLAGS", env::var_os("CARGO_MAKEFLAGS").unwrap_or_default())
        .current_dir(&out_dir)
    );

    println!("cargo:rerun-if-changed={}", cairo_dir.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", libs_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=cairo");
}

fn run(command: &mut Command) {
    println!("Running {:#?}", command);
    assert!(command.status().unwrap().success());
}
