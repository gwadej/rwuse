extern crate quicksilver;
extern crate itertools;
extern crate rand;
#[macro_use]
extern crate clap;

const NUM_LINES: usize = 48;
const MAX_X: i32 = 800;
const MAX_Y: i32 = 600;
const RATE_STR: &str = "30.0";

use quicksilver::{
    geom::Vector,
    lifecycle::{Settings, run_with},
};

use clap::{App,Arg};

mod ampl;
mod coord;
mod phase;
mod wuse;

use rand::random;
use wuse::Wuse;

pub(crate) fn rand(num: i32) -> i32
{
    random::<i32>() % num
}

pub(crate) fn frand(num: u32) -> f32
{
    (random::<u32>() % num) as f32
}

fn main()
{
    let args = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about("Display time waster.")
                .after_help("Control the display with keys:\n\n  \
                Q  - quit\n  \
                F  - run faster\n  \
                S  - run slower\n  \
                R  - reset speed\n")
                .arg(Arg::with_name("geom")
                     .long("geom")
                     .help("Supply the display width and height as two integers.")
                     .takes_value(true)
                     .value_names(&["w","h"])
                     .number_of_values(2))
                .arg(Arg::with_name("fullscreen")
                      .short("f")
                      .help("Display in fullscreen mode"))
                .arg(Arg::with_name("rate")
                     .long("rate")
                     .help("Specify milliseconds between update calls. Defaults to 30.")
                     .default_value(RATE_STR)
                     .takes_value(true))
                .get_matches();

    let (max_x, max_y) = if args.is_present("geom")
    {
        let mut it = args.values_of("geom").unwrap();
        (
          it.next().unwrap().parse::<i32>().expect("Width is not a valid integer"),
          it.next().unwrap().parse::<i32>().expect("Height is not a valid integer"),
        )
    }
    else
    {
        ( MAX_X, MAX_Y )
    };

    let mut settings = Settings::default();

    if args.is_present("fullscreen") { settings.fullscreen = true; }
    settings.update_rate = args.value_of("rate").unwrap().parse::<f64>().expect("rate is not a valid floating number");

    run_with(
        "Wuse",
        Vector::new(max_x, max_y),
        settings,
        || Wuse::sized(NUM_LINES, max_x, max_y)
    );
}
