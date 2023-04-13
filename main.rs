extern crate piston_window;
extern crate user32;
extern crate winapi;


use std::process;
use graphics::color::BLACK;
use piston::input::Input::Text;
use piston_window::*;
use std::fs::File;
use std::io::prelude::*;
use std::ptr::null_mut as NULL;
use breakout_piston::ball::Ball;
use breakout_piston::brick::Brick;
use breakout_piston::paddle::Paddle;
use winapi::um::winuser;
use std::io::prelude::*;
use std::io::stdin;
use std::io::Write;

fn main() {

    let mut file = File::open("score.txt")
        .expect("File is  Missing");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("issues with reading the file data");

    let p_msg: Vec<u16> = data.encode_utf16().collect();
    let p_title: Vec<u16> = "Player Scores ".encode_utf16().collect();
    unsafe {
        let p_result = user32::MessageBoxW(NULL(), p_msg.as_ptr(), p_title.as_ptr(), winuser::MB_OK | winuser::MB_USERICON);
        if p_result == 1 {
            playGame()
        }
    }
}

fn playGame(){
    let mut window: PistonWindow = WindowSettings::new("Break out Rust", [800, 600])
        .fullscreen(false)
        .exit_on_esc(false)
        .build()
        .unwrap();




        let window_width = window.size().width;
    let window_height = window.size().height;
    let mut cursor = [0.0, 0.0];
    let mut x1 = 0.0;
    let mut y1 = 0.0;
    let mut score = 0;

    let mut ball = Ball::new(window_width / 2.0, window_height - 100.0, 11.0);
    let mut paddle = Paddle::new(window_width / 2.0, window_height - 50.0, 125.0, 20.0);
    let mut bricks = Brick::make_bricks(window_width);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            score = score + 1;
            match key {
                Key::Left => x1 = x1 - 50.0,
                Key::Right => x1 = x1 + 50.0,
                _ => {}
            }

            cursor = [x1, y1];
        };


        event.mouse_cursor(|x, y| {
            cursor = [x, y];
            x1 = x;
            y1 = y;
            score = score + 1;
        });

        window.draw_2d(&event, |context, graphics| {
            clear([0.5, 0.7, 0.7, 1.0], graphics);

            ball.draw(&context, graphics);
            // text(BLACK,32,"Hiii",graphics.as_any_mut(), context.transform,graphics);
            ball.update();
            if ball.edge_bounce(window_width, window_height) || bricks.is_empty() {
                let s: String = score.to_string();
                let l_msg: Vec<u16> = s.to_string().encode_utf16().collect();
                let l_title: Vec<u16> = "your Score:".encode_utf16().collect();
                unsafe {
                    let result = user32::MessageBoxW(
                        NULL(),
                        l_msg.as_ptr(),
                        l_title.as_ptr(),
                        winuser::MB_OK | winuser::MB_USERICON,
                    );

                    if result == 1 {
                        let mut file = File::open("score.txt").expect("File is Missing");
                        let mut data = String::new();
                        file.read_to_string(&mut data)
                            .expect("issue reading file data");

                        let p_msg: Vec<u16> = data.encode_utf16().collect();
                        let p_title: Vec<u16> = "player scores:".encode_utf16().collect();
                        unsafe {
                            let p_result = user32::MessageBoxW(
                                NULL(),
                                p_msg.as_ptr(),
                                p_title.as_ptr(),
                                winuser::MB_OK | winuser::MB_USERICON,
                            );

                            if result == 1 {
                                let p_msg: Vec<u16> = "https://play.google.com/store/apps/details?id=com.lt.latte.brick&hl=en_US&gl=US".encode_utf16().collect();
                                let p_title: Vec<u16> = "rate the Game".encode_utf16().collect();
                                unsafe {
                                    let p_result = user32::MessageBoxW(
                                        NULL(),
                                        p_msg.as_ptr(),
                                        p_title.as_ptr(),
                                        winuser::MB_OK | winuser::MB_USERICON,
                                    );
                                }

                                if result == 1 {
                                    let p_msg: Vec<u16> =
                                        " play  again ?".encode_utf16().collect();
                                    let p_title: Vec<u16> = "Breakout".encode_utf16().collect();

                                    unsafe {
                                        let p_result = user32::MessageBoxW(
                                            NULL(),
                                            p_msg.as_ptr(),
                                            p_title.as_ptr(),
                                            winuser::MB_YESNO | winuser::MB_USERICON,
                                        );

                                        if p_result == 6 {

                                            replay();


                                        }
                                        if p_result == 7 {
                                            process::exit(0x0100);
                                        }
                                    }
                                }
                            }


                        }
                    }
                }
            }
            ball.hit_paddle(&paddle);
            ball.break_bricks(&mut bricks);
            paddle.draw(&context, graphics);
            paddle.update(cursor[0]);

            for b in bricks.iter() {
                b.draw(&context, graphics);
            }
        });
    }
}

 fn replay()
{
    playGame();
}
