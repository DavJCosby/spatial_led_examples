use spatial_led::{
    color::{chromatic_adaptation::AdaptInto, Mix, Oklab, Srgb},
    driver::{BufferContainer, Driver, TimeInfo},
    driver_macros::*,
    scheduler::Scheduler,
    Sled, SledResult, Vec2,
};

use rand::{rngs::ThreadRng, Rng};

use noise::{MultiFractal, NoiseFn, Perlin, RidgedMulti};

mod tui;
use tui::SledTerminalDisplay;

fn main() {
    let sled = Sled::new("./complex_room.yap").unwrap();
    let mut display = SledTerminalDisplay::start("Embers", sled.domain());
    let mut driver = build_driver();
    driver.mount(sled);

    let mut scheduler = Scheduler::new(500.0);
    scheduler.loop_until_err(|| {
        driver.step();
        display.set_leds(driver.colors_and_positions_coerced());
        display.refresh()?;
        Ok(())
    });
}

pub fn build_driver() -> Driver {
    let mut driver = Driver::new();

    driver.set_startup_commands(startup);
    driver.set_draw_commands(draw);
    return driver;
}

#[startup_commands]
fn startup(buffers: &mut BufferContainer) -> SledResult {
    let colors = buffers.create_buffer::<(f32, Oklab)>("colors");

    // Credit to Inkpendude for the Midnight Ablaze Color Palette
    // https://lospec.com/palette-list/midnight-ablaze
    colors.extend([
        (0.0, Srgb::new(0.0745, 0.0078, 0.0313).adapt_into()),
        (1.0 / 7.0, Srgb::new(0.1215, 0.0196, 0.0627).adapt_into()),
        (2.0 / 7.0, Srgb::new(0.1922, 0.0196, 0.1176).adapt_into()),
        (3.0 / 7.0, Srgb::new(0.2745, 0.0549, 0.1686).adapt_into()),
        (4.0 / 7.0, Srgb::new(0.4863, 0.0941, 0.2353).adapt_into()),
        (5.0 / 7.0, Srgb::new(0.8353, 0.2353, 0.4157).adapt_into()),
        (6.0 / 7.0, Srgb::new(1.0, 0.5098, 0.4549).adapt_into()),
        (1.0, Srgb::new(1.0, 1.0, 1.0).adapt_into()),
    ]);

    let noise_controls = buffers.create_buffer::<f64>("noise_controls");
    noise_controls.extend([
        1.25, // noise size
        0.2,  // time scale
    ]);

    let move_vec = buffers.create_buffer::<Vec2>("move_vec");
    move_vec.push(Vec2::new(0.0, -0.2));

    let generator = buffers.create_buffer("generator");
    generator.push(
        RidgedMulti::<Perlin>::new(ThreadRng::default().gen_range(0..10_000))
            .set_octaves(4)
            .set_lacunarity(3.5)
            .set_frequency(0.285)
            .set_attenuation(1.37),
    );

    Ok(())
}

#[draw_commands]
fn draw(sled: &mut Sled, buffers: &BufferContainer, time_info: &TimeInfo) -> SledResult {
    let generator: &RidgedMulti<Perlin> = buffers.get_buffer_item("generator", 0)?;

    let noise_controls = buffers.get_buffer::<f64>("noise_controls")?;
    let colors = buffers.get_buffer::<(f32, Oklab)>("colors")?;
    let move_vec = buffers.get_buffer::<Vec2>("move_vec")?[0];

    let size = noise_controls[0];
    let time_scale = noise_controls[1];

    let elapsed_scaled = time_info.elapsed.as_secs_f64() * time_scale;
    sled.map_by_pos(|pos| {
        let t = generator.get([
            size * pos.x as f64 + move_vec.x as f64 * elapsed_scaled,
            size * pos.y as f64 + move_vec.y as f64 * elapsed_scaled,
            elapsed_scaled,
        ]) as f32;

        // go from [-1, 1] to [0, 1]
        let t = ((t + 1.0) * 0.5).clamp(0.0, 1.0);

        // discover which control points our noise output places us between
        let mut start = (0.0, Oklab::new(0.0, 0.0, 0.0));
        let mut end = (1.0, Oklab::new(0.0, 0.0, 0.0));
        for i in 0..colors.len() - 1 {
            let (t_0, c_0) = colors[i];
            let (t_1, c_1) = colors[i + 1];

            if t_0 <= t && t_1 >= t {
                start = (t_0, c_0);
                end = (t_1, c_1);
                break;
            }
        }

        // find a percentage mix between the two control points
        let t_scaled = ((t - start.0) / (end.0 - start.0)).clamp(0.0, 1.0);
        start.1.mix(end.1, t_scaled).adapt_into()
    });
    Ok(())
}
