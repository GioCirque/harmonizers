#[macro_use]
extern crate downcast_rs;

mod canvas;
mod drawable;

use crate::canvas::Canvas;
use crate::drawable::*;

use libremarkable::input::{ev::EvDevContext, InputDevice, InputEvent};

fn main() {
    let mut canvas = Canvas::new();
    let mut current_scene: Box<dyn Drawable> = Box::new(AppChrome::new());
    let (input_tx, input_rx) = std::sync::mpsc::channel::<InputEvent>();
    EvDevContext::new(InputDevice::GPIO, input_tx.clone()).start();
    EvDevContext::new(InputDevice::Multitouch, input_tx).start();

    canvas.clear();
    canvas.update_full();

    loop {
        for event in input_rx.try_iter() {
            current_scene.on_input(event);
        }
        current_scene.draw(&mut canvas);

        let next_scene = current_scene.update(&mut canvas);
        if next_scene.is_some() {
            current_scene = next_scene.unwrap();
        }
    }
}
