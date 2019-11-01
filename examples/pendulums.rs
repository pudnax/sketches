use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    speed: f32,
    angle: f32,
    paths: Vec<(Point2, Srgba<f32>)>,
    joints: usize,
    lenght: f32,
}

impl Model {
    fn refresh(&mut self) {
        self.paths = Vec::new();
        self.speed = 5.;
        self.angle = 0.;
        self.joints = 5;
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    Model {
        _window,
        paths: Vec::new(),
        speed: 5.,
        angle: 0.,
        joints: 5,
        lenght: 100.,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let x = model.lenght * model.angle.cos();
    let y = model.lenght * model.angle.sin();
    model.paths.push((pt2(x, y), srgba(0., 0., 0., 1.)));
    model.angle += 0.01;
    if model.angle > TAU {
        model.refresh()
    }
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
    draw.background().color(SLATEGRAY);
    let win = app.window_rect();
    let t = app.time;

    let n_points = t as i32 + 1;
    let weight = 8.0;

    let hz = ((app.mouse.x + win.right()) / win.w()).powi(4) * 1000.0;
    let vertices = (0..=n_points)
        // A sine wave mapped to the range of the window.
        .map(|i| {
            let amp = 100.;
            let angle = TAU * i as f32 / n_points as f32;
            let x = amp * angle.cos();
            let y = amp * angle.sin();
            pt2(x, y)
        })
        .enumerate()
        // Colour each vertex uniquely based on its index.
        .map(|(i, p)| {
            let fract = i as f32 / n_points as f32;
            let r = (t + fract) % 1.0;
            let g = (t + 1.0 - fract) % 1.0;
            let b = (t + 0.5 + fract) % 1.0;
            let rgba = srgba(r, g, b, 1.0);
            (p, rgba)
        });

    // Draw the polyline as a stroked path.
    // for vertices in &model.paths {}
    draw.polyline()
        .weight(weight)
        .join_round()
        .colored_points(model.paths.clone());

    // Draw a red ellipse with default size and position.
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
