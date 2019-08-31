use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    let max_distance = 500.;
    draw.background().color(PLUM);

    let win = app.window_rect();

    let w_sep = (win.w() / 25.) as i32;
    let h_sep = (win.h() / 25.) as i32;

    for xoff in (-w_sep..w_sep).map(|x| x as f32 * win.w() / 25.) {
        for yoff in (-h_sep..h_sep).map(|x| x as f32 * win.h() / 25.) {
            let mut diameter = pt2(app.mouse.x, app.mouse.y).distance(pt2(xoff, yoff));
            diameter = diameter / max_distance * 40.;
            draw.rect()
                .color(BLACK)
                .stroke(PLUM)
                .x_y(xoff, yoff)
                .w_h(diameter, diameter);
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
