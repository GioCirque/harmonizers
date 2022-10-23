mod canvas;
mod kebab;
mod logo;
mod toolbox;
mod toolbox_items;
mod toolbox_panel;
mod touch;

pub use canvas::event_handlers as canvas_handlers;
pub use canvas::CANVAS_REGION;
pub use toolbox::{is_toolbox_open, toggle_toolbox};
pub use toolbox_items::ToolboxItem;

use super::*;
use libremarkable::appctx;

/// Top-level application elements only
#[derive(Debug, Clone, Copy)]
enum AppElement {
    Canvas,
    Kebab,
}

impl AppElement {
    fn name(self) -> String {
        self.to_string()
    }

    fn find(self, app: &mut appctx::ApplicationContext) -> Option<UIElementHandle> {
        app.get_element_by_name(&AppElement::name(self))
    }

    fn create(self, app: &mut appctx::ApplicationContext) -> UIElementWrapper {
        match self {
            AppElement::Canvas => canvas::create(app),
            AppElement::Kebab => kebab::create(app),
        }
    }
}

impl std::fmt::Display for AppElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn init(app: &mut appctx::ApplicationContext) {
    let root = app.upgrade_ref();
    root.clear(true);
    root.add_element(&AppElement::Kebab.name(), AppElement::Kebab.create(app));
    //app.add_element(AppElement::Canvas.name(), AppElement::Canvas.create(app));
}
