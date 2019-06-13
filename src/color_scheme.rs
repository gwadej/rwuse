// Copyright 2019 G. Wade Johnnson

use quicksilver::graphics::Color;

pub enum Scheme
{
    Valen,
    Rainbow,
    Blues,
    Reds,
    Greens,
    Resistor,
    WarmCool,
    HueWheel,
}

pub const NAMES: [&str; 8] = [
    "default", "rainbow", "blues", "reds", "greens",
    "resistor", "warmcool", "huewheel"
];

impl Default for Scheme
{
    fn default() -> Self
    {
        Scheme::Valen
    }
}

impl Scheme
{
    pub fn get_colors(self) -> Vec<Color>
    {
        match self
        {
            Scheme::Valen    => make_valen_colors(),
            Scheme::Rainbow  => make_rainbow_colors(),
            Scheme::Blues    => make_blue_colors(),
            Scheme::Reds     => make_red_colors(),
            Scheme::Greens   => make_green_colors(),
            Scheme::Resistor => make_resistor_colors(),
            Scheme::WarmCool => make_warm_cool_colors(),
            Scheme::HueWheel => make_hue_wheel_colors(),
        }
    }

    pub fn from_name(name: &str) -> Result<Self, &str>
    {
        match name
        {
            "default"  => Ok(Scheme::Valen),
            "valen"    => Ok(Scheme::Valen),
            "rainbow"  => Ok(Scheme::Rainbow),
            "blues"    => Ok(Scheme::Blues),
            "reds"     => Ok(Scheme::Reds),
            "greens"   => Ok(Scheme::Greens),
            "resistor" => Ok(Scheme::Resistor),
            "warmcool" => Ok(Scheme::WarmCool),
            "huewheel" => Ok(Scheme::HueWheel),
            _          => Err("Unknown scheme name"),
        }
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

fn make_blue_colors() -> Vec<Color>
{
    vec![
        Color::from_hex("#8080ff"),
        Color::from_hex("#6e6fe3"),
        Color::from_hex("#5c5ec8"),
        Color::from_hex("#4b4ead"),
        Color::from_hex("#3a3f93"),
        Color::from_hex("#29307a"),
        Color::from_hex("#182261"),
        Color::from_hex("#06144a"),
        Color::from_hex("#000034"),
    ]
}

fn make_red_colors() -> Vec<Color>
{
    vec![
        Color::from_hex("#ff3030"),
        Color::from_hex("#e3262d"),
        Color::from_hex("#c81d2a"),
        Color::from_hex("#ad1526"),
        Color::from_hex("#930e21"),
        Color::from_hex("#79081c"),
        Color::from_hex("#600416"),
        Color::from_hex("#48020e"),
        Color::from_hex("#320000"),
    ]
}

fn make_green_colors() -> Vec<Color>
{
    vec![
        Color::from_hex("#00aa00"),
        Color::from_hex("#00950c"),
        Color::from_hex("#008112"),
        Color::from_hex("#006d14"),
        Color::from_hex("#025a14"),
        Color::from_hex("#054813"),
        Color::from_hex("#083610"),
        Color::from_hex("#09250a"),
        Color::from_hex("#001500"),
    ]
}
