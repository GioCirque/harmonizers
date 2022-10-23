use super::*;

/// Disappearing toolbox elements only
#[derive(Debug, Clone, Copy)]
pub enum ToolboxItem {
    Panel,
    Touch,
}

impl ToolboxItem {
    /// Gets the display name of an element.
    pub fn name(self) -> String {
        self.to_string()
    }

    /// Maybe finds the element, by name, in the `app`.
    pub fn find(self, app: &mut appctx::ApplicationContext) -> Option<UIElementHandle> {
        app.get_element_by_name(&ToolboxItem::name(self))
    }

    /// Create the associated `UIElementWrapper` for an element.
    pub fn create(self, app: &mut appctx::ApplicationContext) -> UIElementWrapper {
        match self {
            ToolboxItem::Panel => toolbox_panel::create(app),
            ToolboxItem::Touch => touch::create(app),
        }
    }
}

/// Enables `to_string` function for `ToolboxItems`.
impl std::fmt::Display for ToolboxItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
