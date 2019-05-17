// Copyright 2019 G. Wade Johnson

use crate::rand;
use crate::coord::Coord;

pub struct Ampl
{
    span: Coord,
    min: Coord,
    x: i32,
    y: i32,
}

impl Ampl
{
    pub fn new(xrng: i32, yrng: i32) -> Self
    {
        let span = Coord::new(7*xrng/10, 7*yrng/10);
        let min  = Coord::new(span.x/3, span.y/3);
        let mut this = Ampl {
            span,
            min,
            x: 0,
            y: 0,
        };
        this.new_x();
        this.new_y();
        this
    }

    pub fn new_x(&mut self)
    {
        self.x = self.min.x + rand(self.span.x);
    }

    pub fn new_y(&mut self)
    {
        self.y = self.min.y + rand(self.span.y);
    }

    pub fn fx(&self) -> f32
    {
        self.x as f32
    }

    pub fn fy(&self) -> f32
    {
        self.y as f32
    }
}
