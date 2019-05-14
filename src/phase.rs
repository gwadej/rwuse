use std::f32::consts::PI;

use quicksilver::geom::Vector;
use crate::frand;

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
            tau:   Vector::new(26.0 + frand(124), 50.0+frand(30)),
            dtau:  Vector::new(5, 5),
            a:     5.0,
            b:     5.0,
        }
    }

    pub fn new_x(&mut self, t: usize)
    {
        self.theta.x += (t as f32 *  PI/self.tau.x)/(self.tau.x/self.dtau.x + 1.0);
        self.tau.x   += self.dtau.x;
        if self.tau.x > 150.0 { self.a = -25.0 }
        if self.tau.x < 26.0  { self.a = 5.0 }
        self.dtau.x = self.a + frand(21);
    }

    pub fn new_y(&mut self, t: usize)
    {
        self.theta.y += (t as f32 *  PI/self.tau.y)/(self.tau.y/self.dtau.y + 1.0);
        self.tau.y   += self.dtau.y;
        if self.tau.y > 80.0 { self.b = -25.0 }
        if self.tau.y < 50.0  { self.b = 5.0 }
        self.dtau.y = self.b + frand(21);
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
