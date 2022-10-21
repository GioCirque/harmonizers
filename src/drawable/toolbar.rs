use super::{AppButton, Asset, ButtonStyle, Drawable};
use crate::canvas::*;
use libremarkable::input::InputEvent;

pub struct AppToolbar {
    drawn: bool,
    last_expand: usize,
    line_point_a: Point2<Option<i32>>,
    line_point_b: Point2<Option<i32>>,
    line_width: u32,
    partial_refresh_rect: mxcfb_rect,
    children: [Box<AppButton>; 5],
}

const APP_TOOLBAR_HEIGHT: i32 = 58;
const APP_TOOLBAR_WIDTH: i32 = DISPLAYWIDTH as i32;

impl AppToolbar {
    pub fn new() -> Self {
        Self {
            drawn: false,
            last_expand: 0,
            line_width: 2,
            line_point_a: Point2 {
                x: Some(0),
                y: Some(APP_TOOLBAR_HEIGHT),
            },
            line_point_b: Point2 {
                x: Some(APP_TOOLBAR_WIDTH),
                y: Some(APP_TOOLBAR_HEIGHT),
            },
            partial_refresh_rect: mxcfb_rect {
                top: 0,
                left: 0,
                width: APP_TOOLBAR_WIDTH as u32,
                height: APP_TOOLBAR_HEIGHT as u32,
            },
            children: [
                title_button(),
                save_button(),
                clear_button(),
                exit_button(),
                brushes_button(),
            ],
        }
    }
}

impl Drawable for AppToolbar {
    fn update(&mut self, canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
        for child in self.children.as_mut() {
            let next = child.update(canvas);
            if next.is_some() {
                return next;
            }
        }
        None
    }

    fn draw(&mut self, canvas: &mut Canvas) {
        let expanded = self.children[0].selected == 0;
        let toggled = self.children[0].selected != self.last_expand;
        let draw_children = self.children.as_ref().iter().any(|c| c.needs_draw());
        let needs_draw = !self.drawn || toggled || draw_children;
        if !needs_draw {
            return;
        }
        self.last_expand = self.children[0].selected;
        self.drawn = true;

        canvas.clear();
        if expanded {
            canvas.draw_line(self.line_point_a, self.line_point_b, self.line_width);
        }

        let mut index = 0;
        for child in self.children.as_mut() {
            if index > 0 && !expanded {
                break;
            } else {
                child.reset_drawn();
            }
            child.draw(canvas);
            index += 1;
        }
        canvas.update_partial(&self.partial_refresh_rect);
    }

    fn on_input(&mut self, event: InputEvent) {
        for child in self.children.as_mut() {
            child.on_input(event.to_owned());
        }
    }
}

fn exit_button() -> Box<AppButton> {
    Box::new(AppButton::new(
        None,
        Some(Asset::get_icon_exit()),
        Point2 {
            x: Some(((DISPLAYWIDTH as i32) - 48) as i32),
            y: Some(16),
        },
        25.0,
        ButtonStyle::Image,
        None,
        &exit_button_pressed,
    ))
}

fn save_button() -> Box<AppButton> {
    Box::new(AppButton::new(
        None,
        Some(Asset::get_icon_save()),
        Point2 {
            x: Some(offset_center(-110)),
            y: Some(16),
        },
        25.0,
        ButtonStyle::Image,
        None,
        &save_button_pressed,
    ))
}

fn clear_button() -> Box<AppButton> {
    Box::new(AppButton::new(
        None,
        Some(Asset::get_icon_clear()),
        Point2 {
            x: Some(offset_center(90)),
            y: Some(16),
        },
        25.0,
        ButtonStyle::Image,
        None,
        &clear_button_pressed,
    ))
}

fn title_button() -> Box<AppButton> {
    Box::new(AppButton::new(
        None,
        None,
        Point2 {
            x: Some(8),
            y: Some(50),
        },
        50.0,
        ButtonStyle::Toggle,
        Some(vec!["Harmonizers".to_string(), "H >".to_string()]),
        &title_button_pressed,
    ))
}

fn brushes_button() -> Box<AppButton> {
    Box::new(AppButton::new(
        None,
        Some(Asset::get_icon_down()),
        Point2 {
            x: Some(offset_center(-58)),
            y: Some(32),
        },
        25.0,
        ButtonStyle::DropDown,
        Some(vec![
            "SKETCHY".to_string(),
            "SHADED".to_string(),
            "CHROME".to_string(),
            "FUR".to_string(),
            "LONGFUR".to_string(),
            "WEB".to_string(),
            "".to_string(),
            "SIMPLE".to_string(),
            "SQUARES".to_string(),
            "RIBBON".to_string(),
            "".to_string(),
            "CIRCLES".to_string(),
            "GRID".to_string(),
        ]),
        &brushes_button_pressed,
    ))
}

fn offset_center(offset: i32) -> i32 {
    let center: i32 = ((DISPLAYWIDTH / 2) as i32) + offset as i32;
    center
}

fn title_button_pressed(button: &mut AppButton, _canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
    println!("Title Tapped!");
    button.selected = match button.selected {
        0 => 1,
        _ => 0,
    };
    button.reset_drawn();
    None
}

fn brushes_button_pressed(
    button: &mut AppButton,
    _canvas: &mut Canvas,
) -> Option<Box<dyn Drawable>> {
    println!("Select Brush!");
    button.show_options = !button.show_options;
    button.reset_drawn();
    None
}

fn save_button_pressed(_button: &mut AppButton, _canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
    println!("Save Surface!");
    None
}

fn clear_button_pressed(
    _button: &mut AppButton,
    _canvas: &mut Canvas,
) -> Option<Box<dyn Drawable>> {
    println!("Clear Surface!");
    None
}

fn exit_button_pressed(_button: &mut AppButton, canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
    println!("Exit Application!");
    canvas.clear();
    canvas.update_full();
    std::process::exit(0);
}
