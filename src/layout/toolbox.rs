use super::{toolbox_panel::get_toolbox_panel_region, *};
use libremarkable::{
    appctx::ApplicationContext, framebuffer::common::mxcfb_rect,
    ui_extensions::element::UIElementHandle,
};

static mut TOOLBOX_OPEN: bool = false;
static TOOLBOX_ITEMS: [[ToolboxItem; 5]; 2] = [
    [
        ToolboxItem::Undo,
        ToolboxItem::Orientation,
        ToolboxItem::Touch,
        ToolboxItem::Refresh,
        ToolboxItem::Clear,
    ],
    [
        ToolboxItem::Redo,
        ToolboxItem::Save,
        ToolboxItem::Open,
        ToolboxItem::New,
        ToolboxItem::Delete,
    ],
];

/// Inverts the `state` of the toolbox and calls `on_show` or `on_hide` appropriately.
pub fn toggle_toolbox(app: &mut ApplicationContext<'_>, element: UIElementHandle) {
    let mut is_open: bool = false;
    unsafe {
        TOOLBOX_OPEN = !TOOLBOX_OPEN;
        is_open = TOOLBOX_OPEN;
    }

    if is_open {
        on_show(app, element);
    } else {
        on_hide(app, element);
    }
}

/// Returns a `bool` indicating if the toolbox is open or not.
pub fn is_toolbox_open() -> bool {
    unsafe {
        return TOOLBOX_OPEN;
    }
}

/// Handle showing the toolbox.
fn on_show(app: &mut ApplicationContext<'_>, _element: UIElementHandle) {
    let mut col: u16 = 0;
    let mut row: u16 = 0;
    let root = app.upgrade_ref();

    // Add the toolbox and it's items - General items don't abide column and row, it's ignored out side of the loop
    root.add_element(
        &ToolboxItem::Panel.name(),
        ToolboxItem::Panel.create(app.upgrade_ref(), 0, 0),
    );
    root.add_element(
        &ToolboxItem::Logo.name(),
        ToolboxItem::Logo.create(app, 0, 0),
    );

    for set in TOOLBOX_ITEMS.iter() {
        for item in set.iter() {
            let element = item.create(app.upgrade_ref(), col, row);
            app.add_element(&item.name(), element);
            col += 1;
        }
        col = 0;
        row += 1;
    }
    let region = get_toolbox_panel_region();
    clear_region(app, &region);
}

/// Handle hiding the toolbox.
fn on_hide(app: &mut ApplicationContext<'_>, _element: UIElementHandle) {
    // Remove the toolbox and it's items
    let mut region: mxcfb_rect = ToolboxItem::Panel.remove(app.upgrade_ref());
    region = region.merge_rect(&ToolboxItem::Logo.remove(app));
    for set in TOOLBOX_ITEMS.iter().rev() {
        for item in set.iter().rev() {
            region = region.merge_rect(&item.remove(app));
        }
    }
    clear_region(app, &region);
}
