use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    counter: usize,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    Model {
        _window,
        counter: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.counter += 1;
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
    draw.background().color(PLUM);
    // Draw a red ellipse with default size and position.
    let points = vec![
        pt2(-150., -150.),
        pt2(-50., 50.),
        pt2(-25., -25.),
        pt2(0., 0.),
        pt2(25., 25.),
        pt2(50., -50.),
        pt2(150., 150.),
    ];
    draw.polyline().weight(5.).points(
        chaikin_open(points, 0.05, model.counter / 50)
            .iter()
            .map(|&x| x + pt2(0., 0.)),
    );
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn chaikin_cut(a: Point2, b: Point2, mut ratio: f32) -> Vec<Point2> {
    let (mut x, mut y) = (0., 0.);
    let mut n = Vec::new();

    /*
     * If ratio is greater than 0.5 flip it so we avoid cutting across
     * the midpoint of the line.
     */
    if ratio > 0.5 {
        ratio = 1. - ratio;
    }
    /* Find point at a given ratio going from A to B */
    x = lerp(a.x, b.x, ratio);
    y = lerp(a.y, b.y, ratio);
    n.push(pt2(x, y));

    /* Find point at a given ratio going from B to A */
    x = lerp(b.x, a.x, ratio);
    y = lerp(b.y, a.y, ratio);
    n.push(pt2(x, y));

    n
}

fn chaikin(shape: Vec<Point2>, ratio: f32, iterations: usize, close: bool) -> Vec<Point2> {
    // If the number of iterations is zero, return shape as is
    if iterations == 0 {
        return shape;
    }

    let mut next = Vec::new();

    /*
     * Step 1: Figure out how many corners the shape has
     *         depending on whether it's open or closed.
     */
    let mut num_corners = shape.len();
    if !close {
        num_corners = shape.len() - 1;
    }

    /*
     * Step 2: Since we don't have access to edges directly
     *         with a PShape object, do a pairwise iteration
     *         over vertices instead. Same thing.
     */
    for i in 0..num_corners {
        // Get the i'th and (i+1)'th vertex to work on that edge.
        let a = shape[i];
        let b = shape[(i + 1) % shape.len()];

        // Step 3: Break it using our chaikin_break() function
        let n = chaikin_cut(a, b, ratio);

        /*
         * Now we have to deal with one corner case. In the case
         * of open shapes, the first and last endpoints shouldn't
         * be moved. However, in the case of closed shapes, we
         * cut all edges on both ends.
         */
        if !close && i == 0 {
            // For the first point of open shapes, ignore vertex A
            next.push(pt2(a.x, a.y));
            next.push(pt2(n[1].x, n[1].y));
        } else if !close && i == num_corners - 1 {
            // For the last point of open shapes, ignore vertex B
            next.push(pt2(n[0].x, n[0].y));
            next.push(pt2(b.x, b.y));
        } else {
            // For all other cases (i.e. interior edges of open
            // shapes or edges of closed shapes), add both vertices
            // returned by our chaikin_break() method
            next.push(pt2(n[0].x, n[0].y));
            next.push(pt2(n[1].x, n[1].y));
        }
    }

    chaikin(next, ratio, iterations - 1, close)
}

fn chaikin_close(original: Vec<Point2>, ratio: f32, iterations: usize) -> Vec<Point2> {
    chaikin(original, ratio, iterations, true)
}

fn chaikin_open(original: Vec<Point2>, ratio: f32, iterations: usize) -> Vec<Point2> {
    chaikin(original, ratio, iterations, false)
}

fn lerp(v0: f32, v1: f32, d: f32) -> f32 {
    v0 + (v1 - v0) * d.max(0.).min(1.)
}
