extern crate quicksilver;
extern crate itertools;
extern crate rand;

const NUM_LINES: usize = 48;
const MAX_X: i32 = 800;
const MAX_Y: i32 = 600;

use quicksilver::{
    Result,
    geom::{Line,Transform,Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

mod coord;
mod phase;
mod ampl;

use std::collections::VecDeque;
use itertools::repeat_n;
use rand::random;
use phase::Phase;
use ampl::Ampl;
use coord::Coord;

struct Wuse
{
    t:      usize,
    end:    i32,
    center: Coord,
    amp:    Ampl,
    angle:  Phase,
    lines:  VecDeque<Line>,
    colors: Vec<Color>,
}

pub(crate) fn rand(num: i32) -> i32
{
    random::<i32>() % num
}

pub(crate) fn frand(num: u32) -> f32
{
    (random::<u32>() % num) as f32
}

impl Wuse
{
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

    fn initialize(&self) -> VecDeque<Line>
    {
        (0..NUM_LINES).into_iter()
                .map(|t| self.create_line(t))
                .map(|(p1, p2)| Line::new(p1.f(), p2.f()))
                .collect()
    }

    fn next_line(&mut self)
    {
        if self.t % NUM_LINES == 0 { self.end = 3 - self.end; }
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
        let silver: Color = Color::from_hex("C0C0C0");
        let gold: Color   = Color::from_hex("FFD700");
        let brown: Color  = Color::from_hex("A52A2A");

        let colors: Vec<Color>
            = vec![Color::WHITE, Color::ORANGE, Color::YELLOW,
                    Color::MAGENTA, Color::RED, Color::BLUE,
                    Color::GREEN, Color::PURPLE, gold,
                    Color::CYAN, silver, brown
                ];
        let dupes = NUM_LINES / colors.len();
        let colors = colors.iter().flat_map(|c| repeat_n(c.clone(), dupes)).collect();
        let mut wuse = Wuse{
            t:      NUM_LINES,
            end:    1,
            center: Coord::new(MAX_X/2,MAX_Y/2),
            amp:    Ampl::new(MAX_X/2,MAX_Y/2),
            angle:  Phase::new(1.345, 0.0),
            lines:  VecDeque::new(),
            colors: colors
        };
        wuse.lines = wuse.initialize();
        Ok(wuse)
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

fn main()
{
    run::<Wuse>("Wuse", Vector::new(MAX_X, MAX_Y), Settings::default());
}
