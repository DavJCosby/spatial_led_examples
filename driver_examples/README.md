# Calibration
Useful for setting up your LED system for the first time.

* Vertices are colored in white.
* The LED(s) due North (+Y) of your center point are colored green.
* Due South (-Y) is a muted green.
* Due East (+X) is red.
* Due West (-X) is a muted red.
```shell
cargo run --example calibration
```
[![asciicast](https://asciinema.org/a/683178.svg)](https://asciinema.org/a/683178)

# Ripples
Simulates growing rings of color at random points in your room.
```shell
cargo run --example ripples
```
[![asciicast](https://asciinema.org/a/683147.svg)](https://asciinema.org/a/683147)

# Warpspeed
Simulates stars zooming past you, almost like you're traveling in a space ship at light speed. All directions are relative to the center point declared in your config file.
```shell
cargo run --example warpspeed
```
[![asciicast](https://asciinema.org/a/683128.svg)](https://asciinema.org/a/683128)

# Scan
Simulates a straight line sweeping through the room at random angles, whose color gradually progresses through the rainbow over time.
```scan
cargo run --example scan
```
[![asciicast](https://asciinema.org/a/683169.svg)](https://asciinema.org/a/683169)

# Comet
Mostly just used as a benchmark as it requires a *ton* of `set_at_angle()` calls. Looks vaguely like comet orbiting the room.
```comet
cargo run --example comet
```
[![asciicast](https://asciinema.org/a/683189.svg)](https://asciinema.org/a/683189)

# Embers
Animates a rigid multi-fractal noise pattern across the room, mapping the noise output to a color ramp. Credit to [Inkpendude](https://twitter.com/inkpendude) for the [Midnight Ablaze](https://lospec.com/palette-list/midnight-ablaze) color palette used by default for this effect.

[![asciicast](https://asciinema.org/a/693835.svg)](https://asciinema.org/a/693835)