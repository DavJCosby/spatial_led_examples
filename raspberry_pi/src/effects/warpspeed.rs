use rand::Rng;

use palette::rgb::Rgb;

use spatial_led::{
    driver::{Data, Driver, Time},
    Sled, SledResult, Vec2,
};

const NUM_STARS: usize = 5000;
const VELOCITY: f32 = 6.0;
const DIRECTION: Vec2 = Vec2::new(0.7071, -0.7071);

pub fn build_driver() -> Driver<Rgb> {
    let mut driver = Driver::new();

    driver.set_startup_commands(startup);
    driver.set_compute_commands(compute);
    driver.set_draw_commands(draw);

    driver
}

fn startup(sled: &mut Sled<Rgb>, data: &mut Data) -> SledResult {
    let stars = data.store::<Vec<Vec2>>("stars", vec![]);
    let center = sled.center_point();
    let mut rng = rand::thread_rng();

    let orth = DIRECTION.perp();

    for _ in 0..NUM_STARS {
        let sign = match rng.gen_bool(0.5) {
            true => 1.0,
            false => -1.0,
        };

        let spawn_pos = center
            + (DIRECTION * rng.gen_range(40.0..300.0))
            + (orth * rng.gen_range(1.45..35.0) * sign);

        stars.push(spawn_pos);
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

fn compute(sled: &Sled<Rgb>, data: &mut Data, time: &Time) -> SledResult {
    let mut rng = rand::thread_rng();
    let delta = time.delta.as_secs_f32();
    let stars = data.get_mut::<Vec<Vec2>>("stars")?;
    let center = sled.center_point();

    let orth = DIRECTION.perp();

    for star in stars {
        *star -= DIRECTION * VELOCITY * delta;
        if star.x.signum() != DIRECTION.x.signum() && star.y.signum() != DIRECTION.y.signum() {
            let dq = (*star - center).length_squared();
            if dq > 1000.0 {
                let sign = match rng.gen_bool(0.5) {
                    true => 1.0,
                    false => -1.0,
                };

                let spawn_pos = center
                    + (DIRECTION * rng.gen_range(40.0..300.0))
                    + (orth * rng.gen_range(1.5..35.0) * sign);

                *star = spawn_pos;
            }
        }
    }

    Ok(())
}

fn draw(sled: &mut Sled<Rgb>, data: &Data, time: &Time) -> SledResult {
    let stars = data.get::<Vec<Vec2>>("stars")?;
    let center = sled.center_point();
    let delta = time.delta.as_secs_f32();

    let fade_amount = 1.0 - (delta * 25.0);

    sled.for_each(|led| led.color *= fade_amount);

    let mut i = 0;
    for star in stars {
        let d = Vec2::new(star.x - center.x, star.y - center.y);
        let c = data.get::<Vec<Rgb>>("colors")?[i % 10];
        sled.modulate_at_dir(d, |led| {
            let d_sq = (d.length() - led.distance()).powi(2);
            led.color + (c / d_sq)
        });
        i += 1;
    }

    Ok(())
}
