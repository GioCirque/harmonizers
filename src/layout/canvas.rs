use crate::{
    draw_mode::DrawMode, touch_mode::TouchMode, UIConstraintRefresh, UIElement, UIElementWrapper,
    WACOM_HISTORY,
};
use cgmath::Point2;
use libremarkable::{appctx, framebuffer::common::*};

use once_cell::sync::Lazy;
use std::sync::Mutex;

struct CanvasEntry {
    startpt: (Point2<f32>, f32),
    ctrlpt: (Point2<f32>, f32),
    endpt: (Point2<f32>, f32),
    samples: i32,
    v: color,
}

static CANVAS_HISTORY: Lazy<Mutex<Vec<CanvasEntry>>> = Lazy::new(|| Mutex::new(Vec::new()));

// This region will have the following size at rest:
//   raw: 5896 kB
//   zstd: 10 kB
pub const CANVAS_REGION: mxcfb_rect = mxcfb_rect {
    top: 0,
    left: 0,
    height: DISPLAYHEIGHT as u32,
    width: DISPLAYWIDTH as u32,
};

pub fn create(_app: &mut appctx::ApplicationContext) -> UIElementWrapper {
    UIElementWrapper {
        position: CANVAS_REGION.top_left().cast().unwrap() + cgmath::vec2(0, -2),
        refresh: UIConstraintRefresh::RefreshAndWait,
        onclick: None,
        inner: UIElement::Region {
            size: CANVAS_REGION.size().cast().unwrap() + cgmath::vec2(1, 3),
            border_px: 2,
            border_color: color::BLACK,
        },
        ..Default::default()
    }
}

pub mod event_handlers {
    use super::{CanvasEntry, CANVAS_HISTORY, CANVAS_REGION};
    use crate::{
        appctx, display_temp, dither_mode, image,
        image::GenericImage,
        layout::{get_kebab_region, is_toolbox_open},
        waveform_mode, DrawMode, FramebufferDraw, FramebufferRefresh, Lazy, Mutex, Ordering,
        PartialRefreshMode, UIElement, UIElementHandle, DRAWING_QUANT_BIT, G_DRAW_MODE,
        UNPRESS_OBSERVED, WACOM_HISTORY, WACOM_RUBBER_SIDE,
    };
    use cgmath::{EuclideanSpace, Point2, Vector2};
    use libremarkable::{
        end_bench,
        framebuffer::{
            common::{color, mxcfb_rect},
            storage, FramebufferIO,
        },
        start_bench,
    };

    static SAVED_CANVAS: Lazy<Mutex<Option<storage::CompressedCanvasState>>> =
        Lazy::new(|| Mutex::new(None));

    pub fn handle_draw_event(
        app: &mut appctx::ApplicationContext<'_>,
        position: Point2<f32>,
        pressure: u16,
        tilt: Vector2<u16>,
    ) {
        let mut wacom_stack = WACOM_HISTORY.lock().unwrap();

        // This is so that we can click the buttons outside the canvas region
        // normally meant to be touched with a finger using our stylus
        if !CANVAS_REGION.contains_point(&position.cast().unwrap())
            || is_toolbox_open()
            || get_kebab_region().contains_point(&position.cast().unwrap())
        {
            wacom_stack.clear();
            if UNPRESS_OBSERVED.fetch_and(false, Ordering::Relaxed) {
                let region =
                    app.find_active_region(position.y.round() as u16, position.x.round() as u16);
                let element = region.map(|(region, _)| region.element.clone());
                if let Some(element) = element {
                    (region.unwrap().0.handler)(app, element)
                }
            }
            return;
        }

        let (mut col, mut mult) = match G_DRAW_MODE.load(Ordering::Relaxed) {
            DrawMode::Draw(s) => (color::BLACK, s),
            DrawMode::Erase(s) => (color::WHITE, s * 3),
        };
        if WACOM_RUBBER_SIDE.load(Ordering::Relaxed) {
            col = match col {
                color::WHITE => color::BLACK,
                _ => color::WHITE,
            };
            mult = 50; // Rough size of the rubber end
        }

        wacom_stack.push_back((position.cast().unwrap(), pressure as i32));

        while wacom_stack.len() >= 3 {
            let framebuffer = app.get_framebuffer_ref();
            let points = vec![
                wacom_stack.pop_front().unwrap(),
                *wacom_stack.get(0).unwrap(),
                *wacom_stack.get(1).unwrap(),
            ];
            let radii: Vec<f32> = points
                .iter()
                .map(|point| ((mult as f32 * (point.1 as f32) / 2048.) / 2.0))
                .collect();
            // calculate control points
            let start_point = points[2].0.midpoint(points[1].0);
            let ctrl_point = points[1].0;
            let end_point = points[1].0.midpoint(points[0].0);
            // calculate diameters
            let start_width = radii[2] + radii[1];
            let ctrl_width = radii[1] * 2.0;
            let end_width = radii[1] + radii[0];
            let rect = framebuffer.draw_dynamic_bezier(
                (start_point, start_width),
                (ctrl_point, ctrl_width),
                (end_point, end_width),
                10,
                col,
            );

            CANVAS_HISTORY.lock().unwrap().push(CanvasEntry {
                startpt: (start_point, start_width),
                ctrlpt: (ctrl_point, ctrl_width),
                endpt: (end_point, end_width),
                samples: 10,
                v: col,
            });

            framebuffer.partial_refresh(
                &rect,
                PartialRefreshMode::Async,
                waveform_mode::WAVEFORM_MODE_DU,
                display_temp::TEMP_USE_REMARKABLE_DRAW,
                dither_mode::EPDC_FLAG_EXP1,
                DRAWING_QUANT_BIT,
                false,
            );
        }
    }

    pub fn draw_from_history(app: &mut appctx::ApplicationContext<'_>) {
        let framebuffer = app.get_framebuffer_ref();
        for entry in CANVAS_HISTORY.lock().unwrap().iter() {
            let nc = entry.v.as_native();
            let rect = framebuffer.draw_dynamic_bezier(
                entry.startpt,
                entry.ctrlpt,
                entry.endpt,
                entry.samples,
                entry.v,
            );
            framebuffer.partial_refresh(
                &rect,
                PartialRefreshMode::Async,
                waveform_mode::WAVEFORM_MODE_DU,
                display_temp::TEMP_USE_REMARKABLE_DRAW,
                dither_mode::EPDC_FLAG_EXP1,
                DRAWING_QUANT_BIT,
                false,
            );
        }
    }

    pub fn on_save_canvas(app: &mut appctx::ApplicationContext<'_>, _element: UIElementHandle) {
        start_bench!(stopwatch, save_canvas);
        let framebuffer = app.get_framebuffer_ref();
        match framebuffer.dump_region(CANVAS_REGION) {
            Err(err) => println!("Failed to dump buffer: {0}", err),
            Ok(buff) => {
                let mut hist = SAVED_CANVAS.lock().unwrap();
                *hist = Some(storage::CompressedCanvasState::new(
                    buff.as_slice(),
                    CANVAS_REGION.height,
                    CANVAS_REGION.width,
                ));
            }
        };
        end_bench!(save_canvas);
    }

    pub fn on_zoom_out(app: &mut appctx::ApplicationContext<'_>, _element: UIElementHandle) {
        start_bench!(stopwatch, zoom_out);
        let framebuffer = app.get_framebuffer_ref();
        match framebuffer.dump_region(CANVAS_REGION) {
            Err(err) => println!("Failed to dump buffer: {0}", err),
            Ok(buff) => {
                let resized = image::DynamicImage::ImageRgb8(
                    storage::rgbimage_from_u8_slice(
                        CANVAS_REGION.width,
                        CANVAS_REGION.height,
                        buff.as_slice(),
                    )
                    .unwrap(),
                )
                .resize(
                    (CANVAS_REGION.width as f32 / 1.25f32) as u32,
                    (CANVAS_REGION.height as f32 / 1.25f32) as u32,
                    image::imageops::Nearest,
                );

                // Get a clean image the size of the canvas
                let mut new_image =
                    image::DynamicImage::new_rgb8(CANVAS_REGION.width, CANVAS_REGION.height);
                new_image.invert();

                // Copy the resized image into the subimage
                new_image
                    .copy_from(&resized, CANVAS_REGION.width / 8, CANVAS_REGION.height / 8)
                    .unwrap();

                framebuffer.draw_image(
                    new_image.as_rgb8().unwrap(),
                    CANVAS_REGION.top_left().cast().unwrap(),
                );
                framebuffer.partial_refresh(
                    &CANVAS_REGION,
                    PartialRefreshMode::Async,
                    waveform_mode::WAVEFORM_MODE_GC16_FAST,
                    display_temp::TEMP_USE_REMARKABLE_DRAW,
                    dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                    0,
                    false,
                );
            }
        };
        end_bench!(zoom_out);
    }

    pub fn on_blur_canvas(app: &mut appctx::ApplicationContext<'_>, _element: UIElementHandle) {
        start_bench!(stopwatch, blur_canvas);
        let framebuffer = app.get_framebuffer_ref();
        match framebuffer.dump_region(CANVAS_REGION) {
            Err(err) => println!("Failed to dump buffer: {0}", err),
            Ok(buff) => {
                let dynamic = image::DynamicImage::ImageRgb8(
                    storage::rgbimage_from_u8_slice(
                        CANVAS_REGION.width,
                        CANVAS_REGION.height,
                        buff.as_slice(),
                    )
                    .unwrap(),
                )
                .blur(0.6f32);

                framebuffer.draw_image(
                    dynamic.as_rgb8().unwrap(),
                    CANVAS_REGION.top_left().cast().unwrap(),
                );
                framebuffer.partial_refresh(
                    &CANVAS_REGION,
                    PartialRefreshMode::Async,
                    waveform_mode::WAVEFORM_MODE_GC16_FAST,
                    display_temp::TEMP_USE_REMARKABLE_DRAW,
                    dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                    0,
                    false,
                );
            }
        };
        end_bench!(blur_canvas);
    }

    pub fn on_invert_canvas(app: &mut appctx::ApplicationContext<'_>, element: UIElementHandle) {
        start_bench!(stopwatch, invert);
        let framebuffer = app.get_framebuffer_ref();
        match framebuffer.dump_region(CANVAS_REGION) {
            Err(err) => println!("Failed to dump buffer: {0}", err),
            Ok(mut buff) => {
                buff.iter_mut().for_each(|p| {
                    *p = !(*p);
                });
                match framebuffer.restore_region(CANVAS_REGION, &buff) {
                    Err(e) => println!("Error while restoring region: {0}", e),
                    Ok(_) => {
                        framebuffer.partial_refresh(
                            &CANVAS_REGION,
                            PartialRefreshMode::Async,
                            waveform_mode::WAVEFORM_MODE_GC16_FAST,
                            display_temp::TEMP_USE_REMARKABLE_DRAW,
                            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                            0,
                            false,
                        );
                    }
                };
            }
        };
        end_bench!(invert);

        // Invert the draw color as well for more natural UX
        on_toggle_eraser(app, element);
    }

    pub fn on_load_canvas(app: &mut appctx::ApplicationContext<'_>, _element: UIElementHandle) {
        start_bench!(stopwatch, load_canvas);
        match *SAVED_CANVAS.lock().unwrap() {
            None => {}
            Some(ref compressed_state) => {
                let framebuffer = app.get_framebuffer_ref();
                let decompressed = compressed_state.decompress();

                match framebuffer.restore_region(CANVAS_REGION, &decompressed) {
                    Err(e) => println!("Error while restoring region: {0}", e),
                    Ok(_) => {
                        framebuffer.partial_refresh(
                            &CANVAS_REGION,
                            PartialRefreshMode::Async,
                            waveform_mode::WAVEFORM_MODE_GC16_FAST,
                            display_temp::TEMP_USE_REMARKABLE_DRAW,
                            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                            0,
                            false,
                        );
                    }
                };
            }
        };
        end_bench!(load_canvas);
    }

    pub fn on_toggle_eraser(app: &mut appctx::ApplicationContext<'_>, _: UIElementHandle) {
        let (new_mode, name) = match G_DRAW_MODE.load(Ordering::Relaxed) {
            DrawMode::Erase(s) => (DrawMode::Draw(s), "Black".to_owned()),
            DrawMode::Draw(s) => (DrawMode::Erase(s), "White".to_owned()),
        };
        G_DRAW_MODE.store(new_mode, Ordering::Relaxed);

        let indicator = app.get_element_by_name("colorIndicator");
        if let UIElement::Text { ref mut text, .. } = indicator.unwrap().write().inner {
            *text = name;
        }
        app.draw_element("colorIndicator");
    }
}
