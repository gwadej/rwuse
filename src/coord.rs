// Copyright 2019 G. Wade Johnson

use quicksilver::geom::Vector;

#[derive(Clone)]
pub struct Coord
{
    pub x: i32,
    pub y: i32,
}

impl Coord
{
    pub fn new(x: i32, y: i32) -> Self
    {
        Coord { x, y }
    }

    pub fn f(&self) -> Vector
    {
        Vector::new(self.x, self.y)
    }
}


