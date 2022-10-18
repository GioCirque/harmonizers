use super::Drawable;
use crate::canvas::*;
use libremarkable::input::{InputEvent, MultitouchEvent};
use std::process::Command;

pub struct AppChrome {
    drawn: bool,

    exit_button_hitbox: Option<mxcfb_rect>,
    pub exit_button_pressed: bool,
}

impl AppChrome {
    pub fn new() -> Self {
        Self {
            drawn: false,
            exit_button_hitbox: None,
            exit_button_pressed: false,
        }
    }
}

impl Drawable for AppChrome {
    fn update(&mut self, canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
        if self.exit_button_pressed {
            canvas.clear();
            canvas.update_full();
            Command::new("systemctl")
                .arg("start")
                .arg("xochitl")
                .status()
                .ok();
            std::process::exit(0);
        }
        None
    }

    fn draw(&mut self, canvas: &mut Canvas) {
        if self.drawn {
            return;
        }
        self.drawn = true;

        canvas.clear();
        canvas.draw_text(
            Point2 {
                x: Some(8),
                y: Some(50),
            },
            "Harmonizers",
            50.0,
        );
        canvas.draw_line(
            Point2 {
                x: Some(0),
                y: Some(58),
            },
            Point2 {
                x: Some(DISPLAYWIDTH as i32),
                y: Some(58),
            },
            2,
        );

        self.exit_button_hitbox = Some(canvas.draw_button_round(
            Point2 {
                x: Some((DISPLAYWIDTH - 36) as i32),
                y: Some(38),
            },
            "X",
            25.0,
        ));

        canvas.update_full();
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
