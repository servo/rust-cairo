/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// FIXME: Use bindgen

extern mod core_graphics;

use cairo;
use cairo_quartz::core_graphics::font::CGFontRef;

#[nolink]
pub extern mod bindgen {
    fn cairo_quartz_font_face_create_for_cgfont(font: CGFontRef) -> *cairo::cairo_font_face_t;
    // XXX: This is here because otherwise the symbol goes missing from the library after linking,
    // and it's used by azure
    fn cairo_quartz_surface_get_cg_context();
}
