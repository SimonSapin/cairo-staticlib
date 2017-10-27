//! Run as `cargo run --example rectangles > /tmp/image.ppm`

extern crate cairo_static_sys;
use cairo_static_sys::*;

use std::io::Write;

fn main() {
    unsafe {
        let width = 300;
        let height = 200;
        let surface = cairo_image_surface_create(CAIRO_FORMAT_RGB24, width, height);
        let context = cairo_create(surface);

        cairo_set_source_rgb(context, 0., 0., 1.);  // Blue
        cairo_rectangle(context, 20., 20., 200., 100.);
        cairo_fill(context);

        cairo_set_source_rgb(context, 0., 1., 0.);  // Green
        cairo_rectangle(context, 100., 50., 150., 100.);
        cairo_fill(context);

        cairo_surface_flush(surface);
        let mut data = cairo_image_surface_get_data(surface);
        let stride = cairo_image_surface_get_stride(surface);

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        writeln!(stdout, "P6 {} {} 255", width, height).unwrap();
        for _ in 0..height {
            for pixel in std::slice::from_raw_parts(data as *const u32, width as usize) {
                let red = (pixel >> 16) as u8;
                let green = (pixel >> 8) as u8;
                let blue = (pixel >> 0) as u8;
                stdout.write_all(&[red, green, blue]).unwrap();
            }
            data = data.offset(stride as isize);
        }
    }
}
