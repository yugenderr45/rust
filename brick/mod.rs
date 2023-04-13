use rand::{thread_rng, Rng};

use graphics::{Context, Graphics};
use piston_window::rectangle;

pub struct Brick {
    color: [f32; 4],
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Brick {
    pub fn new(color: [f32; 4], x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { color, x, y, w, h }
    }

    pub fn make_bricks(window_width: f64) -> Vec<Self> {
        let mut rng = thread_rng();

        let mut bricks = Vec::new();
        let colors = [
            [1.0, 0.0, 0.0, 1.0],
            [1.0, 0.5, 0.0, 1.0],
            [1.0, 1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
        ];
        for (y, c) in colors.iter().enumerate() {
            let mut x = 4.0;
            let actual_y = y as i32 * 35 + 100;

            while x < window_width - 100.0 {
                let w = rng.gen_range(35.0, 85.0);
                bricks.push(Brick::new(*c, x, f64::from(actual_y), w, 30.0));
                x += w + 5.0;
            }
            bricks.push(Brick::new(
                *c,
                x,
                f64::from(actual_y),
                window_width - x - 5.0,
                30.0,
            ));
        }
        bricks
    }

    pub fn draw<G: Graphics>(&self, context: &Context, graphics: &mut G) {
        rectangle(
            self.color,
            [self.x, self.y, self.w, self.h],
            context.transform,
            graphics,
        )
    }
}
