extern crate nannou;
use nannou::prelude::*;

const WIDTH: f32 = 1024.;
const HEIGHT: f32 = 800.;
const RADIUS: f32 = 300.;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points: Vec<Cell>,
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
        points: vec![Cell::new()],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut acc = Vec::new();
    model.points.iter_mut().for_each(|x| {
        let norm = -x.pos.normalize();
        let vel = x.vel.dot(norm);
        let magn = x.vel.magnitude();
        x.update();
        if x.bounce() {
            x.vel *= vel;
            x.vel = x.vel.normalize() * magn;
            acc.push(x.replicate());
        }
    });

    model.points.append(&mut acc);
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
    let draw = app.draw();
    draw.background().color(BLACK);

    for cell in &model.points {
        draw.ellipse().xy(cell.pos);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

#[derive(Copy, Clone)]
struct Cell {
    pos: Vector2<f32>,
    vel: Vector2<f32>,
    r: f32,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            pos: pt2(0., 0.),
            vel: pt2(1.1, -1.1),
            r: 10.,
        }
    }

    fn bounce(&self) -> bool {
        (self.pos.x * self.pos.x + self.pos.y * self.pos.y).sqrt() + self.r > RADIUS
    }

    fn replicate(&self) -> Cell {
        let offx = random_f32() * self.r;
        let offy = random_f32() * self.r;
        let (x, y) = (self.vel.x, self.vel.y);

        Cell {
            pos: self.pos,
            vel: pt2(x + offx, y + offy),
            r: self.r,
        }
    }

    fn update(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }
}
