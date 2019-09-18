use nannou::prelude::*;

extern crate num_complex;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    a: WindowId,
    b: WindowId,
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::wait(3));

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
    let t = app.time;

    let (w, h) = app.window_rect().w_h();
    let (left, right, up, down) = (-w / 2., w / 2., h / 2., -h / 2.);

    let grid = square_fill(left, down, w, 20., 10.);

    let weight = 10. * t.cos();
    match frame.window_id() {
        id if id == model.a => {}
        id if id == model.b => {
            draw.background().color(BLACK);
            draw.ellipse().color(BLACK).w_h(1., 10.);

            // for seq in grid {
            //     draw.polyline().weight(10. * t.cos()).points(seq);
            // }
            let mut tris = Vec::new();
            for seq in grid.iter().skip(10).take(10) {
                tris.extend(
                    // grid[20]
                    seq.windows(2)
                        .flat_map(|slice| {
                            let theta = (slice[1] - slice[0]).angle() + TAU;
                            let dev = pt2(weight * theta.cos(), weight * theta.sin());
                            let a = slice[0] + dev;
                            let b = slice[0] - dev;
                            let c = slice[1] + dev;
                            let d = slice[1] - dev;
                            geom::Quad([a, b, c, d]).triangles_iter()
                        })
                        .map(|tri| {
                            // Color the vertices based on their amplitude.
                            tri.map_vertices(|v| {
                                let color = srgba(0.5, 0.5, 0.5, 1.0);
                                geom::vertex::Srgba(v, color)
                            })
                        }),
                );
            }

            draw.mesh().tris(tris);
        }
        _ => (),
    }
    draw.to_frame(app, frame).unwrap();
}

fn lerp(v0: f64, v1: f64, d: f64) -> f64 {
    v0 + (v1 - v0) * d.max(0.).min(1.)
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

pub fn square_fill(x: f32, y: f32, w: f32, step: f32, a: f32) -> Vec<Vec<Vector2>> {
    let xstart = x + w / 2.;
    let ystart = y + w / 2.;
    // let xstart = x + random_range(0., w);
    // let ystart = y + random_range(0., w);

    let slope = a.tan();
    let c = ystart - slope * xstart;

    let mut down_accept = true;
    let mut up_accept = true;

    let mut i = 0;

    let mut arr = Vec::new();

    //for (int i = 0; i < w / step; i++) {
    while down_accept || up_accept {
        let mut x0 = x - w / 2.;
        let mut y0 = slope * x0 + c + i as f32 * step / a.cos();
        let mut x1 = x + w + w / 2.;
        let mut y1 = slope * x1 + c + i as f32 * step / a.cos();;
        match line_clipped(x0, y0, x1, y1, x, y, w, w) {
            Some((x, y)) => {
                up_accept = true;
                let num_poinst = 10;
                let vertices = (0..num_poinst)
                    .map(|i| {
                        let frac = i as f32 / num_poinst as f32;
                        pt2(x[0], x[1]).lerp(pt2(y[0], y[1]), frac)
                    })
                    .collect();
                arr.push(vertices);
            }
            None => up_accept = false,
        };

        x0 = x - w / 2.;
        y0 = slope * x0 + c - i as f32 * step / a.cos();
        x1 = x + w + w / 2.;
        y1 = slope * x1 + c - i as f32 * step / a.cos();
        match line_clipped(x0, y0, x1, y1, x, y, w, w) {
            Some((x, y)) => {
                down_accept = true;
                let num_poinst = 10;
                let vertices = (0..num_poinst)
                    .map(|i| {
                        let frac = i as f32 / num_poinst as f32;
                        pt2(x[0], x[1]).lerp(pt2(y[0], y[1]), frac)
                    })
                    .collect();
                arr.push(vertices);
            }
            None => down_accept = false,
        };
        i += 1;
    }

    arr
}

pub fn mesh_from_arr(arr: &[Vec<Vector2>], weight: f32, draw: &nannou::app::Draw) {
    for seq in arr {
        let tris = seq
            .windows(2)
            .flat_map(|slice| {
                let theta = (slice[1] - slice[0]).angle();
                let dev = pt2(weight * theta.cos(), weight * theta.sin());
                let a = slice[0] + dev;
                let b = slice[1] - dev;
                let c = slice[0] + dev;
                let d = slice[1] - dev;
                geom::Quad([a, b, c, d]).triangles_iter()
            })
            .map(|tri| {
                // Color the vertices based on their amplitude.
                tri.map_vertices(|v| {
                    let color = srgba(0.5, 0.5, 0.5, 1.0);
                    geom::vertex::Srgba(v, color)
                })
            });
        draw.mesh().tris(tris);
    }
}
