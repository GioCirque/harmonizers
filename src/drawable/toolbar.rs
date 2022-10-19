use super::Drawable;
use crate::canvas::*;
use libremarkable::input::{InputEvent, MultitouchEvent};

pub struct AppToolbar {
    drawn: bool,
    line_point_a: Point2<Option<i32>>,
    line_point_b: Point2<Option<i32>>,
    line_width: u32,
    title_point: Point2<Option<i32>>,
    title_text: &'static str,
    title_size: f32,
    partial_refresh_rect: mxcfb_rect,

    exit_button_point: Point2<Option<i32>>,
    exit_button_size: f32,
    exit_button_hitbox: Option<mxcfb_rect>,
    pub exit_button_pressed: bool,
}

const APP_TOOLBAR_HEIGHT: i32 = 58;
const APP_TOOLBAR_WIDTH: i32 = DISPLAYWIDTH as i32;

impl AppToolbar {
    pub fn new() -> Self {
        Self {
            drawn: false,
            line_width: 2,
            line_point_a: Point2 {
                x: Some(0),
                y: Some(APP_TOOLBAR_HEIGHT),
            },
            line_point_b: Point2 {
                x: Some(APP_TOOLBAR_WIDTH),
                y: Some(APP_TOOLBAR_HEIGHT),
            },
            title_point: Point2 {
                x: Some(8),
                y: Some(50),
            },
            title_text: "Harmonizers",
            title_size: 50.0,
            partial_refresh_rect: mxcfb_rect {
                top: 0,
                left: 0,
                width: APP_TOOLBAR_WIDTH as u32,
                height: APP_TOOLBAR_HEIGHT as u32,
            },
            exit_button_point: Point2 {
                x: Some((APP_TOOLBAR_WIDTH - 36) as i32),
                y: Some(38),
            },
            exit_button_size: 25.0,
            exit_button_hitbox: None,
            exit_button_pressed: false,
        }
    }
}

impl Drawable for AppToolbar {
    fn is_hit(&mut self, event: &InputEvent) -> bool {
        if let InputEvent::MultitouchEvent { event } = event {
            if let MultitouchEvent::Press { finger, .. } = event {
                let position = finger.pos;
                return self.exit_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.partial_refresh_rect);
            }
        }
        false
    }

    fn update(&mut self, canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
        if self.exit_button_pressed {
            canvas.clear();
            canvas.update_full();
            std::process::exit(0);
        }
        None
    }

    fn draw(&mut self, canvas: &mut Canvas) {
        if self.drawn {
            return;
        }
        self.drawn = true;

        canvas.draw_text(self.title_point, self.title_text, self.title_size);
        canvas.draw_line(self.line_point_a, self.line_point_b, self.line_width);
        self.exit_button_hitbox =
            Some(canvas.draw_button_round(self.exit_button_point, "X", self.exit_button_size));

        canvas.update_partial(&self.partial_refresh_rect);
    }

    fn on_input(&mut self, event: InputEvent) {
        if let InputEvent::MultitouchEvent { event } = event {
            if let MultitouchEvent::Press { finger, .. } = event {
                let position = finger.pos;
                if self.exit_button_hitbox.is_some()
                    && Canvas::is_hitting(position, self.exit_button_hitbox.unwrap())
                {
                    self.exit_button_pressed = true;
                }
            }
        }
    }
}
