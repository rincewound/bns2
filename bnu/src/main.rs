use game::MouseEvent;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
use std::time::Duration;
use vecmath::Vec2d;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

mod draw;
mod game;
mod vecmath;

pub const window_width: u32 = 800;
pub const window_height: u32 = 600;
pub const window_center: Vec2d = Vec2d::new(window_width as f32 / 2.0, window_height as f32 / 2.0);

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Battleships... against a unicorn",
            window_width,
            window_height,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let texture_c = canvas.texture_creator();
    let uni = texture_c
        .load_texture("./assets/evilu_pixels_transparent.png")
        .unwrap();
    let ocean = texture_c
        .load_texture("./assets/water.png")
        .unwrap();


    let mut texture_dict: HashMap<String, Texture> = HashMap::new();
    texture_dict.insert("unicorn".to_string(), uni);
    texture_dict.insert("water".to_string(), ocean);


    let mut g = game::Game::new(&mut canvas);

    loop {
        if !(do_events(&mut g, &mut event_pump)) {
            break;
        }
        g.tick();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        g.render(&mut canvas, &texture_dict);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

fn do_events(g: &mut game::Game, event_pump: &mut sdl2::EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } =>
            {

                g.mouseeveent(MouseEvent::Motion{x: x.try_into().unwrap() , y: y.try_into().unwrap()});
            }

            Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } =>
            {
                g.mouseeveent(MouseEvent::Click{x: x.try_into().unwrap() , y: y.try_into().unwrap()});
            }
            
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {}
            Event::KeyUp {
                keycode: Some(Keycode::Space),
                ..
            } => {}

            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => {}

            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {}

            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => {}

            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {}
            _ => {}
        }
    }
    return true;
}
