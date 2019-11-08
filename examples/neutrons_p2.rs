extern crate rand;

use rand::prelude::*;

const R_L: f64 = 0.1;

const N: usize = 100;

fn main() {
    let mut rng = rand::thread_rng();

    let mut vel_dist = |x: f64| (-(1. - x).ln()).sqrt();

    let vx = vel_dist(rng.gen());
    let vy = vel_dist(rng.gen());

    let dx = R_L * (vx / vy);

    let phi: f64 = 2. * std::f64::consts::PI * rng.gen::<f64>();
    let ro: f64 = vel_dist(rng.gen());

    let vx_n = ro * phi.sin();
    let vz_n = ro * phi.cos();

    let sin_a = vz_n / (vx_n * vx_n + vy * vy);

    let dx_n = R_L * (vx_n / vy);

    let x0 = 0.;
    let xrange = (0..N).map(|i| x0 + dx * (i as f64)).collect::<Vec<_>>();

    println!("{:?}", &vel_dist(rng.gen()));

    println!("{} {}", vx, vy);

    println!("{:?}", gen_dist().take(10).collect::<Vec<f64>>());
}

fn gen_dist() -> impl Iterator<Item = f64> {
    let mut rng = rand::thread_rng();

    let vel_dist = |x: f64| (-(1. - x).ln()).sqrt();

    (0..).map(move |_| vel_dist(rng.gen::<f64>()))
}
