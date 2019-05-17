// Copyright 2019 G. Wade Johnson

extern crate quicksilver;
extern crate itertools;
extern crate rand;
#[macro_use]
extern crate clap;

const NUM_LINES: usize = 48;
const WIDTH: i32       = 800;
const HEIGHT: i32      = 600;
const RATE_STR: &str   = "25.0";
const THICKNESS: &str  = "1.0";

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

pub(crate) fn frand(num: f32) -> f32
{
    (random::<f32>() * num) as f32
}

fn main()
{
    let args = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about("Wuse is a relatively simple piece of eye candy.")
                .after_help("Control the display with keys:\n\n  \
                q  - quit\n  \
                +  - run faster\n  \
                -  - run slower\n  \
                r  - reset speed\n")
                .arg(Arg::with_name("geom")
                     .long("geom")
                     .help("Supply the display width and height as two integers.")
                     .takes_value(true)
                     .value_names(&["w","h"])
                     .number_of_values(2))
                .arg(Arg::with_name("fullscreen")
                      .short("f")
                      .help("Display in fullscreen mode"))
                .arg(Arg::with_name("thickness")
                     .short("t")
                     .help("Specify the line thickness as a floating number.")
                     .default_value(THICKNESS)
                     .takes_value(true))
                .arg(Arg::with_name("rate")
                     .long("rate")
                     .help("Specify milliseconds between update calls. Defaults to 30.")
                     .default_value(RATE_STR)
                     .takes_value(true))
                .get_matches();

    let (width, height) = if args.is_present("geom")
    {
        let mut it = args.values_of("geom").unwrap();
        (
          opt_convert::<i32>(it.next(), "Width is not a valid integer"),
          opt_convert::<i32>(it.next(), "Height is not a valid integer"),
        )
    }
    else
    {
        ( WIDTH, HEIGHT )
    };

    let mut settings = Settings::default();

    if args.is_present("fullscreen") { settings.fullscreen = true; }
    settings.update_rate = opt_convert(args.value_of("rate"), "rate is not a valid floating point number");
    let thickness: f32 = opt_convert(args.value_of("thickness"), "Thickness is not a valid floating point number");

    run_with(
        "Wuse",
        Vector::new(width, height),
        settings,
        || Wuse::sized(NUM_LINES, width, height, thickness)
    );
}

fn opt_convert<T>(opt: Option<&str>, errmsg: &str) -> T
    where <T as std::str::FromStr>::Err : std::fmt::Debug,
            T: std::str::FromStr
{
    opt.unwrap().parse::<T>().expect(errmsg)
}
