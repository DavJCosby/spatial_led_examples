use spatial_led::{
    driver::{Data, Driver, Time},
    scheduler::Scheduler,
    Sled, SledResult, Vec2,
};

use palette::rgb::Rgb;

use rand::Rng;
use std::ops::Range;

const MAX_RIPPLES: usize = 12;
const MAX_RADIUS: f32 = 12.0;
const FEATHERING: f32 = 0.15;
const INV_F: f32 = 1.0 / FEATHERING;

mod tui;
use tui::SledTerminalDisplay;

fn main() {
    let sled = Sled::new("./complex_room.yap").unwrap();
    let mut display = SledTerminalDisplay::start("Ripples", sled.domain());
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
    driver.set_compute_commands(compute);
    driver.set_draw_commands(draw);
    return driver;
}

fn startup(sled: &mut Sled<Rgb>, data: &mut Data) -> SledResult {
    let sled_bounds = sled.domain();

    let radii = data.store::<Vec<f32>>("radii", vec![]);
    for _ in 0..MAX_RIPPLES {
        radii.push(rand_init_radius());
    }

    let positions = data.store::<Vec<Vec2>>("positions", vec![]);
    for _ in 0..MAX_RIPPLES {
        positions.push(rand_point_in_range(&sled_bounds));
    }

    data.set::<Vec<Rgb>>(
        "colors",
        vec![
            Rgb::new(0.15, 0.5, 1.0),
            Rgb::new(0.25, 0.3, 1.0),
            Rgb::new(0.05, 0.4, 0.8),
            Rgb::new(0.7, 0.0, 0.6),
            Rgb::new(0.05, 0.75, 1.0),
            Rgb::new(0.1, 0.8, 0.6),
            Rgb::new(0.6, 0.05, 0.2),
            Rgb::new(0.85, 0.15, 0.3),
            Rgb::new(0.0, 0.0, 1.0),
            Rgb::new(1.0, 0.71, 0.705),
        ],
    );

    Ok(())
}

fn compute(sled: &Sled<Rgb>, data: &mut Data, time_info: &Time) -> SledResult {
    let delta = time_info.delta.as_secs_f32();
    let bounds = sled.domain();
    for i in 0..MAX_RIPPLES {
        let radius: f32 = data.get::<Vec<f32>>("radii")?[i];
        if radius > MAX_RADIUS {
            let new_pos = rand_point_in_range(&bounds);
            let new_radius = rand_init_radius();
            data.get_mut::<Vec<Vec2>>("positions")?[i] = new_pos;
            data.get_mut::<Vec<f32>>("radii")?[i] = new_radius;
            continue;
        }

        let new_radius = radius + delta * radius.max(1.0).sqrt().recip();
        data.get_mut::<Vec<f32>>("radii")?[i] = new_radius;
    }
    Ok(())
}

fn rand_point_in_range(range: &Range<Vec2>) -> Vec2 {
    let mut rng = rand::thread_rng();
    Vec2::new(
        rng.gen_range(range.start.x * 1.25..range.end.x * 1.25),
        rng.gen_range(range.start.y * 1.25..range.end.y * 1.25),
    )
}

fn rand_init_radius() -> f32 {
    let mut rng = rand::thread_rng();
    // using a negative radius, we can scheudle a delay before the ripple actually appears
    rng.gen_range(-32.0..0.0)
}

fn draw(sled: &mut Sled<Rgb>, data: &Data, _time: &Time) -> SledResult {
    sled.set_all(Rgb::new(0.0, 0.0, 0.0));
    let colors: &Vec<Rgb> = data.get("colors")?;
    let positions: &Vec<Vec2> = data.get("positions")?;
    let radii: &Vec<f32> = data.get("radii")?;
    for i in 0..MAX_RIPPLES {
        let pos = positions[i];
        let radius = radii[i];

        if radius > -FEATHERING {
            draw_ripple_at(sled, pos, radius, colors[i % colors.len()]);
        }
    }

    // reinhard tonemapping
    // sled.map(|led| led.color / (Rgb::new(1.0, 1.0, 1.0) + led.color));
    Ok(())
}

fn draw_ripple_at(sled: &mut Sled<Rgb>, pos: Vec2, radius: f32, color: Rgb) {
    let inv_radius = 1.0 / radius;
    sled.modulate_within_dist_from(radius + FEATHERING, pos, |led| {
        let r = led.position().distance(pos);
        if r >= radius {
            let dist = r - radius;
            if dist < FEATHERING {
                let factor = (FEATHERING - dist) * INV_F;
                return led.color + color * (factor * inv_radius);
            }
        } else {
            let factor = r * inv_radius;
            return led.color + color * factor.powi(2) * inv_radius;
        }
        led.color
    });
}
