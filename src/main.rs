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

mod phase;
mod ampl;

use std::collections::VecDeque;
use itertools::repeat_n;
use rand::random;
use phase::Phase;
use ampl::Ampl;

struct Wuse
{
    t:      usize,
    end:    i32,
    center: Vector,
    amp:    Ampl,
    angle:  Phase,
    lines:  VecDeque<Line>,
    colors: Vec<Color>,
}

pub(crate) fn rand(num: u32) -> u32
{
    random::<u32>() % num
}

impl Wuse
{
    fn new_pt(&self, t: usize) -> Vector
    {
        Vector::new(
            self.center.x + (self.amp.x as f32 * self.angle.x(t).sin()).floor(),
            self.center.y + (self.amp.y as f32 * self.angle.y(t).cos()).floor(),
        )
    }
    fn create_line(&self, t: usize) -> (Vector, Vector)
    {
        let pt = self.new_pt(t);
        (pt.clone(), pt)
    }

    fn initialize(&self) -> VecDeque<Line>
    {
        (0..NUM_LINES).into_iter()
                .map(|t| self.create_line(t))
                .map(|(p1, p2)| Line::new(p1, p2))
                .collect()
    }

    fn next_line(&mut self)
    {
        if self.t % NUM_LINES == 0 { self.end = 3 - self.end; }
        let mut line = self.lines.pop_front().unwrap();

        let pt = self.new_pt( self.t );
        if self.end == 1 { line.a = pt } else { line.b = pt }
        self.lines.push_back(line);

        if pt.x == self.center.x { self.amp.new_x() }
        if pt.y == self.center.y { self.amp.new_y() }
        if self.t % 351 == 0 { self.angle.new_y( self.t ) }
        if self.t % 571 == 0 { self.angle.new_x( self.t ) }

        self.t += 1;
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
            center: Vector::new(MAX_X/2,MAX_Y/2),
            amp:    Ampl::new(300,300),
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
