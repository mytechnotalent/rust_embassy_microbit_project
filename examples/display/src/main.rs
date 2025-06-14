#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_time::Duration;
use microbit_bsp::*;
use {defmt_rtt as _, panic_probe as _};

async fn show_button_press(display: &mut LedMatrix, button_name: &str, arrow: display::Frame<5, 5>) {
    defmt::info!("{} pressed", button_name);
    display.display(arrow, Duration::from_secs(1)).await;
}

async fn handle_button_a_press(display: &mut LedMatrix) {
    show_button_press(display, "A", display::fonts::ARROW_LEFT).await;
}

async fn handle_button_b_press(display: &mut LedMatrix) {
    show_button_press(display, "B", display::fonts::ARROW_RIGHT).await;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let board = Microbit::default();

    let mut display = board.display;
    let mut btn_a = board.btn_a;
    let mut btn_b = board.btn_b;

    display.set_brightness(display::Brightness::MAX);
    display.scroll("Hello, World!").await;
    defmt::info!("Application started, press buttons!");
    loop {
        match select(btn_a.wait_for_low(), btn_b.wait_for_low()).await {
            Either::First(_) => {
                handle_button_a_press(&mut display).await;
            }
            Either::Second(_) => {
                handle_button_b_press(&mut display).await;
            }
        }
    }
}
