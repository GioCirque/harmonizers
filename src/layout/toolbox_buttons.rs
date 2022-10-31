use super::toolbox_items::ToolboxItem;
use crate::{
    UIConstraintRefresh, UIElement, UIElementHandle, UIElementWrapper, APP_BUTTON_GAP_V,
    APP_BUTTON_SPACE_H, APP_BUTTON_SPACE_V, APP_BUTTON_TOP, DISPLAY_EDGE_RIGHT,
};
use libremarkable::{
    appctx::ApplicationContext,
    framebuffer::{
        common::{display_temp, dither_mode, waveform_mode, DRAWING_QUANT_BIT},
        FramebufferRefresh,
    },
    image,
    input::InputDevice,
};

fn create_button(
    inner: UIElement,
    onclick: Option<fn(&mut ApplicationContext, UIElementHandle)>,
    col: u16,
    row: u16,
) -> UIElementWrapper {
    let x = DISPLAY_EDGE_RIGHT - (APP_BUTTON_SPACE_H * col) - 100;
    let y = APP_BUTTON_TOP + (APP_BUTTON_SPACE_V * row) + APP_BUTTON_GAP_V;
    UIElementWrapper {
        position: cgmath::Point2 {
            x: x.into(),
            y: y.into(),
        },
        refresh: UIConstraintRefresh::RefreshAndWait,
        onclick,
        inner,
        ..Default::default()
    }
}

/**
 * Toolbox Delete Button
 */

pub fn create_delete_button(col: u16, row: u16) -> UIElementWrapper {
    let img = image::load_from_memory(include_bytes!("../../assets/dist/delete.png")).unwrap();
    let inner = UIElement::Image { img };

    create_button(inner, None, col, row)
}

/**
 * Toolbox New Button
 */

pub fn create_new_button(col: u16, row: u16) -> UIElementWrapper {
    let img = image::load_from_memory(include_bytes!("../../assets/dist/new.png")).unwrap();
    let inner = UIElement::Image { img };

    return create_button(inner, None, col, row);
}

/**
 * Toolbox Open Button
 */

pub fn create_open_button(col: u16, row: u16) -> UIElementWrapper {
    let img = image::load_from_memory(include_bytes!("../../assets/dist/open.png")).unwrap();
    let inner = UIElement::Image { img };

    return create_button(inner, None, col, row);
}

/**
 * Toolbox Save Button
 */

pub fn create_save_button(col: u16, row: u16) -> UIElementWrapper {
    let img = image::load_from_memory(include_bytes!("../../assets/dist/save.png")).unwrap();
    let inner = UIElement::Image { img };

    return create_button(inner, None, col, row);
}

/**
 * Toolbox Clear Button
 */

pub fn create_clear_button(col: u16, row: u16) -> UIElementWrapper {
    let img = image::load_from_memory(include_bytes!("../../assets/dist/clear.png")).unwrap();
    let inner = UIElement::Image { img };

    return create_button(inner, None, col, row);
}

/**
 * Toolbox Refresh Button
 */

pub fn create_refresh_button(col: u16, row: u16) -> UIElementWrapper {
    let img = image::load_from_memory(include_bytes!("../../assets/dist/refresh.png")).unwrap();
    let inner = UIElement::Image { img };

    return create_button(inner, Some(on_refresh_tapped), col, row);
}

fn on_refresh_tapped(app: &mut ApplicationContext<'_>, _element: UIElementHandle) {
    app.get_framebuffer_ref().full_refresh(
        waveform_mode::WAVEFORM_MODE_GC16,
        display_temp::TEMP_USE_REMARKABLE_DRAW,
        dither_mode::EPDC_FLAG_USE_DITHERING_DRAWING,
        DRAWING_QUANT_BIT,
        true,
    );
}

/**
 * Toolbox Orientation Cycler
 */

pub fn create_orientation_cycler(col: u16, row: u16) -> UIElementWrapper {
    let img = image::load_from_memory(include_bytes!("../../assets/dist/orientation.png")).unwrap();
    let inner = UIElement::Image { img };

    return create_button(inner, None, col, row);
}

/**
 * Toolbox Touch Event Toggle
 */

pub fn create_touch_toggle(
    app: &mut ApplicationContext<'_>,
    col: u16,
    row: u16,
) -> UIElementWrapper {
    let img = get_touch_image(app.upgrade_ref());
    let inner = UIElement::Image { img };

    return create_button(inner, Some(on_touch_toggle), col, row);
}

fn get_touch_image(app: &mut ApplicationContext<'_>) -> image::DynamicImage {
    match app.is_input_device_active(InputDevice::Multitouch) {
        false => {
            image::load_from_memory(include_bytes!("../../assets/dist/touch-off.png")).unwrap()
        }
        true => image::load_from_memory(include_bytes!("../../assets/dist/touch-on.png")).unwrap(),
    }
}

fn on_touch_toggle(app: &mut ApplicationContext<'_>, _element: UIElementHandle) {
    match app.is_input_device_active(InputDevice::Multitouch) {
        true => app.deactivate_input_device(InputDevice::Multitouch),
        false => app.activate_input_device(InputDevice::Multitouch),
    };
    if let Some(ref elem) = ToolboxItem::Touch.find(app) {
        if let UIElement::Image { ref mut img } = elem.write().inner {
            *img = get_touch_image(app);
        }
    }
    app.draw_element(&ToolboxItem::Touch.name());
}
