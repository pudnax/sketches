extern crate nannou;
use nannou::prelude::*;

const WIDTH: f32 = 1024.;
const HEIGHT: f32 = 800.;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait(1));
    let _window = app
        .new_window()
        .with_dimensions(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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

fn view(app: &App, _model: &Model, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    // Clear the background to pink.
    draw.background().color(BLACK);
    // Draw a red ellipse with default size and position.

    draw_pattern(&draw, app.mouse.x, app.mouse.y);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_pattern(draw: &nannou::app::Draw, w: f32, h: f32) {
    let spacing = 5.;
    let mut i = -w;
    while i < h + w {
        draw.line().start((i, 0.).into()).end((i + h, h).into());
        i += spacing;
    }

    let mut i = h + w;
    while i >= -w {
        draw.line().start((i, 0.).into()).end((i - h, h).into());
        i -= spacing;
    }
}
