use super::Drawable;
use crate::canvas::*;
use image::RgbImage;
use libremarkable::input::{InputEvent, MultitouchEvent};

pub enum ButtonStyle {
    Rect,
    RectDark,
    Round,
    RoundDark,
    Image,
    DropDown,
    Toggle,
}

pub struct AppButton {
    drawn: bool,
    point: Point2<Option<i32>>,
    weight: f32,
    pressed: bool,
    style: ButtonStyle,
    text: Option<&'static str>,
    image: Option<RgbImage>,
    hitbox: Option<mxcfb_rect>,
    options: Option<Vec<String>>,
    pub show_options: bool,
    pub selected: usize,
    on_press: &'static dyn Fn(&mut AppButton, &mut Canvas) -> Option<Box<dyn Drawable>>,
}

impl AppButton {
    pub fn new(
        text: Option<&'static str>,
        image: Option<RgbImage>,
        point: Point2<Option<i32>>,
        weight: f32,
        style: ButtonStyle,
        options: Option<Vec<String>>,
        on_press: &'static dyn Fn(&mut AppButton, &mut Canvas) -> Option<Box<dyn Drawable>>,
    ) -> Self {
        Self {
            drawn: false,
            pressed: false,
            point,
            weight,
            style,
            text,
            image,
            options,
            on_press,
            show_options: false,
            selected: 0,
            hitbox: None,
        }
    }

    pub fn needs_draw(&self) -> bool {
        !self.drawn
    }

    pub fn reset_drawn(&mut self) {
        self.drawn = false;
    }
}

impl Drawable for AppButton {
    fn update(&mut self, canvas: &mut Canvas) -> Option<Box<dyn Drawable>> {
        if self.pressed {
            self.pressed = false;
            let next = (self.on_press)(self, canvas);
            if next.is_some() {
                return next;
            }
        }
        None
    }

    fn draw(&mut self, canvas: &mut Canvas) {
        if self.drawn {
            return;
        }
        self.drawn = true;

        let draw_gap: u32 = 6;
        match self.style {
            ButtonStyle::Toggle => {
                let options = self.options.as_ref().unwrap();
                let selected = &options[self.selected] as &str;
                self.hitbox =
                    Some(canvas.draw_text(self.point, selected, self.weight, Some(color::BLACK)));
            }
            ButtonStyle::DropDown => {
                let default_hitbox = mxcfb_rect::default();
                let old_hitbox = self.hitbox.unwrap_or(default_hitbox);
                let new_hitbox = canvas.draw_drop_down(
                    self.point,
                    self.options.as_ref().unwrap(),
                    self.selected,
                    self.weight,
                    draw_gap,
                    draw_gap,
                    self.show_options,
                );
                self.hitbox = Some(new_hitbox);

                if old_hitbox.height > new_hitbox.height {
                    canvas.update_partial(&old_hitbox);
                } else {
                    canvas.update_partial(&new_hitbox);
                }
            }
            ButtonStyle::Image => {
                let image = self.image.as_ref().unwrap();
                self.hitbox = Some(canvas.draw_image(image, self.point));
            }
            ButtonStyle::Rect => {
                self.hitbox = Some(canvas.draw_button_rect_outline(
                    self.point,
                    self.text.unwrap_or(""),
                    self.weight,
                    draw_gap,
                    draw_gap,
                ));
            }
            ButtonStyle::RectDark => {
                self.hitbox = Some(canvas.draw_button_rect_filled(
                    self.point,
                    self.text.unwrap_or(""),
                    self.weight,
                    draw_gap,
                    draw_gap,
                ));
            }
            ButtonStyle::Round => {
                self.hitbox = Some(canvas.draw_button_round_outline(
                    self.point,
                    self.text.unwrap_or(""),
                    self.weight,
                ));
            }
            ButtonStyle::RoundDark => {
                self.hitbox = Some(canvas.draw_button_round_filled(
                    self.point,
                    self.text.unwrap_or(""),
                    self.weight,
                ));
            }
        }

        canvas.update_partial(&self.hitbox.unwrap());
    }

    fn on_input(&mut self, event: InputEvent) {
        if let InputEvent::MultitouchEvent { event } = event {
            if let MultitouchEvent::Press { finger, .. } = event {
                let position = finger.pos;
                if self.hitbox.is_some() && Canvas::is_hitting(position, self.hitbox.unwrap()) {
                    self.pressed = true;
                }
            }
        }
    }
}
