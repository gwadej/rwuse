use quicksilver::{
    Result,
    geom::{Line,Transform},
    graphics::{Background::Col, Color},
    lifecycle::{State, Window},
};

use std::collections::VecDeque;
use itertools::repeat_n;

use crate::phase::Phase;
use crate::ampl::Ampl;
use crate::coord::Coord;

pub struct Wuse
{
    t:      usize,
    end:    i32,
    center: Coord,
    amp:    Ampl,
    angle:  Phase,
    lines:  VecDeque<Line>,
    colors: Vec<Color>,
}

impl Wuse
{
    pub fn sized(num_lines: usize, max_x: i32, max_y: i32) -> Result<Self>
    {
        let silver: Color = Color::from_hex("C0C0C0");
        let gold: Color   = Color::from_hex("FFD700");
        let brown: Color  = Color::from_hex("A52A2A");

        let colors: Vec<Color>
            = vec![Color::WHITE, Color::ORANGE, Color::YELLOW,
                    Color::MAGENTA, Color::RED, Color::BLUE,
                    Color::GREEN, Color::PURPLE, gold,
                    Color::CYAN, silver, brown
                ];
        let dupes = num_lines / colors.len();
        let colors = colors.iter().flat_map(|c| repeat_n(c.clone(), dupes)).collect();
        let mut wuse = Wuse{
            t:      num_lines,
            end:    1,
            center: Coord::new(max_x/2,max_y/2),
            amp:    Ampl::new(max_x/2,max_y/2),
            angle:  Phase::new(1.345, 0.0),
            lines:  VecDeque::new(),
            colors: colors
        };
        wuse.lines = wuse.initialize(num_lines);
        Ok(wuse)
    }

    fn new_pt(&self, t: usize) -> Coord
    {
        Coord::new(
            self.center.x + (self.amp.fx() * self.angle.x(t).sin()).floor() as i32,
            self.center.y + (self.amp.fy() * self.angle.y(t).cos()).floor() as i32,
        )
    }
    fn create_line(&self, t: usize) -> (Coord, Coord)
    {
        let pt = self.new_pt(t);
        (pt.clone(), pt)
    }

    fn initialize(&self, num_lines: usize) -> VecDeque<Line>
    {
        (0..num_lines).into_iter()
                .map(|t| self.create_line(t))
                .map(|(p1, p2)| Line::new(p1.f(), p2.f()))
                .collect()
    }

    fn next_line(&mut self)
    {
        if self.t % self.lines.len() == 0 { self.end = 3 - self.end; }
        let mut line = self.lines.pop_back().unwrap();

        let pt = self.new_pt( self.t );
        if self.end == 1 { line.a = pt.f() } else { line.b = pt.f() }
        self.lines.push_front(line);

        self.rescale(&pt);
        self.phase_change();

        self.t += 1;
    }

    fn rescale(&mut self, pt: &Coord)
    {
        if pt.x == self.center.x { self.amp.new_x() }
        if pt.y == self.center.y { self.amp.new_y() }
    }

    fn phase_change(&mut self)
    {
        if self.t % 351 == 0 { self.angle.new_y( self.t ) }
        if self.t % 571 == 0 { self.angle.new_x( self.t ) }
    }
}

impl State for Wuse
{
    fn new() -> Result<Wuse>
    {
        Wuse::sized( 12, 100, 100 )
    }

    fn update(&mut self, _window: &mut Window) -> Result<()>
    {
        self.next_line();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()>
    {
        window.clear(Color::BLACK)?;

        for (ln, clr) in self.lines.iter().zip(self.colors.iter())
        {
            window.draw_ex(
                &ln.with_thickness(1.0),
                Col(*clr),
                Transform::IDENTITY,
                5
            );
        }

        Ok(())
    }
}
