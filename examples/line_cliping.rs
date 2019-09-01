use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait(1));
    let _window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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

fn view(app: &App, _model: &Model, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    // Clear the background to pink.
    draw.background().color(BLACK);
    // Draw a red ellipse with default size and position.

    let win = app.window_rect();
    let frac = 7;
    let w = win.w() / frac as f32;
    let h = win.h() / frac as f32;

    for xoff in (-frac..frac).map(|x| x as f32 * w) {
        for yoff in (-frac..frac).map(|x| x as f32 * h) {
            draw_square(xoff, yoff, w, random_range(5., 15.), TAU * random_f32(), &draw);
        }
    }
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn encode_endpoint(x: f32, y: f32, clipx: f32, clipy: f32, clipw: f32, cliph: f32) -> usize {
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

fn line_clipped(
    mut x0: f32,
    mut y0: f32,
    mut x1: f32,
    mut y1: f32,
    clipx: f32,
    clipy: f32,
    clipw: f32,
    cliph: f32,
    draw: &nannou::app::Draw,
) -> bool {
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
        draw.line().start(pt2(x0, y0)).end(pt2(x1, y1)).color(WHITE);
    }

    accept
}

fn draw_square(x: f32, y: f32, w: f32, step: f32, a: f32, draw: &nannou::app::Draw) {
    let xstart = x + random_range(0., w);
    let ystart = y + random_range(0., w);

    let slope = a.tan();
    let c = ystart - slope * xstart;

    let mut down_accept = true;
    let mut up_accept = true;

    let mut i = 0;

    //for (int i = 0; i < w / step; i++) {
    while down_accept || up_accept {
        let mut x0 = x - w / 2.;
        let mut y0 = slope * x0 + c + i as f32 * step / a.cos();
        let mut x1 = x + w + w / 2.;
        let mut y1 = slope * x1 + c + i as f32 * step / a.cos();;
        up_accept = line_clipped(x0, y0, x1, y1, x, y, w, w, draw);

        x0 = x - w / 2.;
        y0 = slope * x0 + c - i as f32 * step / a.cos();
        x1 = x + w + w / 2.;
        y1 = slope * x1 + c - i as f32 * step / a.cos();
        down_accept = line_clipped(x0, y0, x1, y1, x, y, w, w, draw);

        i += 1;
    }
}
