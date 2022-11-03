mod canvas;
mod kebab;
mod logo;
mod toolbox;
mod toolbox_button;
mod toolbox_buttons;
mod toolbox_dropdown;
mod toolbox_dropdowns;
mod toolbox_items;
mod toolbox_panel;

pub use canvas::{event_handlers as canvas_handlers, CANVAS_REGION};
pub use cgmath::{Point2, Vector2};
pub use kebab::get_kebab_region;
pub use toolbox::{is_toolbox_open, toggle_toolbox};
pub use toolbox_items::ToolboxItem;

use super::*;
use libremarkable::appctx::ApplicationContext;

/// Top-level application elements only
#[derive(Debug, Clone, Copy)]
enum AppElement {
    /* Canvas, */
    Kebab,
}

impl AppElement {
    fn name(self) -> String {
        self.to_string()
    }

    fn create(self, app: &mut ApplicationContext) -> UIElementWrapper {
        match self {
            /* AppElement::Canvas => canvas::create(app), */
            AppElement::Kebab => kebab::create(app),
        }
    }
}

impl std::fmt::Display for AppElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn init(app: &mut ApplicationContext) {
    let root = app.upgrade_ref();
    root.clear(true);
    root.add_element(&AppElement::Kebab.name(), AppElement::Kebab.create(app));
}

pub fn clear_region(app: &mut ApplicationContext, region: &mxcfb_rect, with_history: bool) {
    let fb = app.get_framebuffer_ref();
    let final_region = mxcfb_rect {
        top: std::cmp::max((region.top as i32) - 5, 0 as i32) as u32,
        left: std::cmp::max((region.left as i32) - 5, 0 as i32) as u32,
        width: std::cmp::max(region.width + 10, 0),
        height: std::cmp::max(region.height + 10, 0),
    };
    fb.fill_rect(
        Point2 {
            x: final_region.left as i32,
            y: final_region.top as i32,
        },
        Vector2 {
            x: final_region.width,
            y: final_region.height,
        },
        color::WHITE,
    );
    fb.partial_refresh(
        &final_region,
        PartialRefreshMode::Async,
        waveform_mode::WAVEFORM_MODE_INIT,
        display_temp::TEMP_USE_REMARKABLE_DRAW,
        dither_mode::EPDC_FLAG_EXP1,
        DRAWING_QUANT_BIT,
        false,
    );
    app.draw_elements();
    if with_history {
        canvas_handlers::draw_from_history(app.upgrade_ref());
    }
}
