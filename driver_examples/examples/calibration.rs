mod tui;
use palette::rgb::Rgb;
use spatial_led::{Sled, SledError, Vec2};
use tui::SledTerminalDisplay;

fn main() -> Result<(), SledError> {
    let mut sled = Sled::<Rgb>::new("./complex_room.yap")?;

    let mut display = SledTerminalDisplay::start("Calibration", sled.domain());

    sled.set_all(Rgb::new(0.1, 0.1, 0.1));
    sled.set_vertices(Rgb::new(0.75, 0.75, 0.75));
    sled.set_at_dir(Vec2::new(1.0, 0.0), Rgb::new(1.0, 0.0, 0.0));
    sled.set_at_dir(Vec2::new(-1.0, 0.0), Rgb::new(0.5, 0.0, 0.0));
    sled.set_at_dir(Vec2::new(0.0, 1.0), Rgb::new(0.0, 1.0, 0.0));
    sled.set_at_dir(Vec2::new(0.0, -1.0), Rgb::new(0.0, 0.5, 0.0));

    display.set_leds(sled.colors_and_positions());
    display.refresh().unwrap();
    Ok(())
}
