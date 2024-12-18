use rand::Rng;
use std::f32::consts::{PI, TAU};
use std::time::Duration;

use palette::{chromatic_adaptation::AdaptInto, oklch::Oklch, rgb::Rgb};

use spatial_led::{
    driver::{Data, Driver, Time},
    scheduler::Scheduler,
    Sled, SledResult, Vec2,
};

const SCAN_DURATION: f32 = 4.0;

mod tui;
use tui::SledTerminalDisplay;

fn main() {
    let sled = Sled::new("./complex_room.yap").unwrap();
    let mut display = SledTerminalDisplay::start("Scan", sled.domain());
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

    driver
}

fn rand_endpoints(sled: &Sled<Rgb>) -> (Vec2, Vec2) {
    let domain = sled.domain();
    let r = (domain.end - domain.start).length() * 0.6;
    let c = sled.center_point();

    let mut rng = rand::thread_rng();
    let start_angle = rng.gen_range(0.0..TAU);
    let end_angle = start_angle + PI;

    let start = c + (Vec2::from_angle(start_angle) * r);
    let end = c + (Vec2::from_angle(end_angle) * r);
    (start, end)
}

fn start_new_scan(sled: &Sled<Rgb>, buffers: &mut Data, now: Duration) {
    let t_buffer = buffers.store::<Vec<Duration>>("times", vec![]);

    t_buffer.push(now);
    t_buffer.push(now + Duration::from_secs_f32(SCAN_DURATION));

    let endpoints = buffers.store::<Vec<Vec2>>("vectors", vec![]);
    let (start, end) = rand_endpoints(&sled);
    endpoints.push(start); // v0 will be start point
    endpoints.push(end); // v1 will be end point
    endpoints.push(start); // v2 will be interpolation between v1 and v2
    endpoints.push((end - start).normalize()); // v3 will be direction of movement
}

fn startup(sled: &mut Sled<Rgb>, data: &mut Data) -> SledResult {
    start_new_scan(sled, data, Duration::from_secs(0));
    Ok(())
}

fn compute(sled: &Sled<Rgb>, data: &mut Data, time: &Time) -> SledResult {
    let t_buffer = data.get::<Vec<Duration>>("times")?;
    let now = time.elapsed;
    let end_t = t_buffer[1];

    if now > end_t {
        start_new_scan(sled, data, time.elapsed);
        return Ok(());
    }

    let v_buffer = data.get_mut::<Vec<Vec2>>("vectors")?;
    let start_p = v_buffer[0];
    let end_p = v_buffer[1];
    let a = 1.0 - ((end_t.as_secs_f32() - now.as_secs_f32()) / SCAN_DURATION);

    v_buffer[2] = start_p.lerp(end_p, a);
    Ok(())
}

fn draw(sled: &mut Sled<Rgb>, data: &Data, time: &Time) -> SledResult {
    // gradual fade to black
    let theta = ((time.elapsed.as_secs_f32() / 12.5).cos() + 1.0) * 180.0;
    sled.map(|led| led.color * (1.0 - time.delta.as_secs_f32() * 2.0));

    let v_buffer = data.get::<Vec<Vec2>>("vectors")?;
    let scan_center = v_buffer[2];
    let scan_direction = v_buffer[3];

    let c: Rgb = Oklch::new(0.99, 0.3, theta).adapt_into();

    sled.set_at_dir_from(scan_direction.perp(), scan_center, c);
    sled.set_at_dir_from(-scan_direction.perp(), scan_center, c);

    Ok(())
}
