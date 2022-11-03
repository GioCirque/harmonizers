use super::{toolbox_dropdown::create_toolbox_dropdown, UIElement, UIElementWrapper};

/**
 * Toolbox Brush Type Dropdown
 */

pub fn create_brush_type_dropdown(row: u16) -> Vec<UIElementWrapper> {
    create_toolbox_dropdown(UIElement::Unspecified, None, row)
}

/**
 * Toolbox Brush Size Dropdown
 */

pub fn create_brush_size_dropdown(row: u16) -> Vec<UIElementWrapper> {
    create_toolbox_dropdown(UIElement::Unspecified, None, row)
}

/**
 * Toolbox Brush Color Dropdown
 */

pub fn create_brush_color_dropdown(row: u16) -> Vec<UIElementWrapper> {
    create_toolbox_dropdown(UIElement::Unspecified, None, row)
}

/**
 * Toolbox Brush Shape Dropdown
 */

pub fn create_brush_shape_dropdown(row: u16) -> Vec<UIElementWrapper> {
    create_toolbox_dropdown(UIElement::Unspecified, None, row)
}

/**
 * Toolbox Brush Layers Dropdown
 */

pub fn create_layers_dropdown(row: u16) -> Vec<UIElementWrapper> {
    create_toolbox_dropdown(UIElement::Unspecified, None, row)
}
