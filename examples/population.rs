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
        points: vec![Cell::new(-20., 0.), Cell::new(20., 0.)],
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
        }
    });

    for i in 0..model.points.len() {
        for j in 0..model.points.len() {
            let aspt = model.points[j];
            if model.points[i].collides(&aspt) {
                acc.push(aspt.replicate());
            }
        }
    }

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
    fn new(x: f32, y: f32) -> Cell {
        let offx = random_f32();
        let offy = random_f32();
        Cell {
            pos: pt2(x, y),
            vel: pt2(1.1 * offx, -1.1 * offy),
            r: 5.,
        }
    }

    fn collides(&self, c: &Cell) -> bool {
        let dist = (c.pos.x - self.pos.x) * (c.pos.x - self.pos.x)
            + (c.pos.y - self.pos.y) * (c.pos.y - self.pos.y);
        if dist <= (self.r + c.r).powi(2) {
            return true;
        }
        false
    }

    fn bounce(&self) -> bool {
        (self.pos.x * self.pos.x + self.pos.y * self.pos.y).sqrt() + self.r > RADIUS
    }

    fn replicate(&self) -> Cell {
        let offx = 2. * random_f32() - 1.;
        let offy = 2. * random_f32() - 1.;
        let off = pt2(offx, offy).normalize() * self.r;
        let (x, y) = (self.vel.x, self.vel.y);

        Cell {
            pos: pt2(x + off.x, y + off.y),
            vel: self.vel,
            r: self.r,
        }
    }

    fn update(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }
}
