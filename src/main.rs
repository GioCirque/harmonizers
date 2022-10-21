#[macro_use]
extern crate downcast_rs;

mod canvas;
mod drawable;

use crate::canvas::Canvas;
use crate::drawable::*;
use libremarkable::input::{ev::EvDevContext, InputDevice, InputEvent};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let mut canvas = Canvas::new();
    let mut current_scene: Box<dyn Drawable> = Box::new(AppChrome::new());
    let (input_tx, input_rx) = std::sync::mpsc::channel::<InputEvent>();
    EvDevContext::new(InputDevice::GPIO, input_tx.clone()).start();
    EvDevContext::new(InputDevice::Multitouch, input_tx).start();

    canvas.clear();
    canvas.update_full();

    const FPS: u16 = 30;
    const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS as u64);

    loop {
        let before_input = Instant::now();

        current_scene.draw(&mut canvas);
        for event in input_rx.try_iter() {
            current_scene.on_input(event);
        }

        let next_scene = current_scene.update(&mut canvas);
        if next_scene.is_some() {
            current_scene = next_scene.unwrap();
        }

        // Wait remaining frame time
        let elapsed = before_input.elapsed();
        if elapsed < FRAME_DURATION {
            sleep(FRAME_DURATION - elapsed);
        }
    }
}
