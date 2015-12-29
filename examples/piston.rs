extern crate piston;
use piston::input::*;

extern crate graphics;
use graphics::*;

extern crate piston_window;
use piston_window::{PistonWindow, WindowSettings, OpenGL, Glyphs};

extern crate find_folder;
extern crate fps_counter;
use fps_counter::FPSCounter;

extern crate poisson;
use poisson::PoissonSurface;

struct App {
    // interactive state of the demo
    poisson: PoissonSurface,
    fps: FPSCounter,
    glyphs: Glyphs,
}

impl App {
    fn new(poisson: PoissonSurface, glyphs: Glyphs) -> App {
        App {
            poisson: poisson,
            fps: FPSCounter::new(),
            glyphs: glyphs,
        }
    }

    fn fps(&mut self) -> String {
        self.fps.tick().to_string()
    }

    fn input(&mut self, input: Input) {}

    fn update(&mut self, args: UpdateArgs) {
        for _ in 1..10 {
            if let Some(point) = self.poisson.generate_point() {
                self.poisson.insert(point);
            }
        }
    }

    fn render(&mut self, args: RenderArgs, window: PistonWindow) {
        window.draw_2d(|ctx, gfx| {
            let red = [1.0, 0.0, 0.0, 1.0];
            let black = [0.0, 0.0, 0.0, 1.0];
            let white = [1.0, 1.0, 1.0, 1.0];

            let dot = Ellipse::new(red);

            clear(black, gfx);
            for p in self.poisson.points_iter() {
                dot.draw(ellipse::circle(p.x, p.y, 2.0),
                         default_draw_state(),
                         ctx.transform,
                         gfx);
            }

            text(white,
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
    let window: PistonWindow = WindowSettings::new("Poisson disc sampling", [640, 480])
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

    let mut app = App::new(PoissonSurface::new(), glyphs);
    let seed = app.poisson.random_point();
    app.poisson.insert(seed);

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
