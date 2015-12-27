extern crate piston_window;
use piston_window::*;

extern crate poisson;
use poisson::PoissonSurface;

struct App {
    // interactive state of the demo
    poisson: PoissonSurface,
}

impl App {
    fn new(poisson: PoissonSurface) -> App {
        App { poisson: poisson }
    }

    fn input(&mut self, input: Input) {}

    fn update(&mut self, args: UpdateArgs) {}

    fn render(&mut self, args: RenderArgs, window: PistonWindow) {
        window.draw_2d(|ctx, gfx| {
            let red = [1.0, 0.0, 0.0, 1.0];
            let dot = Ellipse::new(red);

            clear([0.0, 0.0, 0.0, 1.0], gfx);
            for p in self.poisson.points_iter() {
                dot.draw(ellipse::circle(p.x, p.y, 2.0),
                         default_draw_state(),
                         ctx.transform,
                         gfx);
            }
        })
    }
}

fn main() {
    let window: PistonWindow = WindowSettings::new("Poisson disc sampling", [640, 480])
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap();
    let mut app = App::new(PoissonSurface::new());
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
