use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::{TextChar, WinitInputHelper};
use anyhow::Result;
use comm_lib::get_best_match_device;
use comm_lib::manager::{DeviceManager, Update};
use pixels_graphics_lib::color::*;
use pixels_graphics_lib::math::contains::Contains;
use pixels_graphics_lib::math::Rect;
use pixels_graphics_lib::setup;
use pixels_graphics_lib::text::TextSize;

const LED_START: (isize, isize) = (15, 15);
const LED_SPACING: isize = 12;
const LED_SIZE: isize = 8;
const BUTTON_START: (isize, isize) = (10, 110);
const BUTTON_SPACING: isize = 20;
const BUTTON_WIDTH: isize = 40;
const SCREEN: Rect = Rect::new(90, 10, 210, 50);
const CURSOR_MAX: usize = 84;
const BLANK: u8 = 32;

fn main() -> Result<()> {
    let board = get_best_match_device()?;
    let manager = DeviceManager::new(board);

    run(manager)
}


const BOARD_GREEN: Color = Color::rgb(30, 60, 44);
const LED_BOX: [Rect; 3] = [
    Rect::new(LED_START.0 - LED_SIZE, LED_START.1 - LED_SIZE, LED_START.0 + LED_SIZE, LED_START.1 + LED_SIZE),
    Rect::new(LED_START.0 - LED_SIZE, LED_START.1 + LED_SPACING, LED_START.0 + LED_SIZE, LED_START.1 + LED_SIZE + LED_SIZE + LED_SPACING),
    Rect::new(LED_START.0 - LED_SIZE, LED_START.1 + LED_SIZE + LED_SPACING + LED_SPACING, LED_START.0 + LED_SIZE, LED_START.1 + LED_SIZE + LED_SIZE + LED_SPACING + LED_SIZE + LED_SPACING),
];

fn run(mut manager: DeviceManager) -> Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, mut graphics) = setup(240, 160, "Button Board", true, &event_loop)?;

    let led_colors: [Color; 3] = [GREEN, Color::rgb(0, 150, 255), RED];
    let mut leds: [bool; 3] = [true, false, false];
    let mut text = String::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if graphics.pixels
                .render()
                .map_err(|e| eprintln!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        graphics.clear(BOARD_GREEN);
        if let Err(e) = manager.recv() {
            eprintln!("{:?}", e);
            *control_flow = ControlFlow::Exit;
            return;
        }

        for (i, button) in manager.get_button_state().iter().enumerate() {
            let x = BUTTON_START.0 + (i as isize * (BUTTON_SPACING + BUTTON_WIDTH));
            let y = BUTTON_START.1;

            if *button {
                graphics.draw_rect(x, y, x + BUTTON_WIDTH, y + BUTTON_WIDTH, WHITE);
            } else {
                graphics.draw_frame(x, y, x + BUTTON_WIDTH, y + BUTTON_WIDTH, WHITE);
            }
        }

        for (i, led) in leds.iter().enumerate() {
            let x = LED_START.0;
            let y = LED_START.1 + (i as isize * (LED_SPACING + LED_SIZE));

            if *led {
                graphics.draw_circle_filled(x, y, LED_SIZE, led_colors[i]);
            } else {
                graphics.draw_circle(x, y, LED_SIZE, led_colors[i]);
            }

            // graphics.draw_frame(LED_BOX[i].x1, LED_BOX[i].y1, LED_BOX[i].x2, LED_BOX[i].y2, WHITE);
        }

        graphics.draw_rect(SCREEN.x1, SCREEN.y1, SCREEN.x2, SCREEN.y2, BLACK);

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            let new_text = input.text();
            if !new_text.is_empty() {
                for char in new_text {
                    match char {
                        TextChar::Char(chr) => {
                            if (chr.is_ascii_graphic() || chr as u8 == BLANK)
                                && text.len() < CURSOR_MAX {
                                text.push(chr.to_ascii_uppercase());
                            }
                        }
                        TextChar::Back => {
                            text.pop();
                        }
                    }
                }
                if let Err(e) = manager.send(Update::Text(text.clone())) {
                    eprintln!("{:?}", e);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            let chars = text.chars().collect::<Vec<char>>();
            graphics.draw_text_px(&String::from_iter(&chars), 23, SCREEN.x1 + 1, SCREEN.y1 + 1, TextSize::Small, WHITE);

            if let Some(size) = input.window_resized() {
                graphics.pixels.resize_surface(size.width, size.height);
            }

            if input.mouse_released(0) {
                if let Some((x, y)) = input.mouse() {
                    let x = (x * 0.25).round() as usize;
                    let y = (y * 0.25).round() as usize;

                    for i in 0..=2 {
                        if LED_BOX[i].contains(x, y) {
                            leds[i] = !leds[i];
                            if let Err(e) = manager.send(Update::LED(i, leds[i])) {
                                eprintln!("{:?}", e);
                                *control_flow = ControlFlow::Exit;
                                return;
                            }
                        }
                    }
                }
            }

            window.request_redraw();
        }
    });
}
