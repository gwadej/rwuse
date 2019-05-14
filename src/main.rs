extern crate quicksilver;
extern crate itertools;
extern crate rand;

const NUM_LINES: usize = 48;
const MAX_X: i32 = 800;
const MAX_Y: i32 = 600;

use quicksilver::{
    geom::Vector,
    lifecycle::{Settings, run_with},
};

mod ampl;
mod coord;
mod phase;
mod wuse;

use rand::random;
use wuse::Wuse;

pub(crate) fn rand(num: i32) -> i32
{
    random::<i32>() % num
}

pub(crate) fn frand(num: u32) -> f32
{
    (random::<u32>() % num) as f32
}

fn main()
{
    let (max_x, max_y) = (MAX_X, MAX_Y);
    run_with(
        "Wuse",
        Vector::new(max_x, max_y),
        Settings::default(),
        || Wuse::sized(NUM_LINES, max_x, max_y)
    );
}
