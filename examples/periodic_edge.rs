extern crate nannou;
use nannou::prelude::*;

const WIDTH: f32 = 1024.;
const HEIGHT: f32 = 800.;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points: Vec<Point2>,
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
        points: Vec::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.points.push(Point2::from((
        random_range(-0.5, 0.5),
        random_range(-0.5, 0.5),
    )));
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

fn view(app: &App, _model: &Model, frame: &Frame) {
    let draw = app.draw();
    // draw.background().color(BLACK);

    let frac = 2;
    let w = WIDTH / frac as f32;
    let h = HEIGHT / frac as f32;

    for xoff in (-frac..frac).map(|x| x as f32 * w) {
        for yoff in (-frac..frac).map(|x| x as f32 * h) {
            let rnum = random_f32();
            draw.ellipse()
                .x_y(
                    random_range(-0.5, 0.5) * w as f32 + xoff,
                    random_range(-0.5, 0.5) * h as f32 + yoff,
                )
                .w_h(rnum * 30. / frac as f32, rnum * 30. / frac as f32)
                .hsv(0.9, random_f32(), 0.53);
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
