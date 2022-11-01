use super::{toolbox_button::create_toolbox_button, toolbox_items::ToolboxItem};
use crate::{UIElement, UIElementHandle, UIElementWrapper};
use libremarkable::{
    appctx::ApplicationContext,
    framebuffer::{
        common::{display_temp, dither_mode, waveform_mode, DRAWING_QUANT_BIT},
        FramebufferRefresh,
    },
    image,
    input::InputDevice,
};

/**
 * Toolbox Undo Button
 */

pub fn create_undo_button(col: u16, row: u16) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/undo.png")).unwrap(),
        },
        None,
        col,
        row,
    )
}

/**
 * Toolbox Redo Button
 */

pub fn create_redo_button(col: u16, row: u16) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/redo.png")).unwrap(),
        },
        None,
        col,
        row,
    )
}

/**
 * Toolbox Delete Button
 */

pub fn create_delete_button(col: u16, row: u16) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/delete.png")).unwrap(),
        },
        None,
        col,
        row,
    )
}

/**
 * Toolbox New Button
 */

pub fn create_new_button(col: u16, row: u16) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/new.png")).unwrap(),
        },
        None,
        col,
        row,
    )
}

/**
 * Toolbox Open Button
 */

pub fn create_open_button(col: u16, row: u16) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/open.png")).unwrap(),
        },
        None,
        col,
        row,
    )
}

/**
 * Toolbox Save Button
 */

pub fn create_save_button(col: u16, row: u16) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/save.png")).unwrap(),
        },
        None,
        col,
        row,
    )
}

/**
 * Toolbox Clear Button
 */

pub fn create_clear_button(col: u16, row: u16) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/clear.png")).unwrap(),
        },
        None,
        col,
        row,
    )
}

/**
 * Toolbox Refresh Button
 */

pub fn create_refresh_button(col: u16, row: u16) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/refresh.png")).unwrap(),
        },
        Some(on_refresh_tapped),
        col,
        row,
    )
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
    create_toolbox_button(
        UIElement::Image {
            img: image::load_from_memory(include_bytes!("../../assets/dist/orientation.png"))
                .unwrap(),
        },
        None,
        col,
        row,
    )
}

/**
 * Toolbox Touch Event Toggle
 */

pub fn create_touch_toggle(
    app: &mut ApplicationContext<'_>,
    col: u16,
    row: u16,
) -> UIElementWrapper {
    create_toolbox_button(
        UIElement::Image {
            img: get_touch_image(app.upgrade_ref()),
        },
        Some(on_touch_toggle),
        col,
        row,
    )
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
