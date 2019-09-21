use nannou::prelude::*;

use std::time::Duration;

mod screenshot;

use screenshot::Shots;

struct Model {
    screenshot: Shots,
    capture: bool,
    counter: usize,
}

extern crate num_complex;

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .with_dimensions(1024, 768)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    let screenshot = screenshot::new(app, window_id);
    Model {
        screenshot,
        capture: true,
        counter: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.time;
    model.counter += 1;
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    let t = app.time;

    draw.background().color(BLACK);

    let (w, h) = app.window_rect().w_h();

    let frac = 6.;
    let lines = quad_fill(0., 0., w / frac, h / frac, 3., 1. + t * 0.025);
    // Here applies complex function
    mesh_from_arr(&lines, &draw, 0.05);

    draw.to_frame(app, &frame).unwrap();

    if model.capture && model.counter % 2 == 0 {
        model.screenshot.capture(&frame);
        model.screenshot.take();
    }
}

pub fn encode_endpoint(x: f32, y: f32, clipx: f32, clipy: f32, clipw: f32, cliph: f32) -> usize {
    let mut code = 0;

    let xmin = clipx;
    let xmax = clipx + clipw;
    let ymin = clipy;
    let ymax = clipy + cliph;

    if x < xmin {
        code |= 1 << 0;
    } else if x > xmax {
        code |= 1 << 1;
    }

    if y < ymin {
        code |= 1 << 2;
    } else if y > ymax {
        code |= 1 << 3;
    }

    code
}

pub fn line_clipped(
    mut x0: f32,
    mut y0: f32,
    mut x1: f32,
    mut y1: f32,
    clipx: f32,
    clipy: f32,
    clipw: f32,
    cliph: f32,
) -> Option<([f32; 2], [f32; 2])> {
    /* Stores encodings for the two endpoints of our line */
    //   int e0code, e1code;

    /* Calculate X and Y ranges for our clip window */
    let xmin = clipx;
    let xmax = clipx + clipw;
    let ymin = clipy;
    let ymax = clipy + cliph;

    /* Whether the line should be drawn or not */
    let mut accept = false;

    loop {
        /* Get encodings for the two endpoints of our line */
        let e0code = encode_endpoint(x0, y0, clipx, clipy, clipw, cliph);
        let e1code = encode_endpoint(x1, y1, clipx, clipy, clipw, cliph);

        if e0code == 0 && e1code == 0 {
            /* If line inside window, accept and break out of loop */
            accept = true;
            break;
        } else if (e0code & e1code) != 0 {
            /*
             * If the bitwise AND is not 0, it means both points share
             * an outside zone. Leave accept as 'false' and exit loop.
             */
            break;
        } else {
            /* Pick an endpoint that is outside the clip window */
            let code = if e0code != 0 { e0code } else { e1code };

            let mut newx = 0.;
            let mut newy = 0.;

            /*
             * Now figure out the new endpoint that needs to replace
             * the current one. Each of the four cases are handled
             * separately.
             */
            if (code & (1 << 0)) != 0 {
                /* Endpoint is to the left of clip window */
                newx = xmin;
                newy = ((y1 - y0) / (x1 - x0)) * (newx - x0) + y0;
            } else if (code & (1 << 1)) != 0 {
                /* Endpoint is to the right of clip window */
                newx = xmax;
                newy = ((y1 - y0) / (x1 - x0)) * (newx - x0) + y0;
            } else if (code & (1 << 3)) != 0 {
                /* Endpoint is above the clip window */
                newy = ymax;
                newx = ((x1 - x0) / (y1 - y0)) * (newy - y0) + x0;
            } else if (code & (1 << 2)) != 0 {
                /* Endpoint is below the clip window */
                newy = ymin;
                newx = ((x1 - x0) / (y1 - y0)) * (newy - y0) + x0;
            }

            /* Now we replace the old endpoint depending on which we chose */
            if code == e0code {
                x0 = newx;
                y0 = newy;
            } else {
                x1 = newx;
                y1 = newy;
            }
        }
    }

    //*!* Only draw the line if it was not rejected */
    if accept {
        return Some(([x0, y0], [x1, y1]));
    }

    None
}

pub fn quad_fill(x: f32, y: f32, width: f32, height: f32, step: f32, a: f32) -> Vec<Vec<Vector2>> {
    let lenght = (width * width + height * height).sqrt();
    let num_steps = lenght / (2. * step);

    let num_points = 1000;

    let mut arr = Vec::new();

    let rot = a + TAU / 4.;
    let rlen = 1.;
    let norm = pt2(rot.cos() / rlen, rot.sin() / rlen);
    let mut x0 = x + lenght / 2. * a.cos();
    let mut x1 = x + -lenght / 2. * a.cos();
    let mut y0 = y + lenght / 2. * a.sin();
    let mut y1 = y + -lenght / 2. * a.sin();
    if let Some((start, end)) = line_clipped(
        x0,
        y0,
        x1,
        y1,
        x - width / 2.,
        y - height / 2.,
        width,
        height,
    ) {
        arr.push(
            (0..num_points)
                .map(|i| {
                    pt2(start[0], start[1])
                        .lerp(pt2(end[0], end[1]), i as f32 / (num_points - 1) as f32)
                })
                .collect::<Vec<_>>(),
        )
    }
    for _ in 0..num_steps as usize {
        x0 += step * norm[0];
        x1 += step * norm[0];
        y0 += step * norm[1];
        y1 += step * norm[1];
        if let Some((start, end)) = line_clipped(
            x0,
            y0,
            x1,
            y1,
            x - width / 2.,
            y - height / 2.,
            width,
            height,
        ) {
            arr.push(
                (0..num_points)
                    .map(|i| {
                        pt2(start[0], start[1])
                            .lerp(pt2(end[0], end[1]), i as f32 / (num_points - 1) as f32)
                    })
                    .collect::<Vec<_>>(),
            )
        }
        if let Some((start, end)) = line_clipped(
            -x0 + 2. * x,
            -y0 + 2. * y,
            -x1 + 2. * x,
            -y1 + 2. * y,
            x - width / 2.,
            y - height / 2.,
            width,
            height,
        ) {
            arr.push(
                (0..num_points)
                    .map(|i| {
                        pt2(start[0], start[1])
                            .lerp(pt2(end[0], end[1]), i as f32 / (num_points - 1) as f32)
                    })
                    .collect::<Vec<_>>(),
            )
        }
    }

    arr
}

fn mesh_from_arr(arr: &[Vec<Vector2>], draw: &nannou::app::Draw, weight: f32) {
    let cfunc = |points: [Vector2; 4]| {
        let mut res = [pt2(0., 0.); 4];
        for (i, p) in points.iter().enumerate() {
            let mut acc = num_complex::Complex::new(p.x, p.y);
            acc = 0.001 * acc.exp();
            res[i] = pt2(acc.re, acc.im);
        }
        res
    };

    for line in arr {
        let num_points = line.len();
        let tris = line
            .windows(2)
            .flat_map(|slice| {
                let dev = (slice[1] - slice[0]).angle() + TAU / 4.;
                let dev = pt2(weight * dev.cos(), weight * dev.sin());
                let a = slice[0] + dev;
                let b = slice[0] - dev;
                let c = slice[1] + dev;
                let d = slice[1] - dev;
                let [a, b, c, d] = cfunc([a, b, c, d]);
                geom::Quad([a, c, d, b]).triangles_iter()
            })
            .enumerate()
            .map(|(i, tri)| {
                let i = i as f32 / num_points as f32;
                let mut j = 0.;
                tri.map_vertices(|v| {
                    let color = srgba(map_range(j, 0., 3., 0., 1.), i, 1. - i, 1.0);
                    j += 1.;
                    geom::vertex::Srgba(v, color)
                })
            });

        draw.mesh().tris(tris);
    }
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            if let Key::S = key {
                // Adds a screenshot to the queue to be taken
                model.screenshot.take();
            }
        }
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

fn exit(_: &App, model: Model) {
    // If you are getting an Access error then you
    // might need to raise the wait time
    model.screenshot.flush(Duration::from_secs(3));
}
