use nannou::prelude::*;

const speed_relation: f32 = 2.;
const center: Point2<f32> = Point2 { x: 0., y: 0. };

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    speed: f32,
    angle: f32,
    paths: Vec<Vec<(Point2, Srgba<f32>)>>,
    joints: usize,
    lenght: f32,
}

impl Model {
    fn refresh(&mut self) {
        self.joints = 5;
        let speed = 8. / 1.75.powf(self.joints as f32 - 1.) / 2f32.powf(speed_relation - 1.);
        self.speed = speed;
        self.angle = 0.;
        self.paths = vec![Vec::new(); self.joints];
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

    let joints = 5;
    let speed = 8. / 1.75.powf(joints as f32 - 1.) / 2f32.powf(speed_relation - 1.);
    Model {
        _window,
        paths: vec![Vec::new(); joints],
        speed,
        angle: 0.,
        lenght: 100.,
        joints,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mut pos = center;

    for i in 0..model.joints {
        // let mut a = -model.angle * speed_relation.powi(i as i32);
        let mut a = -model.angle;
        if i % 2 == 1 {
            a = -a;
        }

        let frac = (model.joints as f32 - i as f32) / model.joints as f32;

        let next_pos: Point2<f32> = Point2::one().with_magnitude(frac * model.lenght);
        let next_pos = rotate(next_pos, a) + pos;
        // dbg!(next_pos);

        model.paths[i].push((next_pos, rgba(frac * 0.5, 0., 1. - frac, 1.)));
        pos = next_pos;
    }

    model.angle += model.speed * 0.01;

    if model.angle > TAU / 2. {
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
    let weight = 4.0;

    for vertices in &model.paths {
        draw.polyline()
            .weight(weight)
            .colored_points(vertices.clone());
    }

    draw.to_frame(app, &frame).unwrap();
}

fn rotate(p: Point2, a: f32) -> Point2 {
    let x = p.x * a.cos() - p.y * a.sin();
    let y = p.x * a.sin() + p.y * a.cos();
    pt2(x, y)
}
