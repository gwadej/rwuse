// Copyright 2019 G. Wade Johnson

use quicksilver::{
    Result,
    geom::{Line,Transform},
    graphics::{Background::Col, Color},
    lifecycle::{Event, State, Window},
};

use std::collections::VecDeque;
use itertools::repeat_n;

use crate::phase::Phase;
use crate::ampl::Ampl;
use crate::coord::Coord;
use crate::color_scheme::{Scheme,get_colors};

const ANGLE_X: f32 = 1.345;
const ANGLE_Y: f32 = 0.0;

pub struct Wuse
{
    line_width: f32,
    t:          usize,
    start:      bool,
    center:     Coord,
    amp:        Ampl,
    angle:      Phase,
    lines:      VecDeque<Line>,
    colors:     Vec<Color>,
}

impl Wuse
{
    //pub fn sized(num_lines: usize, max_x: i32, max_y: i32, line_width: f32) -> Result<Self>
    pub fn sized(scheme: Scheme, max_x: i32, max_y: i32, line_width: f32) -> Result<Self>
    {
        let colors: Vec<Color> = get_colors(scheme);
//        let dupes = num_lines / colors.len();
        let dupes = 4;
        let num_lines = dupes * colors.len();
        let colors = colors.iter().flat_map(|c| repeat_n(c.clone(), dupes)).collect();
        let mut wuse = Wuse{
            line_width,
            t:          num_lines,
            start:      true,
            center:     Coord::new(max_x/2,max_y/2),
            amp:        Ampl::new(max_x/2,max_y/2),
            angle:      Phase::new(ANGLE_X, ANGLE_Y),
            lines:      VecDeque::new(),
            colors:     colors
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
        if self.t % self.lines.len() == 0 { self.start = !self.start; }

        let pt = self.new_pt( self.t );

        let mut line = self.lines.pop_back().unwrap();
        if self.start { line.a = pt.f() } else { line.b = pt.f() }
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
        // These values were chosen through trial and error
        if self.t % 351 == 0 { self.angle.new_y( self.t ) }
        if self.t % 571 == 0 { self.angle.new_x( self.t ) }
    }
}

impl State for Wuse
{
    fn new() -> Result<Wuse>
    {
        Wuse::sized( Scheme::Valen, 100, 100, 1.0 )
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
                &ln.with_thickness(self.line_width),
                Col(*clr),
                Transform::IDENTITY,
                5
            );
        }

        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()>
    {
        if let Event::Typed(c) = event
        {
            match c
            {
                'q' => window.close(),
                'r' => window.set_update_rate(25.0),
                '+' => window.set_update_rate(speed_up(window.update_rate())),
                '-' => window.set_update_rate(slow_down(window.update_rate())),
                _ => (),
            }
        }
        Ok(())
    }
}

fn speed_up(rate: f64) -> f64
{
    limit(
            if rate > 500.0 { 500.0 }
        else if rate <= 0.05 { 0.02 }
        else                 { rate - step(rate) }
    )
}

fn slow_down(rate: f64) -> f64
{
    limit(
            if rate >= 500.0 { 500.0 }
        else if rate < 0.05   { 0.02 }
        else                  { rate + step(rate) }
    )
}

// Pseudo-logarithmic scale for speed steps
fn step(rate: f64) -> f64
{
         if rate > 100.0 { 50.0 }
    else if rate > 10.0  { 10.0 }
    else if rate > 1.0   { 1.0 }
    else if rate > 0.2   { 0.1 }
    else { 0.01 }
}

fn limit(rate: f64) -> f64
{
    if rate > 500.0 { 500.0 }
    else if rate < 0.05 { 0.05 }
    else { rate }
}
