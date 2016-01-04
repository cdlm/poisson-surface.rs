extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate find_folder;
extern crate fps_counter;
use piston::input::*;
use graphics::*;
use graphics::types::Color;
use piston_window::{PistonWindow, WindowSettings, OpenGL, Glyphs};
use fps_counter::FPSCounter;

extern crate rand;
extern crate nalgebra;
extern crate poisson;

use rand::XorShiftRng;
use nalgebra::Vec2;

use poisson::{PoissonDisk, PoissonGen, PoissonIter};

struct App<'a> {
    // interactive state of the demo
    poisson: PoissonIter<'a, XorShiftRng, Vec2<f64>>,
    points: Vec<Vec2<f64>>,
    fps: FPSCounter,
    glyphs: Glyphs,
}


const RED: Color = [1.0, 0.0, 0.0, 1.0];

impl<'a> App<'a> {
    fn new(poisson: &'a mut PoissonGen<XorShiftRng, Vec2<f64>>, glyphs: Glyphs) -> App<'a> {
        App {
            poisson: poisson.into_iter(),
            points: Vec::new(),
            fps: FPSCounter::new(),
            glyphs: glyphs,
        }
    }

    fn fps(&mut self) -> String {
        self.fps.tick().to_string()
    }

    fn input(&mut self, _: Input) {}

    fn update(&mut self, _: UpdateArgs) {
        self.poisson.next().map({
            |p| self.points.push(p)
        });
    }

    fn render(&mut self, _: RenderArgs, window: PistonWindow) {
        window.draw_2d(|ctx, gfx| {
            let dot = Ellipse::new(RED);
            clear(color::BLACK, gfx);

            for p in &self.points {
                dot.draw(ellipse::circle(p.x * 700., p.y * 700., 1.1),
                         default_draw_state(),
                         ctx.transform,
                         gfx);
            }

            text(color::WHITE,
                 12,
                 &self.fps(),
                 &mut self.glyphs,
                 ctx.transform.trans(20.0, 20.0),
                 gfx)
        });
    }
}

fn main() {
    let opengl_version = OpenGL::V3_2;
    let window: PistonWindow = WindowSettings::new("Poisson disc sampling", [700, 700])
                                   .exit_on_esc(true)
                                   .opengl(opengl_version)
                                   .samples(8)
                                   .build()
                                   .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
                     .for_folder("assets")
                     .unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.otf");
    let factory = window.factory.borrow().clone();
    let glyphs = Glyphs::new(font, factory).unwrap();

    let samples: u32 = std::env::args()
                           .nth(1)
                           .and_then(|arg| arg.parse().ok())
                           .unwrap_or(500);
    let radius: f64 = std::env::args()
                          .nth(2)
                          .and_then(|arg| arg.parse().ok())
                          .unwrap_or(0.8);
    let mut poisson = PoissonDisk::new(rand::weak_rng()).build_samples(samples, radius);
    let mut app = App::new(&mut poisson, glyphs);

    for e in window {
        match e.event {
            Some(Event::Input(i)) => {
                app.input(i);
            }
            Some(Event::Update(u)) => {
                app.update(u);
            }
            Some(Event::Render(r)) => {
                app.render(r, e);
            }
            _ => {}
        }
    }
}
