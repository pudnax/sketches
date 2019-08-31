#![allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Angle {
    val: f64,
    step: f64,
}

impl Angle {
    pub fn new(step: f64) -> Angle {
        Angle {
            val: 2. * std::f64::consts::PI,
            step,
        }
    }
}

impl Iterator for Angle {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        self.val -= self.step;
        if self.val < 0. {
            self.val = 2. * std::f64::consts::PI;
            None
        } else {
            Some(self.val)
        }
    }
}

extern crate num;
use num::{Float, FromPrimitive};

pub fn linspace<T>(start: T, stop: T, nstep: u32) -> Vec<T>
where
    T: Float + FromPrimitive,
{
    let delta: T = (stop - start) / T::from_u32(nstep - 1).expect("out of range");
    (0..(nstep))
        .map(|i| start + T::from_u32(i).expect("out of range") * delta)
        .collect()
}
