use super::*;

/// Disappearing toolbox elements only
#[derive(Debug, Clone, Copy)]
pub enum ToolboxItem {
    Panel,
    Logo,
    Clear,
    Orientation,
    Refresh,
    Touch,
    New,
    Open,
    Save,
    Delete,
    Undo,
    Redo,
}

impl ToolboxItem {
    /// Gets the display name of an element.
    pub fn name(self) -> String {
        self.to_string()
    }

    /// Maybe finds the element, by name, in the `app`.
    pub fn find(self, app: &mut appctx::ApplicationContext) -> Option<UIElementHandle> {
        app.get_element_by_name(&self.name())
    }

    pub fn remove(self, app: &mut appctx::ApplicationContext) -> mxcfb_rect {
        let item = self.find(app);
        if item.is_none() {
            return mxcfb_rect::default();
        }

        let rect = item.unwrap().read().last_drawn_rect.unwrap();
        app.remove_element(&self.name());

        return rect;
    }

    /// Create the associated `UIElementWrapper` for an element.
    pub fn create(
        self,
        app: &mut appctx::ApplicationContext,
        col: u16,
        row: u16,
    ) -> UIElementWrapper {
        match self {
            // The containing panel rectangle
            ToolboxItem::Panel => toolbox_panel::create(),
            ToolboxItem::Logo => logo::create(app),

            // First row of buttons
            ToolboxItem::Undo => toolbox_buttons::create_undo_button(col, row),
            ToolboxItem::Orientation => toolbox_buttons::create_orientation_cycler(col, row),
            ToolboxItem::Touch => toolbox_buttons::create_touch_toggle(app.upgrade_ref(), col, row),
            ToolboxItem::Refresh => toolbox_buttons::create_refresh_button(col, row),
            ToolboxItem::Clear => toolbox_buttons::create_clear_button(col, row),

            // Second row of buttons
            ToolboxItem::Redo => toolbox_buttons::create_redo_button(col, row),
            ToolboxItem::Save => toolbox_buttons::create_save_button(col, row),
            ToolboxItem::Open => toolbox_buttons::create_open_button(col, row),
            ToolboxItem::New => toolbox_buttons::create_new_button(col, row),
            ToolboxItem::Delete => toolbox_buttons::create_delete_button(col, row),
        }
    }
}

/// Enables `to_string` function for `ToolboxItems`.
impl std::fmt::Display for ToolboxItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
