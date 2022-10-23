use super::toolbox_items::ToolboxItem;
use crate::{
    UIConstraintRefresh, UIElement, UIElementHandle, UIElementWrapper, APP_BUTTON_GAP_V,
    APP_BUTTON_SPACE_H, APP_BUTTON_SPACE_V, APP_BUTTON_TOP, DISPLAY_EDGE_RIGHT,
};
use libremarkable::{appctx, image, input::InputDevice};

pub fn create(app: &mut appctx::ApplicationContext) -> UIElementWrapper {
    let x = DISPLAY_EDGE_RIGHT - APP_BUTTON_SPACE_H - 200;
    let y = APP_BUTTON_SPACE_V + APP_BUTTON_TOP + APP_BUTTON_GAP_V;
    UIElementWrapper {
        position: cgmath::Point2 {
            x: x.into(),
            y: y.into(),
        },
        refresh: UIConstraintRefresh::Refresh,
        onclick: Some(on_touch_toggle),
        inner: UIElement::Image {
            img: get_image(app),
        },
        ..Default::default()
    }
}

fn get_image(app: &mut appctx::ApplicationContext<'_>) -> image::DynamicImage {
    match app.is_input_device_active(InputDevice::Multitouch) {
        true => {
            app.deactivate_input_device(InputDevice::Multitouch);
            image::load_from_memory(include_bytes!("../../assets/dist/touch-off.png")).unwrap()
        }
        false => {
            app.activate_input_device(InputDevice::Multitouch);
            image::load_from_memory(include_bytes!("../../assets/dist/touch-on.png")).unwrap()
        }
    }
}

fn on_touch_toggle(app: &mut appctx::ApplicationContext<'_>, _element: UIElementHandle) {
    let name = &ToolboxItem::Touch.name();
    if let Some(ref elem) = ToolboxItem::Touch.find(app) {
        if let UIElement::Image { ref mut img } = elem.write().inner {
            *img = get_image(app);
        }
    }
    app.draw_element(name);
}
