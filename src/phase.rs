use std::f32::consts::PI;
use quicksilver::geom::Vector;
use crate::frand;

const TAU_X: f32       = 26.0;
const TAU_X_RANGE: f32 = 124.0;
const TAU_X_HIGH: f32  = 150.0;

const TAU_Y: f32       = 50.0;
const TAU_Y_RANGE: f32 = 30.0;
const TAU_Y_HIGH: f32  = 80.0;

const BASE_DTAU_P: f32 = 5.0;
const BASE_DTAU_N: f32 = -25.0;
const DTAU_RANGE: f32  = 21.0;

pub struct Phase
{
    theta: Vector,
    tau: Vector,
    dtau: Vector,
    a: f32,
    b: f32,
}

impl Phase
{
    pub fn new(x: f32, y: f32) -> Self
    {
        Phase {
            theta: Vector::new(x, y),
            tau:   Vector::new(TAU_X + frand(TAU_X_RANGE), TAU_Y + frand(TAU_Y_RANGE)),
            dtau:  Vector::new(BASE_DTAU_P, BASE_DTAU_P),
            a:     BASE_DTAU_P,
            b:     BASE_DTAU_P,
        }
    }

    pub fn new_x(&mut self, t: usize)
    {
        self.theta.x += (t as f32 *  PI/self.tau.x)/(self.tau.x/self.dtau.x + 1.0);
        self.tau.x   += self.dtau.x;

        if self.tau.x > TAU_X_HIGH { self.a = BASE_DTAU_N }
        if self.tau.x < TAU_X      { self.a = BASE_DTAU_P }

        self.dtau.x = self.a + frand(DTAU_RANGE);
    }

    pub fn new_y(&mut self, t: usize)
    {
        self.theta.y += (t as f32 *  PI/self.tau.y)/(self.tau.y/self.dtau.y + 1.0);
        self.tau.y   += self.dtau.y;

        if self.tau.y > TAU_Y_HIGH { self.b = BASE_DTAU_N }
        if self.tau.y < TAU_Y      { self.b = BASE_DTAU_P }

        self.dtau.y = self.b + frand(DTAU_RANGE);
    }

    pub fn x(&self, t: usize) -> f32
    {
        t as f32 * PI/self.tau.x+self.theta.x
    }

    pub fn y(&self, t: usize) -> f32
    {
        t as f32 * PI/self.tau.y+self.theta.y
    }
}
