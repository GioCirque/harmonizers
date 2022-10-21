use image::RgbImage;
pub use libremarkable::framebuffer::{
    cgmath::Point2, cgmath::Vector2, common::color, common::mxcfb_rect, common::DISPLAYHEIGHT,
    common::DISPLAYWIDTH, core::Framebuffer, FramebufferBase, FramebufferDraw, FramebufferIO,
    FramebufferRefresh,
};
use libremarkable::framebuffer::{
    common::display_temp, common::dither_mode, common::waveform_mode, PartialRefreshMode,
};
use std::{cmp::max, ops::DerefMut};

use crate::drawable::Asset;

pub struct Canvas {
    framebuffer: Box<Framebuffer>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            framebuffer: Box::new(Framebuffer::new()),
        }
    }

    pub fn framebuffer_mut(&mut self) -> &'static mut Framebuffer {
        unsafe { std::mem::transmute::<_, &'static mut Framebuffer>(self.framebuffer.deref_mut()) }
    }

    pub fn clear(&mut self) {
        self.framebuffer_mut().clear();
    }

    pub fn update_full(&mut self) {
        self.framebuffer_mut().full_refresh(
            waveform_mode::WAVEFORM_MODE_GC16,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
            0,
            true,
        );
    }

    pub fn update_partial(&mut self, region: &mxcfb_rect) {
        self.framebuffer_mut().partial_refresh(
            region,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_GLR16,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0, // See documentation on DRAWING_QUANT_BITS in libremarkable/framebuffer/common.rs
            false,
        );
    }

    pub fn update_partial_mono(&mut self, region: &mxcfb_rect) {
        self.framebuffer_mut().partial_refresh(
            region,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_DU,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0, // See documentation on DRAWING_QUANT_BITS in libremarkable/framebuffer/common.rs
            false,
        );
    }

    pub fn draw_image(&mut self, img: &RgbImage, pos: Point2<Option<i32>>) -> mxcfb_rect {
        let pos = centered_point(pos);
        return self.framebuffer_mut().draw_image(img, pos);
    }

    pub fn draw_text(
        &mut self,
        pos: Point2<Option<i32>>,
        text: &str,
        size: f32,
        color: Option<color>,
    ) -> mxcfb_rect {
        let mut pos = pos;
        if pos.x.is_none() || pos.y.is_none() {
            // Do dryrun to get text size
            let rect = self.framebuffer_mut().draw_text(
                Point2 {
                    x: 0.0,
                    y: DISPLAYHEIGHT as f32,
                },
                &text,
                size,
                color::BLACK,
                true,
            );

            if pos.x.is_none() {
                // Center horizontally
                pos.x = Some(DISPLAYWIDTH as i32 / 2 - rect.width as i32 / 2);
            }

            if pos.y.is_none() {
                // Center vertically
                pos.y = Some(DISPLAYHEIGHT as i32 / 2 - rect.height as i32 / 2);
            }
        }
        let pos = Point2 {
            x: pos.x.unwrap() as f32,
            y: pos.y.unwrap() as f32,
        };

        let clr = color.or(Some(color::BLACK)).unwrap();
        self.framebuffer_mut()
            .draw_text(pos, &text, size, clr, false)
    }

    pub fn draw_line(
        &mut self,
        a: Point2<Option<i32>>,
        b: Point2<Option<i32>>,
        width: u32,
    ) -> mxcfb_rect {
        let a = centered_point(a);
        let b = centered_point(b);
        return self.framebuffer_mut().draw_line(a, b, width, color::BLACK);
    }

    pub fn draw_rect(
        &mut self,
        pos: Point2<Option<i32>>,
        size: Vector2<u32>,
        border_px: u32,
    ) -> mxcfb_rect {
        let pos = centered_point(pos);
        self.framebuffer_mut()
            .draw_rect(pos, size, border_px, color::BLACK);
        mxcfb_rect {
            top: pos.y as u32,
            left: pos.x as u32,
            width: size.x,
            height: size.y,
        }
    }

    pub fn draw_button_round_outline(
        &mut self,
        pos: Point2<Option<i32>>,
        text: &str,
        size: f32,
    ) -> mxcfb_rect {
        let rect = self.draw_text(pos, text, size, None);
        let c_pos = Point2 {
            x: (rect.left + (rect.width / 2)) as i32,
            y: (rect.top + (rect.height / 2)) as i32,
        };
        let c_rad = max(rect.height, rect.width);
        self.framebuffer_mut()
            .draw_circle(c_pos, c_rad, color::BLACK);

        rect
    }

    pub fn draw_button_round_filled(
        &mut self,
        pos: Point2<Option<i32>>,
        text: &str,
        size: f32,
    ) -> mxcfb_rect {
        let clr = Some(color::WHITE);
        let rect = self.draw_text(pos, text, size, clr);
        let c_pos = Point2 {
            x: (rect.left + (rect.width / 2)) as i32,
            y: (rect.top + (rect.height / 2)) as i32,
        };
        let c_rad = max(rect.height, rect.width);
        self.framebuffer_mut()
            .fill_circle(c_pos, c_rad, color::BLACK);

        rect
    }

    pub fn draw_button_rect_outline(
        &mut self,
        pos: Point2<Option<i32>>,
        text: &str,
        font_size: f32,
        vgap: u32,
        hgap: u32,
    ) -> mxcfb_rect {
        let text_rect = self.draw_text(pos, text, font_size, None);
        self.draw_rect(
            Point2 {
                x: Some((text_rect.left - hgap) as i32),
                y: Some((text_rect.top - vgap) as i32),
            },
            Vector2 {
                x: hgap + text_rect.width + hgap,
                y: vgap + text_rect.height + vgap,
            },
            5,
        )
    }

    pub fn draw_drop_down(
        &mut self,
        point: Point2<Option<i32>>,
        options: &Vec<String>,
        selected: usize,
        font_size: f32,
        vgap: u32,
        hgap: u32,
        is_open: bool,
    ) -> mxcfb_rect {
        let color = Some(color::BLACK);
        let mut rect = mxcfb_rect::default();
        let mut pos = Point2 {
            x: Some(0),
            y: Some(0),
        };
        let mut box_rect = mxcfb_rect::default();
        let img = Asset::get_icon_down();

        if is_open {
            let mut it_hgap = hgap.clone() as i32;
            for text in options {
                pos = Point2 {
                    x: Some((point.x.unwrap() + (vgap as i32)) as i32),
                    y: Some(
                        ((point.y.unwrap() + pos.y.unwrap() + rect.height as i32) + it_hgap) as i32,
                    ),
                };
                if text == "" || text == "-" {
                    rect = self.draw_line(
                        Point2 {
                            x: pos.x.clone(),
                            y: Some(pos.y.unwrap() - rect.height as i32),
                        },
                        Point2 {
                            x: Some(
                                (pos.x.unwrap() + img.width() as i32)
                                    + hgap as i32
                                    + box_rect.width as i32,
                            ),
                            y: Some(pos.y.unwrap() - rect.height as i32),
                        },
                        1,
                    );
                    rect.width = box_rect.width;
                } else {
                    rect = self.draw_text(pos, text, font_size, color);
                }
                box_rect = box_rect.merge_rect(&rect);
                it_hgap = 0;
            }
        } else {
            let text = &options[selected] as &str;
            pos = Point2 {
                x: Some((point.x.unwrap() + (vgap as i32)) as i32),
                y: Some(
                    ((point.y.unwrap() + pos.y.unwrap() + rect.height as i32) + (hgap as i32))
                        as i32,
                ),
            };
            rect = self.draw_text(pos, text, font_size, color);
            box_rect = box_rect.merge_rect(&rect);
        }

        let hit_rect = self.draw_rect(
            Point2 {
                x: Some((box_rect.left - hgap) as i32),
                y: Some((box_rect.top - vgap) as i32),
            },
            Vector2 {
                x: hgap + box_rect.width + hgap + img.width() + hgap,
                y: vgap + box_rect.height + vgap,
            },
            3,
        );

        let img_pos = Point2 {
            x: Some(((hit_rect.left + hit_rect.width) - (img.width() + hgap)) as i32),
            y: Some((hit_rect.top + (rect.height / 2) + (vgap / 2)) as i32),
        };
        self.draw_image(&img, img_pos);

        hit_rect
    }

    pub fn draw_button_rect_filled(
        &mut self,
        pos: Point2<Option<i32>>,
        text: &str,
        size: f32,
        vgap: u32,
        hgap: u32,
    ) -> mxcfb_rect {
        let clr = Some(color::WHITE);
        let text_rect = self.draw_text(pos, text, size, clr);
        let pos = Point2 {
            x: (text_rect.left - hgap) as i32,
            y: (text_rect.top - vgap) as i32,
        };
        let size = Vector2 {
            x: hgap + text_rect.width + hgap,
            y: vgap + text_rect.height + vgap,
        };

        self.framebuffer_mut().fill_rect(pos, size, color::BLACK);
        mxcfb_rect {
            top: pos.y as u32,
            left: pos.x as u32,
            width: size.x,
            height: size.y,
        }
    }

    pub fn is_hitting(pos: Point2<u16>, hitbox: mxcfb_rect) -> bool {
        (pos.x as u32) >= hitbox.left
            && (pos.x as u32) < (hitbox.left + hitbox.width)
            && (pos.y as u32) >= hitbox.top
            && (pos.y as u32) < (hitbox.top + hitbox.height)
    }
}

fn centered_point(point: Point2<Option<i32>>) -> Point2<i32> {
    let mut point = point;
    if point.x.is_none() || point.y.is_none() {
        if point.x.is_none() {
            // Center horizontally
            point.x = Some(DISPLAYWIDTH as i32 / 2);
        }

        if point.y.is_none() {
            // Center vertically
            point.y = Some(DISPLAYHEIGHT as i32 / 2);
        }
    }
    return Point2 {
        x: point.x.unwrap(),
        y: point.y.unwrap(),
    };
}

/* #[cfg(test)]
mod basic {
    use super::*;
    use core::fmt::Error;

    #[test]
    fn can_create_canvas() {
        let test = || -> Result<(), Error> {
            let mut canvas = Canvas::new();
            Ok(())
        };
        assert_eq!(test().is_err(), true);
    }
} */
