mod layout;
use crate::layout::{canvas_handlers, CANVAS_REGION};

use libremarkable::framebuffer::cgmath;
use libremarkable::framebuffer::cgmath::EuclideanSpace;
use libremarkable::framebuffer::common::*;
use libremarkable::framebuffer::PartialRefreshMode;
use libremarkable::framebuffer::{
    common::DISPLAYHEIGHT, common::DISPLAYWIDTH, FramebufferDraw, FramebufferRefresh,
};
use libremarkable::input::{InputDevice, InputEvent};
use libremarkable::ui_extensions::element::{
    UIConstraintRefresh, UIElement, UIElementHandle, UIElementWrapper,
};
use libremarkable::{appctx, image, input};

#[cfg(feature = "enable-runtime-benchmarking")]
use libremarkable::stopwatch;

use atomic::Atomic;
use log::info;
use once_cell::sync::Lazy;

use std::collections::VecDeque;
use std::fmt;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

const APP_BUTTON_TOP: u16 = 42;
const APP_BUTTON_GAP_H: u16 = 10;
const APP_BUTTON_GAP_V: u16 = 10;
const APP_BUTTON_WIDTH: u16 = 32;
const APP_BUTTON_WIDTH_HALF: u16 = APP_BUTTON_WIDTH / 2;
const APP_BUTTON_SPACE_H: u16 = APP_BUTTON_WIDTH + (APP_BUTTON_GAP_H * 2);
const APP_BUTTON_SPACE_V: u16 = APP_BUTTON_WIDTH + (APP_BUTTON_GAP_V * 2);
const DISPLAY_EDGE_TOP: u16 = 0;
const DISPLAY_EDGE_RIGHT: u16 = DISPLAYWIDTH;

#[derive(Copy, Clone, PartialEq)]
enum DrawMode {
    Draw(u32),
    Erase(u32),
}
impl DrawMode {
    fn set_size(self, new_size: u32) -> Self {
        match self {
            DrawMode::Draw(_) => DrawMode::Draw(new_size),
            DrawMode::Erase(_) => DrawMode::Erase(new_size),
        }
    }
    fn color_as_string(self) -> String {
        match self {
            DrawMode::Draw(_) => "Black",
            DrawMode::Erase(_) => "White",
        }
        .into()
    }
    fn get_size(self) -> u32 {
        match self {
            DrawMode::Draw(s) => s,
            DrawMode::Erase(s) => s,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum TouchMode {
    OnlyUI,
    Bezier,
    Circles,
    Diamonds,
    FillDiamonds,
}
impl TouchMode {
    fn toggle(self) -> Self {
        match self {
            TouchMode::OnlyUI => TouchMode::Bezier,
            TouchMode::Bezier => TouchMode::Circles,
            TouchMode::Circles => TouchMode::Diamonds,
            TouchMode::Diamonds => TouchMode::FillDiamonds,
            TouchMode::FillDiamonds => TouchMode::OnlyUI,
        }
    }
}

impl fmt::Display for TouchMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode = match self {
            TouchMode::OnlyUI => "None",
            TouchMode::Bezier => "Bezier",
            TouchMode::Circles => "Circles",
            TouchMode::Diamonds => "Diamonds",
            TouchMode::FillDiamonds => "FDiamonds",
        };
        write!(f, "{}", mode)
    }
}

static G_TOUCH_MODE: Lazy<Atomic<TouchMode>> = Lazy::new(|| Atomic::new(TouchMode::OnlyUI));
static G_DRAW_MODE: Lazy<Atomic<DrawMode>> = Lazy::new(|| Atomic::new(DrawMode::Draw(2)));
static UNPRESS_OBSERVED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static WACOM_IN_RANGE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static WACOM_RUBBER_SIDE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static WACOM_HISTORY: Lazy<Mutex<VecDeque<(cgmath::Point2<f32>, i32)>>> =
    Lazy::new(|| Mutex::new(VecDeque::new()));

// ####################
// ## Button Handlers
// ####################

fn on_change_touchdraw_mode(app: &mut appctx::ApplicationContext<'_>, _: UIElementHandle) {
    let new_val = G_TOUCH_MODE.load(Ordering::Relaxed).toggle();
    G_TOUCH_MODE.store(new_val, Ordering::Relaxed);

    let indicator = app.get_element_by_name("touchModeIndicator");
    if let UIElement::Text { ref mut text, .. } = indicator.unwrap().write().inner {
        *text = new_val.to_string();
    }
    // Make sure you aren't trying to draw the element while you are holding a write lock.
    // It doesn't seem to cause a deadlock however it may cause higher lock contention.
    app.draw_element("touchModeIndicator");
}

// ####################
// ## Miscellaneous
// ####################

/// Called on button press on rm2 or left gpio on rm1
fn quick_redraw(app: &mut appctx::ApplicationContext<'_>) {
    app.clear(false);
    app.draw_elements();
}

/// Called on button press on rm2 or middle gpio on rm1
fn full_redraw(app: &mut appctx::ApplicationContext<'_>) {
    app.clear(true);
    app.draw_elements();
}

/// Called on button press (pen can press, too) on rm2 or right gpio on rm1
fn toggle_touch(app: &mut appctx::ApplicationContext<'_>) {
    let new_state = match app.is_input_device_active(InputDevice::Multitouch) {
        true => {
            app.deactivate_input_device(InputDevice::Multitouch);
            "Enable Touch"
        }
        false => {
            app.activate_input_device(InputDevice::Multitouch);
            "Disable Touch"
        }
    };

    if let Some(ref elem) = app.get_element_by_name("toggleTouch") {
        if let UIElement::Text {
            ref mut text,
            scale: _,
            foreground: _,
            border_px: _,
        } = elem.write().inner
        {
            *text = new_state.to_string();
        }
    }
    app.draw_element("toggleTouch");
}

fn change_brush_width(app: &mut appctx::ApplicationContext<'_>, delta: i32) {
    let current = G_DRAW_MODE.load(Ordering::Relaxed);
    let current_size = current.get_size() as i32;
    let proposed_size = current_size + delta;
    let new_size = if proposed_size < 1 {
        1
    } else if proposed_size > 99 {
        99
    } else {
        proposed_size
    };
    if new_size == current_size {
        return;
    }

    G_DRAW_MODE.store(current.set_size(new_size as u32), Ordering::Relaxed);

    let element = app.get_element_by_name("displaySize").unwrap();
    if let UIElement::Text { ref mut text, .. } = element.write().inner {
        *text = format!("size: {0}", new_size);
    }
    app.draw_element("displaySize");
}

// ####################
// ## Input Handlers
// ####################

fn on_wacom_input(app: &mut appctx::ApplicationContext<'_>, input: input::WacomEvent) {
    match input {
        input::WacomEvent::Draw {
            position,
            pressure,
            tilt: _,
        } => {
            let mut wacom_stack = WACOM_HISTORY.lock().unwrap();

            // This is so that we can click the buttons outside the canvas region
            // normally meant to be touched with a finger using our stylus
            if !CANVAS_REGION.contains_point(&position.cast().unwrap()) {
                wacom_stack.clear();
                if UNPRESS_OBSERVED.fetch_and(false, Ordering::Relaxed) {
                    let region = app
                        .find_active_region(position.y.round() as u16, position.x.round() as u16);
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
        input::WacomEvent::InstrumentChange { pen, state } => {
            match pen {
                // Whether the pen is in range
                input::WacomPen::ToolPen => {
                    WACOM_IN_RANGE.store(state, Ordering::Relaxed);
                    WACOM_RUBBER_SIDE.store(false, Ordering::Relaxed);
                }
                input::WacomPen::ToolRubber => {
                    WACOM_IN_RANGE.store(state, Ordering::Relaxed);
                    WACOM_RUBBER_SIDE.store(true, Ordering::Relaxed);
                }
                // Whether the pen is actually making contact
                input::WacomPen::Touch => {
                    // Stop drawing when instrument has left the vicinity of the screen
                    if !state {
                        let mut wacom_stack = WACOM_HISTORY.lock().unwrap();
                        wacom_stack.clear();
                    }
                }
                _ => unreachable!(),
            }
        }
        input::WacomEvent::Hover {
            position: _,
            distance,
            tilt: _,
        } => {
            // If the pen is hovering, don't record its coordinates as the origin of the next line
            if distance > 1 {
                let mut wacom_stack = WACOM_HISTORY.lock().unwrap();
                wacom_stack.clear();
                UNPRESS_OBSERVED.store(true, Ordering::Relaxed);
            }
        }
        _ => {}
    };
}

fn on_touch_handler(app: &mut appctx::ApplicationContext<'_>, input: input::MultitouchEvent) {
    let framebuffer = app.get_framebuffer_ref();
    match input {
        input::MultitouchEvent::Press { finger } | input::MultitouchEvent::Move { finger } => {
            if !CANVAS_REGION.contains_point(&finger.pos.cast().unwrap()) {
                return;
            }
            let rect = match G_TOUCH_MODE.load(Ordering::Relaxed) {
                TouchMode::Bezier => {
                    let position_float = finger.pos.cast().unwrap();
                    let points = vec![
                        (cgmath::vec2(-40.0, 0.0), 2.5),
                        (cgmath::vec2(40.0, -60.0), 5.5),
                        (cgmath::vec2(0.0, 0.0), 3.5),
                        (cgmath::vec2(-40.0, 60.0), 6.5),
                        (cgmath::vec2(-10.0, 50.0), 5.0),
                        (cgmath::vec2(10.0, 45.0), 4.5),
                        (cgmath::vec2(30.0, 55.0), 3.5),
                        (cgmath::vec2(50.0, 65.0), 3.0),
                        (cgmath::vec2(70.0, 40.0), 0.0),
                    ];
                    let mut rect = mxcfb_rect::invalid();
                    for window in points.windows(3).step_by(2) {
                        rect = rect.merge_rect(&framebuffer.draw_dynamic_bezier(
                            (position_float + window[0].0, window[0].1),
                            (position_float + window[1].0, window[1].1),
                            (position_float + window[2].0, window[2].1),
                            100,
                            color::BLACK,
                        ));
                    }
                    rect
                }
                TouchMode::Circles => {
                    framebuffer.draw_circle(finger.pos.cast().unwrap(), 20, color::BLACK)
                }

                m @ TouchMode::Diamonds | m @ TouchMode::FillDiamonds => {
                    let position_int = finger.pos.cast().unwrap();
                    framebuffer.draw_polygon(
                        &[
                            position_int + cgmath::vec2(-10, 0),
                            position_int + cgmath::vec2(0, 20),
                            position_int + cgmath::vec2(10, 0),
                            position_int + cgmath::vec2(0, -20),
                        ],
                        match m {
                            TouchMode::Diamonds => false,
                            TouchMode::FillDiamonds => true,
                            _ => false,
                        },
                        color::BLACK,
                    )
                }
                _ => return,
            };
            framebuffer.partial_refresh(
                &rect,
                PartialRefreshMode::Async,
                waveform_mode::WAVEFORM_MODE_DU,
                display_temp::TEMP_USE_REMARKABLE_DRAW,
                dither_mode::EPDC_FLAG_USE_DITHERING_ALPHA,
                DRAWING_QUANT_BIT,
                false,
            );
        }
        _ => {}
    }
}

fn on_button_press(app: &mut appctx::ApplicationContext<'_>, input: input::GPIOEvent) {
    let (btn, new_state) = match input {
        input::GPIOEvent::Press { button } => (button, true),
        input::GPIOEvent::Unpress { button } => (button, false),
        _ => return,
    };

    // Ignoring the unpressed event
    if !new_state {
        return;
    }

    // Simple but effective accidental button press filtering
    if WACOM_IN_RANGE.load(Ordering::Relaxed) {
        return;
    }

    match btn {
        input::PhysicalButton::LEFT => quick_redraw(app),
        input::PhysicalButton::MIDDLE => full_redraw(app),
        input::PhysicalButton::RIGHT => toggle_touch(app),

        input::PhysicalButton::POWER => {
            Command::new("systemctl")
                .arg("start")
                .arg("xochitl")
                .spawn()
                .unwrap();
            std::process::exit(0);
        }
        input::PhysicalButton::WAKEUP => {
            println!("WAKEUP button(?) pressed(?)");
        }
    };
}

fn main() {
    env_logger::init();

    // Takes callback functions as arguments
    // They are called with the event and the &mut framebuffer
    let mut app: appctx::ApplicationContext<'_> = appctx::ApplicationContext::default();

    // Perform the initial layout
    layout::init(&mut app);

    // Draw the scene
    app.draw_elements();

    info!("Init complete. Beginning event dispatch...");

    // Blocking call to process events from digitizer + touchscreen + physical buttons
    app.start_event_loop(true, true, true, |ctx, evt| match evt {
        InputEvent::WacomEvent { event } => on_wacom_input(ctx, event),
        InputEvent::MultitouchEvent { event } => on_touch_handler(ctx, event),
        InputEvent::GPIO { event } => on_button_press(ctx, event),
        _ => {}
    });
}
