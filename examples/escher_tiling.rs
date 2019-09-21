use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    // Begin drawing
    let win = app.window_rect();
    let draw = app.draw();
    let t = app.time;

    // Clear the background to blue.
    draw.background().color(BLACK);

    // Use the mouse position to affect the frequency and amplitude.
    let (w, h) = app.window_rect().w_h();
    let (left, right, down, up) = (-w / 2., w / 2., -h / 2., h / 2.);
    let hx = map_range(app.mouse.x, win.left(), win.right(), 0.0, w / 2.);
    let hy = map_range(app.mouse.y, win.bottom(), win.top(), 0.0, h / 2.);
    let amp = app.mouse.y;

    let (x, y) = (app.mouse.x, app.mouse.y);
    let num_points = 16;

    let weight = 5.;
    let line = (0..num_points)
        .map(|i| pt2(0., 0.).lerp(pt2(x, y), i as f32 / (num_points - 1) as f32))
        .collect::<Vec<_>>();
    let tris = line
        .windows(2)
        // .iter()
        .flat_map(|slice| {
            let dev = (slice[1] - slice[0]).angle() + TAU / 4.;
            let dev = pt2(weight * dev.cos(), weight * dev.sin());
            let a = slice[0] + dev;
            let b = slice[0] - dev;
            let c = slice[1] + dev;
            let d = slice[1] - dev;
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

    let lines = quad_fill(0., 0., w, h, 20., t * 0.1);
    let lines = lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|&p| {
                    let (x, y) = (p.x * 0.1, p.y * 0.1);
                    pt2(x.exp() * y.cos(), x.exp() * y.sin())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    mesh_from_arr(&lines, &draw, 5.);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
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

    let num_points = 30;

    let mut arr = Vec::new();

    let rot = a + TAU / 4.;
    let rlen = 1.;
    let norm = pt2(rot.cos() / rlen, rot.sin() / rlen);
    let mut x0 = x + lenght / 2. * a.cos();
    let mut x1 = x + -lenght / 2. * a.cos();
    let mut y0 = y + lenght / 2. * a.sin();
    let mut y1 = y + -lenght / 2. * a.sin();
    if let Some((start, end)) = line_clipped(x0, y0, x1, y1, x - width / 2., y - height / 2., width, height) {
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
        if let Some((start, end)) = line_clipped(x0, y0, x1, y1, x - width / 2., y - height / 2., width, height) {
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
    for line in arr {
        let num_points = line.len();
        let tris = line
            .windows(2)
            // .iter()
            .flat_map(|slice| {
                let dev = (slice[1] - slice[0]).angle() + TAU / 4.;
                let dev = pt2(weight * dev.cos(), weight * dev.sin());
                let a = slice[0] + dev;
                let b = slice[0] - dev;
                let c = slice[1] + dev;
                let d = slice[1] - dev;
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
