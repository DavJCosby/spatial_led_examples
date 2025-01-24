use spatial_led::Sled;

const ROOM_CONFIG: &str = "center: (1.75, 0.5) \n
density: 45 \n
--segments-- \n
(0, 2.8) --> (0, 3.5) --> (1.54, 3.15) --> (3.64, 3.15) --> \n
(4.2, 2.8) --> (4.1, -0.7) --> (-1.4, -0.7) --> (-1.4, 2.1) --> \n
(0, 2.8) --> (1.54, 2.45) --> (2.85, 1.8) --> (2.85, 1.4)";

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut sled = Sled::<(u8, u8, u8)>::new_from_str(ROOM_CONFIG).unwrap();
    log::info!("LED Count: {}", sled.num_leds());

    sled.set_vertices((255, 25, 255));
}
