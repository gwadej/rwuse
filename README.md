# RWuse

## Synopsis

Wuse is a relatively simple piece of eye-candy I ported to Rust to see how it
would work. The original was written in the late eighties in Borland C. The
color scheme for the current version was chosen by my son in 2003. Enjoy.

In the late eighties, I decided to make my first foray into graphics
programming. I'm not much of a graphics person, but I had seen a few programs
that used randomly varying patterns on the screen to generate interesting
effects. One of these programs was called fuse (I think), and it used a set of
lines bouncing off the edges of the screen as the basis of its effect.

I decided to make my own version that circled around the center of the screen.
I named it wuse since it was a variation on fuse. With input from a few other
engineering types, I settled on the current design. It randomly modifies the
parameters to a couple of trigonometric functions to determine the endpoints of
the lines. The transition points were chosen to avoid discontinuities. We tried
several variations, but this was the one I kept around.

This program actually serves no purpose and doesn't really demonstrate any
interesting programming principles. I have ported it to a number of languages,
with minor cleanups along the way. Most of the bad habits from Wade-80s have
been cleaned up over time, but I would not describe this as my best work.
On the other hand, it continues to be amusing.

## Command Line

The Rust version supports a few interesting features not present in some earlier
versions.

    rwuse [FLAGS] [OPTIONS]

FLAGS:

    -f               Display in fullscreen mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:

        --geom <w> <h>    Supply the display width and height as two integers.
        --rate <rate>     Specify milliseconds between update calls. Defaults to 30. [default: 25.0]
    -t <thickness>        Specify the line thickness as a floating number. [default: 1.0]

Control the display with keys:

    `q`:   quit
    `+`:   run faster
    `-`:   run slower
    `r`:   reset speed

## Non-Obvious Effects

The speed of the lines is limited at the top and bottom when using the +/-
keys. Those limits don't apply to the command line. If you make the rate value
too low, the display will never update. If you make the rate value too large,
it just takes a long time.

Because of the line-drawing algorithm, there can be gaps in the lines at certain
angles. This generates interesting transient patterns. You can use the thickness
parameter to make these less noticeable by choosing a number between 1 and about 3.

Thicknesses below 1 increases the effect, until the lines start having permanent
gaps. At 0.02, the effect is more like a cloud of multi-color dots.

Larger thicknesses generate an effect that looks somewhat like a series of planks
rotating through space. A value around 25.0 shows this effect well.

----

Copyright 2019 G. Wade Johnson
