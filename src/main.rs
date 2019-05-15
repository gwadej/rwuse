extern crate quicksilver;
extern crate itertools;
extern crate rand;
#[macro_use]
extern crate clap;

const NUM_LINES: usize = 48;
const MAX_X: &str = "800";
const MAX_Y: &str = "600";

use quicksilver::{
    geom::Vector,
    lifecycle::{Settings, run_with},
};

use clap::{App,Arg,ArgMatches};

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

fn validate_size(val: &str) -> Result<(), &'static str>
{
    match val.parse::<i32>()
    {
        Ok(val) if val > 0 => Ok(()),
        Ok(val)  => Err("Not a positive integer"),
        _ => Err("Not a valid integer"),
    }
}

fn main()
{
    let args = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about("")
                .arg(Arg::with_name("width")
                     .short("w")
                     .help("Declare the width of the display")
                     .default_value(MAX_X))
                .arg(Arg::with_name("height")
                     .short("h")
                     .help("Declare the height of the display")
                     .default_value(MAX_Y))
                .get_matches();

    let max_x = args.value_of("width").unwrap().parse::<i32>().expect("Width is not a valid integer");
    let max_y = args.value_of("height").unwrap().parse::<i32>().expect("Height is not a valid integer");
    run_with(
        "Wuse",
        Vector::new(max_x, max_y),
        Settings::default(),
        || Wuse::sized(NUM_LINES, max_x, max_y)
    );
}
