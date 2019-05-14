extern crate quicksilver;
extern crate itertools;
extern crate rand;

const NUM_LINES: usize = 48;

use quicksilver::{
    Result,
    geom::{Line,Transform,Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
};

mod phase;
mod ampl;

use itertools::repeat_n;
use rand::random;
use phase::Phase;
use ampl::Ampl;

struct Wuse
{
    t:      usize,
    center: Vector,
    amp:    Ampl,
    angle:  Phase,
    lines:  Vec<Line>,
    colors: Vec<Color>,
}

pub(crate) fn rand(num: u32) -> u32
{
    random::<u32>() % num
}

impl Wuse
{
    fn new_pt(&self) -> Vector
    {
        Vector::new(
            self.center.x + (self.amp.x as f32 * self.angle.x(self.t).sin()).floor(),
            self.center.y + (self.amp.y as f32 * self.angle.y(self.t).cos()).floor(),
        )
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
        Ok(Wuse{
            t:      0,
            center: Vector::new(0,0),
            amp:    Ampl::new(300,300),
            angle:  Phase::new(1.345, 0.0),
            lines:  vec![],
            colors: colors
        })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()>
    {
        for c in &self.colors
        {
            println!("{:?}", c);
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()>
    {
        window.clear(Color::BLACK)?;

        for (ln, clr) in self.lines.iter().zip(self.colors.iter())
        {

        }

        Ok(())
    }
}

fn main()
{
    run::<Wuse>("Wuse", Vector::new(800, 600), Settings::default());
}
