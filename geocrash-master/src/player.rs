use ggez::graphics::{DrawParam, BlendMode, Mesh};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;
use nphysics2d::object::{RigidBodyDesc, BodyStatus, RigidBody, Collider, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet, DefaultColliderHandle};
use nalgebra::{Isometry2, Vector2};
use rand::prelude::*;
use std::f32::consts::PI;
use std::alloc::handle_alloc_error;
use ncollide2d::shape::{ShapeHandle, Ball};
use ggez::nalgebra::{UnitComplex, Isometry};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use std::ops::Index;


pub struct Player{
    //TODO: implement player attributes
    score: i32,
    //stores a reference to the RigidBodyObject representing the player
    rigid_body_handle:  Option<DefaultBodyHandle>,
    collider_handle: Option<DefaultColliderHandle>,
}

impl Player {
    pub fn new() -> Self {

        //TODO: create a new player in the center of the screen
        Player {
            score: 0,
            rigid_body_handle: None,
            collider_handle: None,
        }
    }

    pub fn createRigidBody(&mut self, bodies: &mut DefaultBodySet<f32>){

        let left_bound = -50.0;
        let right_bound = 50.0;
        let top_bounds = 0.0;
        let bottom_bounds = 50.0;

        //should players get an id?? Could be helpful
        let id = 1791;

        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(left_bound, right_bound);
        let y_pos = rng.gen_range(top_bounds, bottom_bounds);

        let position = Isometry2::new(Vector2::new(x_pos, y_pos), PI);

        let rigid_body= RigidBodyDesc::new()
            .rotation(5.0)
            .position(position)
            .gravity_enabled(false)
            .status(BodyStatus::Kinematic)
            .max_linear_velocity(10.0)
            .mass(5.0)
            .build();
        let handle = bodies.insert(rigid_body);
        self.rigid_body_handle = Some(handle);

    }

    pub fn createCollider(&mut self, bodies: &mut DefaultBodySet<f32>, colliders: &mut DefaultColliderSet<f32>){

        let handle = self.rigid_body_handle?;

        //retrieve position from RigidBody
        let position =  bodies.get_mut(handle).get_position();

        let shape = ShapeHandle::new(Ball::new(1.5));
        let collider = ColliderDesc::new(shape)
            .position(position)
            .density(1.5)
            .material(MaterialHandle::new(BasicMaterial::new(0.3, 0.5)))
            .margin(0.02)
            .id(1791)
            .build(handle);
        let handle = colliders.insert(collider);
        self.collider_handle = Some(handle);
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

