use ggez::graphics::{DrawParam, BlendMode, Mesh};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;
use nphysics2d::object::{RigidBodyDesc, BodyStatus, RigidBody};
use nalgebra::{Isometry2, Vector2};
use rand::prelude::*;
use std::f32::consts::PI;


pub struct Player{
    //TODO: implement player attributes
    score: i32,
    rigid_body: RigidBody<f64>,
}

impl Player {
    pub fn new() -> Self {

        let left_bound = -50.0;
        let right_bound = 50.0;
        let top_bounds = 0.0;
        let bottom_bounds = 50.0;

        //should players get an id?? Could be helpful
        let id = 1791;

        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(left_bound, right_bound);
        let y_pos = rng.gen_range(top_bounds, bottom_bounds);

        let rigid_body= RigidBodyDesc::new()
            .rotation(5.0)
            .position(Isometry2::new(Vector2::new(x_pos, y_pos), PI))
            .gravity_enabled(false)
            .status(BodyStatus::Kinematic)
            .max_linear_velocity(10.0)
            .mass(5.0)
            .build();

        //TODO: create a new player in the center of the screen
        Player {
            score: 0,
            rigid_body: rigid_body,
        }
    }

    pub fn update() {
        //TODO: update player
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<i8>{

        //these should later be changed to get the real values out of the player struct
        let x_pos = 200f32;
        let y_pos = 200f32;
        let radius = 30f32;
        let tolerance = 0.00001f32;
        //--
        let p: Point2<f32> =  Point2{
            x: x_pos,
            y: y_pos,
        };

        let r2 = graphics::Mesh::new_circle(context, graphics::DrawMode::fill(), p,
            radius, tolerance, graphics::Color::new(0.7, 0.4, 0.9, 0.8))?;
        graphics::draw(context, &r2, DrawParam::default())?;
        Ok(0)
    }
}

