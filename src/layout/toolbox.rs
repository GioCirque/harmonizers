use super::{toolbox_dropdown::DROPDOWN_ELEMENT_COUNT, toolbox_panel::get_toolbox_panel_region, *};
use libremarkable::{appctx::ApplicationContext, framebuffer::common::mxcfb_rect};

static TOOLBOX_OPEN: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static TOOLBOX_BUTTONS: [[ToolboxItem; 5]; 2] = [
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
static TOOLBOX_DROPDOWNS: [ToolboxItem; 5] = [
    ToolboxItem::BrushType,
    ToolboxItem::BrushSize,
    ToolboxItem::BrushColor,
    ToolboxItem::BrushShape,
    ToolboxItem::Layers,
];

/// Inverts the `state` of the toolbox and calls `on_show` or `on_hide` appropriately.
pub fn toggle_toolbox(app: &mut ApplicationContext<'_>) {
    let is_open = !TOOLBOX_OPEN.swap(!TOOLBOX_OPEN.load(Ordering::Relaxed), Ordering::Relaxed);
    if is_open {
        on_show(app);
    } else {
        on_hide(app);
    }
}

/// Returns a `bool` indicating if the toolbox is open or not.
pub fn is_toolbox_open() -> bool {
    TOOLBOX_OPEN.load(Ordering::Relaxed)
}

/// Handle showing the toolbox.
fn on_show(app: &mut ApplicationContext<'_>) {
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

    for set in TOOLBOX_BUTTONS.iter() {
        for item in set.iter() {
            let element = item.create(app.upgrade_ref(), col, row);
            app.add_element(&item.name(), element);
            col += 1;
        }
        col = 0;
        row += 1;
    }

    row = 0;
    for dropdown in TOOLBOX_DROPDOWNS.iter() {
        let name = dropdown.name();
        let elements = dropdown.create_compound(app.upgrade_ref(), row);
        col = 0;
        for element in elements.iter() {
            let name = format!("{}_{}", name, col);
            app.add_element(&name, element.to_owned());
            col += 1;
        }
        row += 1;
    }

    let region = get_toolbox_panel_region();
    clear_region(app, &region, false);
}

/// Handle hiding the toolbox.
fn on_hide(app: &mut ApplicationContext<'_>) {
    // Remove the toolbox and it's items
    ToolboxItem::Logo.remove(app);
    for set in TOOLBOX_BUTTONS.iter().rev() {
        for item in set.iter().rev() {
            item.remove(app);
        }
    }
    for dropdown in TOOLBOX_DROPDOWNS.iter() {
        let name = dropdown.name();
        for i in 0..DROPDOWN_ELEMENT_COUNT {
            let name = format!("{}_{}", name, i);
            app.remove_element(&name);
        }
    }
    ToolboxItem::Panel.remove(app.upgrade_ref());
    let region = get_toolbox_panel_region();
    clear_region(app, &region, true);
}
