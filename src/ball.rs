//! Represents the ball in a game of Pong.

use {Game, Paddle, Rectangle, Result, UpdateParams};
use glium::Frame;
use rand::{self, Rng};
use std::f32::consts::PI;

const WIDTH: f32 = 15.0;
const HEIGHT: f32 = 15.0;
const BALL_BOUNCE_SPEEDUP: f32 = 1.15;
const BALL_STARTING_SPEED: f32 = 500.0;

// The ball has a speed and moves once per frame.
#[derive(Clone, Debug)]
pub struct Ball {
    bounds: Rectangle,
    vx: f32,
    vy: f32,
    start_timer: f32,
}

impl Ball {
    // Creates a new ball at the given position.
    pub fn new(x: f32, y: f32) -> Ball {
        let mut ball = Ball {
            vx: 0.0,
            vy: 0.0,
            start_timer: 0.0,
            bounds: Rectangle {
                x: 0.0,
                y: 0.0,
                width: WIDTH,
                height: HEIGHT,
            },
        };
        ball.reset(x, y);
        ball
    }

    /// Resets the ball back to the given position.
    /// The ball will stay in place for a moment before moving.
    fn reset(&mut self, x: f32, y: f32) {
        self.bounds.x = x - self.bounds.width / 2.0;
        self.bounds.y = y - self.bounds.height / 2.0;

        let mut rng = rand::thread_rng();
        let dv = rng.gen_range(0.0, 2.0 * PI);
        self.vx = BALL_STARTING_SPEED * dv.cos();
        self.vy = BALL_STARTING_SPEED * dv.sin();
    }

    /// Updates the position of the ball and checks for collisions.
    pub fn update(&mut self, params: &UpdateParams, left_paddle: &mut Paddle, right_paddle: &mut Paddle) {
        // The ball stays still until a timer elapses.
        if self.start_timer > 0.0 {
            self.start_timer -= params.dt;
        } else {
            self.bounds.x += self.vx * params.dt;
            self.bounds.y += self.vy * params.dt;
        }

        // Check collision.
        self.check_paddle_collision(left_paddle);
        self.check_paddle_collision(right_paddle);
        self.check_wall_collision(params);
        self.check_goal(params, left_paddle, right_paddle);
    }

    /// Draws the paddle on the screen.
    pub fn render(&self, game: &Game, frame: &mut Frame) -> Result<()> {
        game.draw_rectangle(frame, self.bounds, [1.0, 1.0, 1.0, 1.0])
    }

    /// Handles collision between the ball and a paddle.
    fn check_paddle_collision(&mut self, paddle: &Paddle) {
        // TODO: Check collision against the given paddle, and reflect the ball if it's colliding.
    }

    /// Handles collision between the ball and the top or bottom of the screen.
    fn check_wall_collision(&mut self, params: &UpdateParams) {
        if self.bounds.y < 0.0 {
            // TOP
            self.vy = -self.vy;
        } else if self.bounds.y + self.bounds.height > params.game_height {
            // BOTTOM
            self.vy = -self.vy;
        }
    }

    /// Handles collision between the ball and the left or right edge of the screen.
    fn check_goal(&mut self, params: &UpdateParams, left_paddle: &mut Paddle, right_paddle: &mut Paddle) {
        // TODO: Check collision against the left or right edge, and give the appropriate player a point.
    }
}
