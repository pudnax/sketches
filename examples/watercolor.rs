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

    let win = app.window_rect();
    let _t = app.time;

    let r = win.w().min(win.h()) / 2.;
    let base_poly = create_base_poly(r, 10);
    let variation = deform(&base_poly, 5, random_range(r / 10., r / 4.), 4.);
    draw.polygon().join_round().color(MAROON).points(variation);

    for poly in polystack(r, 10) {
        draw.polygon().join_round().color(MAROON).points(poly);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn polystack(r: f32, nsides: u32) -> Vec<Vec<Point2>> {
    let mut stack = Vec::new();

    /* Generate a base polygon with depth 5 and variance 15 */
    let base_poly = rpoly(r, nsides);
    let base_poly = deform(&base_poly, 5, r / 10., 2.);

    /* Generate a variation of the base polygon with a random variance */
    for _ in 0..5 {
        let poly = deform(&base_poly, 5, random_range(r / 15., r / 5.), 4.);
        stack.push(poly);
    }

    stack
}

fn create_base_poly(r: f32, nsides: u32) -> Vec<Point2> {
    deform(&rpoly(r, nsides), 5, r / 2., 2.0)
}

fn rpoly(radius: f32, n_points: u32) -> Vec<Point2> {
    (0..n_points)
        .map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        })
        .collect()
}

fn deform(points: &Vec<Point2>, depth: i64, variance: f32, vdiv: f32) -> Vec<Point2> {
    let mut new_points = Vec::new();

    for i in 0..points.len() {
        let sx1 = points[i].x;
        let sy1 = points[i].y;
        let sx2 = points[(i + 1) % points.len()].x;
        let sy2 = points[(i + 1) % points.len()].y;

        new_points.push(pt2(sx1, sy1));
        sub_divide(&mut new_points, sx1, sy1, sx2, sy2, depth, variance, vdiv);
    }

    new_points
}

fn sub_divide(
    new_points: &mut Vec<Point2>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    depth: i64,
    variance: f32,
    vdiv: f32,
) {
    if depth >= 0 {
        let midx = (x1 + x2) / 2.;
        let midy = (y1 + y2) / 2.;
        let nx = midx + random_range(-0.5, 0.5) * variance;
        let ny = midy + random_range(-0.5, 0.5) * variance;

        sub_divide(new_points, x1, y1, nx, ny, depth - 1, variance / vdiv, vdiv);
        new_points.push(pt2(nx, ny));
        sub_divide(new_points, nx, ny, x2, y2, depth - 1, variance / vdiv, vdiv);
    }
}
