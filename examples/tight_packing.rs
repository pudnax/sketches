use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

const MARGIN: f32 = -5.;
const _MAX_R: f32 = 40.;
const _MIN_R: f32 = 5.;

struct Model {
    _window: window::Id,
    circles: Vec<Circle>,
    failed_tries: u64,
    current_radius: f32,
}

impl Model {
    fn valid_circle(&self, aspt: &Circle) -> bool {
        if pt2(aspt.x, aspt.y).distance(pt2(0., 0.)) > 300. {
            return false;
        }

        for circle in &self.circles {
            if aspt.collides(&circle) {
                return false;
            }
        }
        true
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
        circles: Vec::new(),
        failed_tries: 0,
        current_radius: 20.,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let (w, h) = app.window_rect().w_h();
    let aspt = Circle::new(
        random_range(-w / 2., w / 2.),
        random_range(-h / 2., h / 2.),
        model.current_radius,
        hsv(358. / 360., random_range(0.4, 1.), 0.76),
    );

    if model.valid_circle(&aspt) {
        model.circles.push(aspt);
    } else {
        model.failed_tries += 1;
        if model.failed_tries > 32 * 1024 / model.current_radius as u64 {
            model.current_radius /= 2.;
            model.failed_tries = 0;
        }
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
    // Draw a red ellipse with default size and position.
    for circle in &model.circles {
        circle.draw(&draw);
    }
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

struct Circle {
    x: f32,
    y: f32,
    r: f32,
    col: nannou::color::Hsv,
}

impl Circle {
    fn new(x: f32, y: f32, r: f32, col: nannou::color::Hsv) -> Circle {
        Circle { x, y, r, col }
    }

    fn draw(&self, draw: &nannou::app::Draw) {
        draw.ellipse()
            .color(self.col)
            .x_y(self.x, self.y)
            .w_h(2. * self.r, 2. * self.r);
    }

    fn collides(&self, c: &Circle) -> bool {
        let dist = (c.x - self.x) * (c.x - self.x) + (c.y - self.y) * (c.y - self.y);
        if dist <= (self.r + c.r + MARGIN).powi(2) {
            return true;
        }
        false
    }
}
