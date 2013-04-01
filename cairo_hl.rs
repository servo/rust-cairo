// High-level bindings to Cairo.

use core::libc::*;
use cairo::{cairo_format_t, cairo_surface_t, cairo_t};
use cairo::bindgen::{cairo_create, cairo_fill, cairo_image_surface_create};
use cairo::bindgen::{cairo_image_surface_get_data, cairo_image_surface_get_format};
use cairo::bindgen::{cairo_image_surface_get_height, cairo_image_surface_get_stride};
use cairo::bindgen::{cairo_image_surface_get_width, cairo_rectangle, cairo_set_line_width};
use cairo::bindgen::{cairo_set_source_rgb, cairo_stroke, cairo_surface_destroy};
use cairo::bindgen::{cairo_surface_reference};
use core::cast::transmute;

// FIXME: We should have a hierarchy of surfaces, but this needs to wait on case classes.
pub struct ImageSurface {
    cairo_surface: *cairo_surface_t,
}

impl Drop for ImageSurface {
    fn finalize(&self) {
        unsafe {
            cairo_surface_destroy(self.cairo_surface);
        }
    }
}


pub impl ImageSurface {
    fn width(&self)  -> c_int {
        unsafe {
            cairo_image_surface_get_width(self.cairo_surface)
        }
    }
    fn height(&self) -> c_int {
        unsafe {
            cairo_image_surface_get_height(self.cairo_surface)
        }
    }
    fn stride(&self) -> c_int {
        unsafe {
            cairo_image_surface_get_stride(self.cairo_surface)
        }
    }
    fn format(&self) -> c_int    {
        unsafe {
            cairo_image_surface_get_format(self.cairo_surface)
        }
    }

    fn data(&self) -> &'self [u8] {
        unsafe {
            let buffer = cairo_image_surface_get_data(self.cairo_surface);
            let len = (self.stride() * self.height()) as uint;
            return vec::raw::buf_as_slice(buffer, len, |x| transmute(x));
        }
    }
}

// Should be private.
fn image_surface_from_cairo_surface(cairo_surface: *cairo_surface_t) -> ImageSurface {
    assert!(!cairo_surface.is_null());
    ImageSurface { cairo_surface: cairo_surface }
}

pub fn ImageSurface(format: cairo_format_t, width: c_int, height: c_int) -> ImageSurface {
    unsafe {
        let cairo_surface = cairo_image_surface_create(format, width, height);
        if cairo_surface.is_null() {
            fail!(~"couldn't create Cairo image surface");
        }
        return image_surface_from_cairo_surface(cairo_surface);
    }
}

impl ImageSurface {
    /*fn write_to_png_stream(buffer: &BytesWriter) -> Result<(),cairo_status_t> unsafe {
        extern fn write_fn(closure: *c_void, data: *c_uchar, len: c_uint)
                        -> cairo_status_t unsafe {
            let writer: *BytesWriter = reinterpret_cast(&closure);
            do buf_as_slice(data, len as uint) |bytes| {
                (*writer).write(bytes);
            }
            return CAIRO_STATUS_SUCCESS;
        }

        let buffer_ptr = reinterpret_cast(&buffer);
        let status = cairo_surface_write_to_png_stream(self.cairo_surface, write_fn, buffer_ptr);
        if status != CAIRO_STATUS_SUCCESS {
            return Err(status);
        }

        return Ok(());
    }*/

    fn clone(&self) -> ImageSurface {
        unsafe {
            cairo_surface_reference(self.cairo_surface);
            return image_surface_from_cairo_surface(self.cairo_surface);
        }
    }
}

struct Context {
    cairo_context: *cairo_t,
}

impl Context {
    fn set_line_width(&self, width: c_double) {
        unsafe {
            cairo_set_line_width(self.cairo_context, width);
        }
    }

    fn set_source_rgb(&self, r: c_double, g: c_double, b: c_double) {
        unsafe {
            cairo_set_source_rgb(self.cairo_context, r, g, b);
        }
    }

    fn rectangle(&self, x: c_double, y: c_double, width: c_double, height: c_double) {
        unsafe {
            cairo_rectangle(self.cairo_context, x, y, width, height);
        }
    }

    fn stroke(&self) {
        unsafe {
            cairo_stroke(self.cairo_context);
        }
    }

    fn fill(&self) {
        unsafe {
            cairo_fill(self.cairo_context);
        }
    }
}


fn make_context(surface: &ImageSurface) -> Context {
    unsafe {
        Context {
            cairo_context: cairo_create(surface.cairo_surface)
        }
    }
}
