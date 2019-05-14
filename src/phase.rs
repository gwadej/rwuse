use std::f32::consts::PI;

use quicksilver::{Result, geom::Vector};
use crate::rand;

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
            tau:   Vector::new(26.0 + rand(124) as f32, 50.0+rand(30) as f32),
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
        self.dtau.x = self.a + rand(21) as f32;
    }

    pub fn new_y(&mut self, t: usize)
    {
        self.theta.y += (t as f32 *  PI/self.tau.y)/(self.tau.y/self.dtau.y + 1.0);
        self.tau.y   += self.dtau.y;
        if self.tau.y > 80.0 { self.a = -25.0 }
        if self.tau.y < 50.0  { self.a = 5.0 }
        self.dtau.y = self.a + rand(21) as f32;
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
