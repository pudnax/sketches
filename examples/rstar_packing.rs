use nannou::prelude::*;

extern crate rstar;
use rstar::Point;
use rstar::RTree;

fn main() {
    nannou::app(model).update(update).run();
}

const MARGIN: f32 = 0.;
const _MAX_R: f32 = 40.;
const _MIN_R: f32 = 5.;

struct Model {
    _window: window::Id,
    circles: Vec<Circle>,
    failed_tries: u64,
    current_radius: f32,
    tree: RTree<Circle>,
}

impl Model {
    fn valid_circle(&self, aspt: &Circle) -> bool {
        if pt2(aspt.x, aspt.y).distance(pt2(0., 0.)) > 300. {
            return false;
        }

        let nearest = self
            .tree
            .nearest_neighbor(&aspt)
            .expect("can't find nearest");
        if aspt.collides(&nearest) {
            return false;
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
    let (w, h) = app.window_rect().w_h();
    let circle = Circle::new(
        random_range(-w / 2., w / 2.),
        random_range(-h / 2., h / 2.),
        20.,
        hsv(358. / 360., random_range(0.4, 1.), 0.76),
    );
    let mut tree = RTree::new();
    tree.insert(circle);
    Model {
        _window,
        circles: Vec::new(),
        failed_tries: 0,
        current_radius: 45.,
        tree,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for _ in 0..10 {
        let (w, h) = app.window_rect().w_h();
        let aspt = Circle::new(
            random_range(-w / 2., w / 2.),
            random_range(-h / 2., h / 2.),
            model.current_radius,
            hsv(358. / 360., random_range(0.4, 1.), 0.76),
        );

        if model.valid_circle(&aspt) {
            model.circles.push(aspt);
            model.tree.insert(aspt);
        } else {
            model.failed_tries += 1;
            if model.failed_tries > 64 * 1024 / model.current_radius as u64 {
                model.current_radius /= 1.5;
                model.failed_tries = 0;
            }
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

#[derive(Copy, Clone)]
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

impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.r)
    }
}

impl std::cmp::PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point for Circle {
    type Scalar = f32;
    const DIMENSIONS: usize = 2;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        Circle {
            x: generator(0),
            y: generator(1),
            r: 0.,
            col: hsv(1., 1., 1.),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x,
            1 => self.y,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!(),
        }
    }
}
