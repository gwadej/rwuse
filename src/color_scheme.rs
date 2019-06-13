// Copyright 2019 G. Wade Johnnson

use quicksilver::graphics::Color;

pub enum Scheme
{
    Valen,
    Rainbow,
    Resistor,
    WarmCool,
    HueWheel,
}

pub const NAMES: [&str; 5] = [ "default", "rainbow", "resistor", "warmcool", "huewheel" ];

impl From<&str> for Scheme
{
    fn from(name: &str) -> Self
    {
        match name
        {
            "default"  => Scheme::Valen,
            "valen"    => Scheme::Valen,
            "rainbow"  => Scheme::Rainbow,
            "resistor" => Scheme::Resistor,
            "warmcool" => Scheme::WarmCool,
            "huewheel" => Scheme::HueWheel,
            _          => panic!("Unknown color scheme"),
        }
    }
}

pub fn get_colors(s: Scheme) -> Vec<Color>
{
    match s
    {
        Scheme::Valen    => make_valen_colors(),
        Scheme::Rainbow  => make_rainbow_colors(),
        Scheme::Resistor => make_resistor_colors(),
        Scheme::WarmCool => make_warm_cool_colors(),
        Scheme::HueWheel => make_hue_wheel_colors(),
    }
}

fn make_valen_colors() -> Vec<Color>
{
    // Based on HTML/CSS named colors
    let silver: Color = Color::from_hex("C0C0C0");
    let gold: Color   = Color::from_hex("FFD700");
    let brown: Color  = Color::from_hex("A52A2A");

    vec![
        Color::WHITE, Color::ORANGE, Color::YELLOW,
        Color::MAGENTA, Color::RED, Color::BLUE,
        Color::GREEN, Color::PURPLE, gold,
        Color::CYAN, silver, brown
    ]
}

fn make_rainbow_colors() -> Vec<Color>
{
    vec![
        Color::from_hex("FF0000"), // red
        Color::from_hex("FFA500"), // orange
        Color::from_hex("AAAA00"), // yellow
        Color::from_hex("008000"), // green
        Color::from_hex("0000FF"), // blue
        Color::from_hex("4B0082"), // indigo
        Color::from_hex("330033")
    ]
}

fn make_resistor_colors() -> Vec<Color>
{
    vec![
        Color::from_hex("A52A2A"), // brown
        Color::from_hex("FF0000"), // red
        Color::from_hex("FFA500"), // orange
        Color::from_hex("AAAA00"), // yellow
        Color::from_hex("008000"), // green
        Color::from_hex("0000FF"), // blue
        Color::from_hex("663399"), // purple
        Color::from_hex("808080"), // slate
        Color::from_hex("FFFFFF"),
    ]
}

fn make_warm_cool_colors() -> Vec<Color>
{
    vec![
        Color::from_hex("ff0001"),
        Color::from_hex("ff5837"),
        Color::from_hex("ff8363"),
        Color::from_hex("ffa890"),
        Color::from_hex("fecbbe"),
        Color::from_hex("eeeeee"),
        Color::from_hex("c7bcd9"),
        Color::from_hex("a08bc3"),
        Color::from_hex("785dae"),
        Color::from_hex("4c3198"),
        Color::from_hex("000082")
    ]
}

fn make_hue_wheel_colors() -> Vec<Color>
{
    vec![
        Color::from_hex("FF0000"),
        Color::from_hex("FF8000"),
        Color::from_hex("FFFF00"),
        Color::from_hex("80FF00"),
        Color::from_hex("00FF00"),
        Color::from_hex("00FF80"),
        Color::from_hex("00FFFF"),
        Color::from_hex("0080FF"),
        Color::from_hex("0000FF"),
        Color::from_hex("8000FF"),
        Color::from_hex("FF00FF"),
        Color::from_hex("FF0080")
    ]
}
