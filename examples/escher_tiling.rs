use nannou::prelude::*;

extern crate num_complex;

mod cliping;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    a: WindowId,
    b: WindowId,
}

fn model(app: &App) -> Model {
    let a = app
        .new_window()
        .with_title("window a")
        .event(event_a)
        .build()
        .unwrap();
    let b = app
        .new_window()
        .with_title("window b")
        .event(event_b)
        .build()
        .unwrap();

    Model { a, b }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event_a(_app: &App, _model: &mut Model, event: WindowEvent) {
    // println!("window a: {:?}", event);
}

fn event_b(_app: &App, _model: &mut Model, event: WindowEvent) {
    // println!("window b: {:?}", event);
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw_for_window(frame.window_id()).unwrap();
    let (w, h) = app.window_rect().w_h();
    let (left, right, up, down) = (-w / 2., w / 2., h / 2., -h / 2.);
    let n = 10;
    let vertices = (0..n)
        .map(|i| {
            let frac = i as f32 / n as f32;
            pt2(left, up).lerp(pt2(right, down), frac)
        })
        .enumerate()
        .map(|(_i, p)| {
            let rgba = srgba(0.5, 0.5, 1., 1.0);
            (p, rgba)
        });
    match frame.window_id() {
        id if id == model.a => {
            draw.background().color(INDIANRED);
            draw.polyline()
                .weight(5.)
                .join_round()
                .colored_points(vertices);
        }
        id if id == model.b => {
            draw.background().color(INDIANRED);
            let vertices = vertices.map(|(p, rgba)| {
                let (x, y) = (p.x.exp() * p.y.cos(), p.x.exp() * p.y.sin());
                (pt2(x, y), rgba)
            });
            draw.polyline()
                .weight(5.)
                .join_round()
                .colored_points(vertices);
        }
        _ => (),
    }
    draw.to_frame(app, frame).unwrap();
}

fn lerp(v0: f64, v1: f64, d: f64) -> f64 {
    v0 + (v1 - v0) * d.max(0.).min(1.)
}
