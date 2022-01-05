use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

type Result = std::result::Result<(), Box<dyn Error>>;

const WINDOW_SIZE: i32 = 600;
const CELL_SIZE: i32 = 600;

pub fn run() -> Result {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
}
