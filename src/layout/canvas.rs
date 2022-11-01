use crate::{
    draw_mode::DrawMode, touch_mode::TouchMode, UIConstraintRefresh, UIElement, UIElementWrapper,
};
use libremarkable::{appctx, framebuffer::common::*};

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
    use super::CANVAS_REGION;
    use crate::{
        appctx, display_temp, dither_mode, image, image::GenericImage, waveform_mode, DrawMode,
        FramebufferDraw, FramebufferRefresh, Lazy, Mutex, Ordering, PartialRefreshMode, UIElement,
        UIElementHandle, G_DRAW_MODE,
    };
    use libremarkable::{
        end_bench,
        framebuffer::{storage, FramebufferIO},
        start_bench,
    };

    static SAVED_CANVAS: Lazy<Mutex<Option<storage::CompressedCanvasState>>> =
        Lazy::new(|| Mutex::new(None));

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
