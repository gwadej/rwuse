use quicksilver::geom::Vector;
use crate::rand;

pub struct Ampl
{
    span: Vector,
    min: Vector,
    pub x: i32,
    pub y: i32,
}

impl Ampl
{
    pub fn new(xrng: i32, yrng: i32) -> Self
    {
        let span = Vector::new(7*xrng/10, 7*yrng/10);
        let mut this = Ampl {
            span,
            min: Vector::new(span.x as i32 / 3, span.y as i32 / 3),
            x: 0,
            y: 0,
        };
        this.new_x();
        this.new_y();
        this
    }

    pub fn new_x(&mut self)
    {
        self.x = self.min.x as i32 + rand(self.span.x as u32) as i32;
    }

    pub fn new_y(&mut self)
    {
        self.x = self.min.y as i32 + rand(self.span.y as u32) as i32;
    }
}
