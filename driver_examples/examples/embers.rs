use palette::{chromatic_adaptation::AdaptInto, rgb::Rgb, Mix, Oklab, Srgb};
use spatial_led::{
    driver::{Data, Driver, Time},
    scheduler::Scheduler,
    Sled, SledResult, Vec2,
};

use rand::{rngs::ThreadRng, Rng};

use noise::{MultiFractal, NoiseFn, Perlin, RidgedMulti};

mod tui;
use tui::SledTerminalDisplay;

#[derive(Debug)]
struct NoiseSettings {
    noise_size: f64,
    time_scale: f64,
}

fn main() {
    let sled = Sled::new("./complex_room.yap").unwrap();
    let mut display = SledTerminalDisplay::start("Embers", sled.domain());
    let mut driver = build_driver();
    driver.mount(sled);

    let mut scheduler = Scheduler::new(500.0);
    scheduler.loop_until_err(|| {
        driver.step();
        display.set_leds(driver.colors_and_positions());
        display.refresh()?;
        Ok(())
    });
}

pub fn build_driver() -> Driver<Rgb> {
    let mut driver = Driver::new();

    driver.set_startup_commands(startup);
    driver.set_draw_commands(draw);
    return driver;
}

fn startup(_sled: &mut Sled<Rgb>, data: &mut Data) -> SledResult {
    let colors = data.store::<Vec<(f32, Oklab)>>("colors", vec![]);

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

    data.set::<NoiseSettings>(
        "noise_settings",
        NoiseSettings {
            noise_size: 1.25,
            time_scale: 0.2,
        },
    );

    data.set::<Vec2>("move_vec", Vec2::new(0.0, -0.2));

    data.set(
        "generator",
        RidgedMulti::<Perlin>::new(ThreadRng::default().gen_range(0..10_000))
            .set_octaves(4)
            .set_lacunarity(3.5)
            .set_frequency(0.285)
            .set_attenuation(1.37),
    );

    Ok(())
}

fn draw(sled: &mut Sled<Rgb>, data: &Data, time_info: &Time) -> SledResult {
    let generator: &RidgedMulti<Perlin> = data.get("generator")?;

    let noise_settings: &NoiseSettings = data.get("noise_settings")?;
    let colors: &Vec<(f32, Oklab)> = data.get("colors")?;
    let move_vec: &Vec2 = data.get("move_vec")?;

    let size = noise_settings.noise_size;
    let time_scale = noise_settings.time_scale;

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
