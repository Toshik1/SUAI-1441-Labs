extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Scancode::*;
use std::time::Duration;

mod matrix;
mod primitives;
mod line;
use primitives::Primitive2D;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn render_pixel(renderer: &mut sdl2::render::Renderer, x: i32, y: i32, color: Color) {
    let last_color = renderer.draw_color();
    renderer.set_draw_color(color);
    renderer.draw_point(Point::new(x, y)).unwrap();
    renderer.set_draw_color(last_color);
}

fn main() {
    // Initialize SDL2
    let sdl_context   = sdl2::init().unwrap();
    let video_context = sdl_context.video().unwrap();

    // Create window
    let window = video_context.window("Graphics Task 1", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Get renderer binded to window
    let mut renderer = window.renderer().build().unwrap();

    // Set draw color and clear the screen
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    // Get event pump
    let mut events = sdl_context.event_pump().unwrap();

    // Set initial lines coordinates
    let startx  = ((WIDTH/2) - 10) as f32;
    let starty1 = (HEIGHT/4)       as f32;
    let starty2 = (HEIGHT/4*3)     as f32;

    // Create lines
    let mut line1 = line::Line::new(
        startx - 10.0,
        starty1,
        startx - 10.0,
        starty2,
        Color::RGB(0, 255, 0)
    );

    let mut line2 = line::Line::new(
        startx + 10.0,
        starty1,
        startx + 10.0,
        starty2,
        Color::RGB(255, 0, 0)
    );

    // Start main loop
    'main:
    loop {
        // If exit event passed, break the loop
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                _ => ()
            }
        }

        // Initialize variables
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut scale = 1.0;
        let mut angle = 0.0;

        // Poll presed keys
        for key in events.keyboard_state().pressed_scancodes() {
            match key {
                W       => dy -= 1.0,
                S       => dy += 1.0,
                D       => dx += 1.0,
                A       => dx -= 1.0,
                Up      => scale += 0.01,
                Down    => scale -= 0.01,
                Left    => angle -= 1.0,
                Right   => angle += 1.0,
                _       => (),
            }
        }

        // Do affine transformations
        line1.translate(dx, dy);
        line1.scale(scale, scale);
        line1.rotate(angle);

        line2.translate(dx, dy);
        line2.scale(scale, scale);
        line2.rotate(angle);

        // Clear render buffer
        renderer.clear();

        // Draw lines
        line1.draw(|x, y, color| render_pixel(&mut renderer, x, y, color));
        line2.draw_builtin_line(&mut renderer);

        // Present render buffer
        renderer.present();

        // Sleep for 10 msecs
        std::thread::sleep(Duration::from_millis(10));
    }

}
