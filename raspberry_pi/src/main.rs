use std::time::Instant;

use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder};
use sled::{color::Srgb, Sled};

mod effects;

fn main() {
    let sled = Sled::new("./config.yap").unwrap();
    let num_leds = sled.num_leds();
    println!("Starting SLED system of {} LEDs.", num_leds);

    let mut driver = effects::ripples::build_driver();
    driver.mount(sled);

    println!("Ripples effect Running. Press CTRL+C to quit.");

    let mut gpio_controller = construct_gpio_controller(num_leds);
    loop {
        driver.step();
        let colors = driver.colors();
        update_gpio(&mut gpio_controller, colors);
    }
}

fn construct_gpio_controller(num_leds: usize) -> Controller {
    ControllerBuilder::new()
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(num_leds as i32)
                .strip_type(rs_ws281x::StripType::Ws2811Gbr)
                .brightness(255)
                .build(),
        )
        .build()
        .unwrap()
}

fn update_gpio<'a>(controller: &mut Controller, colors: impl Iterator<Item = &'a Srgb>) {
    let leds = controller.leds_mut(0);

    let mut i = 0;
    for color in colors {
        leds[i] = [
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
            0,
        ];
        i += 1;
    }
    controller.render().unwrap();
}
