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

use itertools::repeat_n;
use rand::random;
use phase::Phase;

struct Wuse
{
    t:      usize,
    center: Vector,
    amp:    Vector,
    angle:  Phase,
    lines:  Vec<Line>,
    colors: Vec<Color>,
}

struct Ampl
{
    span: Vector,
    min: Vector,
    x: i32,
    y: i32,
}

impl Ampl
{
    fn new(xrng: i32, yrng: i32) -> Self
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

    fn new_x(&mut self)
    {
        self.x = self.min.x as i32 + rand(self.span.x as u32) as i32;
    }

    fn new_y(&mut self)
    {
        self.x = self.min.y as i32 + rand(self.span.y as u32) as i32;
    }
}

pub(crate) fn rand(num: u32) -> u32
{
    random::<u32>() % num
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
            amp:    Vector::new(300,300),
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
