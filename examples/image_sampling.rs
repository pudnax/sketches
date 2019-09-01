extern crate image;
use image::GenericImageView;

use nannou::prelude::*;

const FACTOR: f64 = 3.;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    poise: PoissonDisk,
}

fn model(app: &App) -> Model {
    let image = image::open("in.png").unwrap();
    let (w, h) = image.dimensions();

    let _window = app
        .new_window()
        .with_dimensions(w, h)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    Model {
        _window,
        poise: PoissonDisk::new(4, 10, image),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for _ in 0..50 {
        model.poise.tick();
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
    draw.background().color(BLACK);
    let win = app.window_rect();
    // Draw a red ellipse with default size and position.
    for p in &model.poise.samples {
        draw.ellipse()
            .x_y(p.0 as f32 - win.w() / 2., p.1 as f32 - win.h() / 2.)
            .color(WHITE)
            .w_h(2., 2.);
    }
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

pub struct Point(pub usize, pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    // No object at this location
    EMPTY = 0,
    // No longer on the active list
    DEAD = 1,
    // In the active list (used to generate additional points)
    ACTIVE = 2,
}

pub struct PoissonDisk {
    image: image::DynamicImage,
    width: u32,
    height: u32,
    cell_size: f64,
    cell_width: f64,
    cell_height: f64,
    cells: Vec<Cell>,
    radius: u32,
    num_samples: u32,
    // Grid used to determine point sampling.
    grid: Vec<Option<(usize, usize)>>,
    // List of points we want to generate more points around.
    active: Vec<(usize, usize)>,
    samples: Vec<Point>,
}

impl PoissonDisk {
    pub fn new(radius: u32, num_samples: u32, image: image::DynamicImage) -> Self {
        let (width, height) = image.dimensions();

        let cells = vec![Cell::EMPTY; (width * height) as usize];

        // Step 0
        // Initialize an n-dimensional background grid for storing samples
        let dim = 2.;

        // We choose cell size to be radius / (dimensions) so that we
        // are guaranteed to have at most one point in any given cell.
        let cell_size = radius as f64 / (dim as f64).sqrt();
        let cell_width = (width as f64 / cell_size).ceil() + 1.0;
        let cell_height = (height as f64 / cell_size).ceil() + 1.0;
        let grid = vec![None; (cell_width * cell_height) as usize];

        let mut disk = PoissonDisk {
            image,
            width,
            height,
            cells,
            cell_size,
            cell_width,
            cell_height,
            grid,
            radius,
            num_samples,
            active: Vec::new(),
            samples: Vec::new(),
        };

        // Step 1
        // Select the initial sample to be randomly chosen uniformly in the domain.
        let point = (
            (random_range(0, width) as f64) as usize,
            (random_range(0, height) as f64) as usize,
        );

        disk.insert_point(point);
        disk.active.push(point);

        disk
    }

    fn distance(&self, pa: (usize, usize), pb: (usize, usize)) -> f64 {
        let dx = pa.0 as f64 - pb.0 as f64;
        let dy = pa.1 as f64 - pb.1 as f64;

        (dx * dx + dy * dy).sqrt()
    }

    fn is_valid(&self, point: (usize, usize)) -> bool {
        let xidx = (point.0 as f64 / self.cell_size).floor();
        let yidx = (point.1 as f64 / self.cell_size).floor();

        let start_x = (xidx - 2.0).max(0.0) as usize;
        let end_x = (xidx + 2.0).min(self.cell_width - 1.0) as usize;
        let start_y = (yidx - 2.0).max(0.0) as usize;
        let end_y = (yidx + 2.0).min(self.cell_height - 1.0) as usize;

        for x in start_x..end_x {
            for y in start_y..end_y {
                let cell_idx = y * self.cell_width as usize + x;
                if let Some(grid_point) = self.grid[cell_idx] {
                    let fraction = self
                        .image
                        .get_pixel(grid_point.0 as u32, self.height - 1 - grid_point.1 as u32)[0]
                        as f64
                        / 255.;
                    if self.distance(point, grid_point)
                        <= (self.radius as f64) * (fraction * FACTOR)
                    {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn insert_point(&mut self, point: (usize, usize)) {
        let cell_x = (point.0 as f64 / self.cell_size).floor();
        let cell_y = (point.1 as f64 / self.cell_size).floor();

        let idx = point.1 * self.width as usize + point.0;
        self.cells[idx] = Cell::ACTIVE;

        let cell_idx = (cell_y * self.cell_width + cell_x) as usize;
        self.grid[cell_idx] = Some(point);
    }

    fn new_point(&mut self, point: (usize, usize)) -> (usize, usize) {
        let theta = TAU_F64 * random_f64();
        // Pick a random radius between `r` and `2r`
        let fraction =
            self.image
                .get_pixel(point.0 as u32, self.height - 1 - point.1 as u32)[0] as f64
                / 255.;
        let new_radius =
            self.radius as f64 * (random_f64() * (fraction * FACTOR) + fraction * FACTOR);
        // Find new coordinates relative to point p.
        let new_x = point.0 as f64 + new_radius * theta.cos();
        let new_y = point.1 as f64 + new_radius * theta.sin();

        (
            new_x.max(0.0).min(self.width as f64 - 1.0) as usize,
            new_y.max(0.0).min(self.height as f64 - 1.0) as usize,
        )
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn num_points(&self) -> usize {
        self.samples.len()
    }

    pub fn point_at_idx(&self, idx: usize) -> Point {
        let point = &self.samples[idx];
        Point(point.0, point.1)
    }

    pub fn reset(&mut self) {
        self.active.clear();
        self.grid.clear();
        self.samples.clear();

        let point = (
            (random_range(0, self.width) as f64) as usize,
            (random_range(0, self.height) as f64) as usize,
        );

        self.insert_point(point);
        self.active.push(point);
    }

    pub fn tick(&mut self) -> bool {
        // While the active list is not empty, choose a random index.
        if self.active.is_empty() {
            return false;
        }

        // Choose a point randomly from active list
        let idx = (random_f64() * (self.active.len() - 1) as f64) as usize;
        let point = self.active[idx];

        // Generate up to `k` points chosen uniformly from the spherical
        // annulus between radius `r` and `2r` around `x_{i}`.
        let mut found = false;
        for _ in 0..self.num_samples {
            let new_point = self.new_point(point);
            // Add the new point to the grid, active list, and to the
            // final grid.
            if self.is_valid(new_point) {
                self.insert_point(new_point);
                self.active.push(new_point);
                self.samples.push(Point(new_point.0, new_point.1));
                found = true;
            }
        }

        if !found {
            self.active.remove(idx);
            let cidx = point.1 * self.width as usize + point.0;
            self.cells[cidx] = Cell::DEAD;
        }

        true
    }
}
