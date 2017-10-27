use std::env;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let pixman_subdir = "pixman-0.34.0";
    let cairo_subdir = "cairo-1.15.8";
    let disabled_cairo_features = r#"
        xlib xcb qt quartz win32 skia os2 beos drm gallium gl directfb gobject fc ft
    "#;

    let src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let pixman_src_dir = src_dir.join(pixman_subdir);
    let pixman_out_dir = out_dir.join("pixman");
    let pixman_libs_dir = pixman_out_dir.join("pixman").join(".libs");
    let cairo_src_dir = src_dir.join(cairo_subdir);
    let cairo_out_dir = out_dir.join("cairo");
    let cairo_libs_dir = cairo_out_dir.join("src").join(".libs");

    create_dir_if_not_exists(&pixman_out_dir);
    create_dir_if_not_exists(&cairo_out_dir);

    run(Command::new(pixman_src_dir.join("configure"))
        .arg("--disable-gtk")
        .current_dir(&pixman_out_dir)
    );
    run(Command::new("make")
        .env("MAKEFLAGS", env::var_os("CARGO_MAKEFLAGS").unwrap_or_default())
        .current_dir(&pixman_out_dir)
    );

    run(Command::new(cairo_src_dir.join("configure"))
        .args(disabled_cairo_features.split_whitespace().map(|f| format!("--disable-{}", f)))
        .current_dir(&cairo_out_dir)
    );
    run(Command::new("make")
        .env("MAKEFLAGS", env::var_os("CARGO_MAKEFLAGS").unwrap_or_default())
        .current_dir(&cairo_out_dir)
    );

    println!("cargo:rerun-if-changed={}", cairo_src_dir.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", pixman_src_dir.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", cairo_libs_dir.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", pixman_libs_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=cairo");
    println!("cargo:rustc-link-lib=static=pixman-1");
}

fn run(command: &mut Command) {
    println!("Running {:#?}", command);
    assert!(command.status().unwrap().success());
}

fn create_dir_if_not_exists(path: &PathBuf) {
    if !path.exists() {
        create_dir(&path).unwrap()
    }
}
