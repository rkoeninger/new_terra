extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use image::ImageDecoder;
use image::bmp::BMPDecoder;
use std::fs::File;

const SIZE: u32 = 50;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct App {
    gl: GlGraphics,
    rotation_blue: f64,
    rotation_red: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, SIZE as f64);
        let (rot_blue, rot_red) = (self.rotation_blue, self.rotation_red);
        let (x, y) = (args.width as f64 / 2.0, args.height as f64 / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform_blue = c.transform
                .trans(x, y)
                .rot_rad(rot_blue)
                .trans(SIZE as f64 * -2.5, SIZE as f64 * -2.5);
            rectangle(BLUE, square, transform_blue, gl);

            let transform_red = c.transform
                .trans(x, y)
                .rot_rad(rot_red)
                .trans(SIZE as f64 * -1.25, SIZE as f64 * -1.25);
            rectangle(RED, square, transform_red, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation_blue -= args.dt;
        self.rotation_red += 2.0 * args.dt;
    }
}

fn run() -> std::io::Result<()> {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let file = File::open("./scout.bmp")?;
    let mut decoder = BMPDecoder::new(file);

    let mut data = match decoder.read_image() {
        Ok(x) => Ok(x),
        Err(_) => Err(std::io::Error::last_os_error())
    }?;

    let mut window: Window = WindowSettings::new(
        "Orbits",
        [SIZE * 8, SIZE * 8]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation_blue: 0.0,
        rotation_red: 0.0
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }

    Ok(())
}

fn main() {
    match std::env::current_dir() {
        Ok(p) => println!("pwd: {:?}", p),
        _ => println!("PWD UNKNOWN")
    }
    match run() {
        Err(x) => println!("FAILURE: {}", x),
        _ => ()
    }
}
