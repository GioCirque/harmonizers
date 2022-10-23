use super::ToolboxItem;
use libremarkable::{
    appctx,
    framebuffer::{
        common::{display_temp, dither_mode, waveform_mode, DRAWING_QUANT_BIT},
        FramebufferRefresh, PartialRefreshMode,
    },
    ui_extensions::element::UIElementHandle,
};

static mut TOOLBOX_OPEN: bool = false;
static ELEMENTS: [ToolboxItem; 2] = [ToolboxItem::Panel, ToolboxItem::Touch];

/// Inverts the `state` of the toolbox and calls `on_show` or `on_hide` appropriately.
pub fn toggle_toolbox(app: &mut appctx::ApplicationContext<'_>, element: UIElementHandle) {
    unsafe {
        println!("Toggling toolbox {:?} -> {:?}", TOOLBOX_OPEN, !TOOLBOX_OPEN);
        TOOLBOX_OPEN = !TOOLBOX_OPEN;
        if TOOLBOX_OPEN {
            on_show(app, element);
        } else {
            on_hide(app, element);
        }
    }
}

/// Returns a `bool` indicating if the toolbox is open or not.
pub fn is_toolbox_open() -> bool {
    unsafe {
        return TOOLBOX_OPEN;
    }
}

/// Handle showing the toolbox.
fn on_show(app: &mut appctx::ApplicationContext<'_>, _element: UIElementHandle) {
    // add everything except the kebab
    println!("Showing {:?} toolbox items.", ELEMENTS.len());
    let root = app.upgrade_ref();
    for e in ELEMENTS.iter() {
        root.add_element(&e.name(), e.create(app));
    }
    app.draw_elements();
}

/// Handle hiding the toolbox.
fn on_hide(app: &mut appctx::ApplicationContext<'_>, _element: UIElementHandle) {
    // remove everything except the kebab
    println!("Hiding {:?} toolbox items.", ELEMENTS.len());
    for e in ELEMENTS.iter().rev() {
        let rect = e
            .find(app.upgrade_ref())
            .unwrap()
            .read()
            .last_drawn_rect
            .unwrap();
        println!(
            "Refreshing area {{ left:{}, top:{}, width:{}, height:{} }}.",
            rect.left, rect.top, rect.width, rect.height
        );
        app.remove_element(&e.name());
        app.get_framebuffer_ref().partial_refresh(
            &rect,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_GC16_FAST,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
            DRAWING_QUANT_BIT,
            true,
        );
    }
}
