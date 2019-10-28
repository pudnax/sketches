// use nannou::prelude::{App, Frame, BLACK, PLUM, WHITE};
use nannou::prelude::*;
// use std::convert::From::from;
// use num_traits::cast::NumCast::from;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    // Prepare to draw.
    let draw = app.draw();
    draw.background().color(BLACK);

    let (w, h) = app.window_rect().w_h();

    let frac = map_range(app.mouse.x, -w / 2., w / 2., 0., 1.);

    let n = 11000;
    let points = 10000..n;

    let points = points.map(|x| {
        Collatz::new(x as f32)
            .collect::<Vec<f32>>()
            .iter()
            .rev()
            .map(|&x| x)
            .collect::<Vec<f32>>()
    });

    for series in points {
        let mut y = -h / 2.;
        let mut x = 0.;
        for p in series.windows(2) {
            let div = (p[0] - p[1]).abs() / p[0];
            let y0 = y;
            let y1 = y + 8. * (div);
            let x0 = x;
            let step = 15. * div;
            // let step = map_range(app.mouse.x, -w / 2., w / 2., 1., 15.);
            let x1 = if p[1] % 2. == 0. { x - step } else { x + step };
            draw.line().points(pt2(x0, y0), pt2(x1, y1));
            y = y1;
            x = x1;
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

struct Collatz {
    count: f32,
}

impl Collatz {
    fn new(x: f32) -> Collatz {
        Collatz { count: x }
    }
}

impl Iterator for Collatz {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 1. {
            Some(if self.count % 2. == 0. {
                self.count /= 2.;
                self.count
            } else {
                self.count = (3. * self.count + 1.) / 2.;
                self.count
            })
        } else {
            None
        }
    }
}
