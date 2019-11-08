extern crate nannou;
use nannou::noise::NoiseFn;
use nannou::prelude::*;

use std::convert::From;

const WIDTH: f32 = 1024.;
const HEIGHT: f32 = 800.;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    perlin: nannou::noise::Perlin,
    counter: f64,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    Model {
        _window,
        perlin: nannou::noise::Perlin::new(),
        counter: 0.,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.counter += 0.01;
}

fn window_event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    // Clear the background to pink.
    draw.background().color(BLACK);
    // Draw a red ellipse with default size and position.

    for i in linspace(-WIDTH / 2., WIDTH / 2., 50) {
        for j in linspace(-HEIGHT / 2., HEIGHT / 2., 50) {
            draw.ellipse().x_y(i, j).w_h(5., 5.);
            draw.line()
                .xy([i, j].into())
                .end(Vector2::from([10., 0.]))
                .rotate(
                    model
                        .perlin
                        .get([i as f64 * 0.01, j as f64 * 0.01, model.counter])
                        as f32
                        * 2.
                        * std::f32::consts::PI,
                );
        }
    }
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn linspace(start: f32, stop: f32, nstep: u32) -> Vec<f32> {
    let delta: f32 = (stop - start) / f32::from_u32(nstep - 1).expect("out of range");
    (0..(nstep))
        .map(|i| start + f32::from_u32(i).expect("out of range") * delta)
        .collect()
}
