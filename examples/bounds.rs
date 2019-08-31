extern crate nannou;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points: Vec<Vector2>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(1024, 800)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    let points = (0..500)
        .map(|_x| Vector2::from((random_range(-512., 512.), random_range(-400., 400.))))
        .collect::<Vec<_>>();
    Model { _window, points }
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

fn view(app: &App, model: &Model, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    // Clear the background to pink.
    draw.background().color(BLACK);
    // Draw a red ellipse with default size and position.

    for p in &model.points {
        draw.ellipse().w_h(1., 1.).x_y(p.x, p.y);
    }

    let range = 80.;
    let md = 300.;

    let mx = app.mouse.x;
    let my = app.mouse.y;
    for i in 0..model.points.len() {
        let p0 = model.points[i];
        for j in (i + 1)..model.points.len() - 1 {
            let p1 = model.points[j];
            let dist = p0.distance(p1);
            let max_dist = md.min(range * range / Vector2::from((mx, my)).distance((p0 + p1) / 2.));
            if dist < max_dist {
                draw.line()
                    .weight(2. * (1. - dist / max_dist))
                    .start(p0)
                    .end(p1);
            }
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
