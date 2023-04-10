use chip8_core::*;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, sys::SDL_HINT_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK};
use std::{fs::File, io::Read};

const SCALE: u32 = 10;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGTH: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let mut chip8 = Emulator::new();
    let mut rom = File::open("ibm.ch8").expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Buffer error");
    chip8.load(&buffer);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("chip8 emulator", WINDOW_WIDTH, WINDOW_HEIGTH)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'gameloop;
                }
                _ => (),
            }
        }
        for _ in 0..10 {
            chip8.tick();
        }
        draw_screen(&chip8, &mut canvas);
    }
}

fn draw_screen(emulator: &Emulator, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let screen_buffer = emulator.get_display();
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    for (index, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            let x = (index % SCREEN_WIDTH) as u32;
            let y = (index / SCREEN_WIDTH) as u32;
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
