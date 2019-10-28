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

    let n = 100;
    let points = (0..n)
        .map(|x| map_range(x as f32, 0., n as f32, -w / 2., w / 2.))
        .collect::<Vec<_>>();

    let points = points.iter().map(|&x| Collatz::new(x).collect::<Vec<_>>());

    for series in points {
        let mut y = h / 2.;
        for p in series {
            draw.ellipse().x_y(p, y).w_h(5., 5.);
            y -= 10.;
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn collatz(x: &f32) -> f32 {
    if x % 2. == 0. {
        x / 2.
    } else {
        (3. * x + 1.) / 2.
    }
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
