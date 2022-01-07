use std::{
    error::Error,
    thread::sleep,
    time::Duration,
};
use std::hint::unreachable_unchecked;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Point,
    render::WindowCanvas,
};

use utils::{Board, BoardIndex, Player, Winner};

type Result = std::result::Result<(), Box<dyn Error>>;

const WINDOW_SIZE: i32 = 600;
const CELL_SIZE: i32 = WINDOW_SIZE / 3;
const FPS: f32 = 8.;
const MARGIN: i32 = CELL_SIZE / 6;

pub fn run() -> Result {
    let mut board = Board::default();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Tic-Tac-Toe", WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    clear_canvas(&mut canvas);
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => process_left_click(x, y, &mut board),
                _ => {}
            }
        }

        clear_canvas(&mut canvas);
        draw_grid(&mut canvas)?;
        draw_cells(&mut canvas, &mut board)?;
        canvas.present();
        sleep(Duration::from_secs_f32(1. / FPS));
    }
    Ok(())
}

fn clear_canvas(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
}

fn draw_grid(canvas: &mut WindowCanvas) -> Result {
    canvas.set_draw_color(Color::BLACK);

    for x in (CELL_SIZE..WINDOW_SIZE).step_by(CELL_SIZE as usize) {
        let start = Point::new(x, 0);
        let end = Point::new(x, WINDOW_SIZE);
        canvas.draw_line(start, end)?;
    }

    for y in (CELL_SIZE..WINDOW_SIZE).step_by(CELL_SIZE as usize) {
        let start = Point::new(0, y);
        let end = Point::new(WINDOW_SIZE, y);
        canvas.draw_line(start, end)?;
    }

    Ok(())
}

fn draw_cells(canvas: &mut WindowCanvas, board: &mut Board) -> Result {
    fn draw_cross(canvas: &mut WindowCanvas, center: Point, radius: i32) -> Result {
        canvas.set_draw_color(Color::BLUE);

        let offset = Point::new(radius, radius);
        let start = center - offset;
        let end = center + offset;

        canvas.draw_line(start, end)?;

        let offset = Point::new(-radius, radius);
        let start = center - offset;
        let end = center + offset;

        canvas.draw_line(start, end)?;

        Ok(())
    }

    fn draw_circle(canvas: &mut WindowCanvas, center: Point, radius: i32) -> Result {
        canvas.set_draw_color(Color::RED);

        let diameter = radius * 2;
        let mut x = radius - 1;
        let mut y = 0;
        let mut tx = 1;
        let mut ty = 1;
        let mut error = tx - diameter;

        while x >= y {
            canvas.draw_points(&[
                center.offset(x, -y),
                center.offset(x, y),
                center.offset(-x, -y),
                center.offset(-x, y),
                center.offset(y, -x),
                center.offset(y, x),
                center.offset(-y, -x),
                center.offset(-y, x)
            ][..])?;

            if error <= 0 {
                y += 1;
                error += ty;
                ty += 2;
            }

            if error > 0 {
                x -= 1;
                tx += 2;
                error += tx - diameter;
            }
        }

        Ok(())
    }

    for row in 0..3 {
        for col in 0..3 {
            let (x, y) = (col as i32 * CELL_SIZE, row as i32 * CELL_SIZE);
            let offset = CELL_SIZE / 2;
            let center = Point::new(x + offset, y + offset);

            match board.get_cell((col, row)) {
                Some(Player::X) => draw_cross(canvas, center, offset - MARGIN)?,
                Some(Player::O) => draw_circle(canvas, center, offset - MARGIN)?,
                None => {}
            }
        }
    }

    Ok(())
}

fn process_left_click(x: i32, y: i32, board: &mut Board) {
    let i = ((x / CELL_SIZE) as usize, (y / CELL_SIZE) as usize);
    if board.cell_is_empty(i) {
        update_board(board, i);
    }
}

fn update_board(board: &mut Board, i: BoardIndex) {

    fn make_move_and_check_win(board: &mut Board, i: BoardIndex, player: Player) {
        if let Some(winner) = board.set_cell(i, player) {
            display_winner(winner);
            board.clear();
        }
    }

    make_move_and_check_win(board, i, Player::X);
    let i = ai::generate_move(*board, Player::O);
    make_move_and_check_win(board, i, Player::O);
}

fn display_winner(winner: &Winner) {
    match winner {
        Winner::Draw => println!("Draw"),
        Winner::Player(Player::O) => println!("You lost! (again...)"),
        Winner::Player(Player::X) => unsafe { unreachable_unchecked() },
        // yes i do trust my code that much, why are you asking
    }
}
