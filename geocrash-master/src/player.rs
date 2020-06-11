use ggez::graphics::{DrawParam, BlendMode, Mesh};
use ggez::{graphics, ContextBuilder, GameResult, Context};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::mint::Point2;
use nphysics2d::object::{RigidBodyDesc, BodyStatus, RigidBody, Collider, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet, DefaultColliderHandle, BodyPartHandle};
use nalgebra::{Isometry2, Vector2};
use rand::prelude::*;
use std::f32::consts::PI;
use std::alloc::handle_alloc_error;
use ncollide2d::shape::{ShapeHandle, Ball};
use ggez::nalgebra::{UnitComplex, Isometry};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use std::ops::Index;
use ggez::conf::Conf;
use std::borrow::{Borrow, BorrowMut};
use ggez::input::keyboard::KeyboardContext;
use ggez::input::keyboard;
use nphysics2d::algebra::ForceType::Force;
use nphysics2d::algebra::{ForceType, Force2};
use nphysics2d::force_generator::{ConstantAcceleration, DefaultForceGeneratorSet};


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

        //TODO: use context object to make bounds fitted to window
        let left_bound = 0.0;
        let right_bound = 350.0;
        let top_bounds = 0.0;
        let bottom_bounds = 250.0;

        //should players get an id?? Could be helpful
        let id = 1791;

        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(left_bound, right_bound);
        println!("x_pos: {}", x_pos);
        let y_pos = rng.gen_range(top_bounds, bottom_bounds);

        let position = Isometry2::new(Vector2::new(x_pos, y_pos), PI);

        let mut rigid_body= RigidBodyDesc::new()
            .rotation(5.0)
            .position(position)
            .gravity_enabled(true)
            .status(BodyStatus::Dynamic)
            .max_linear_velocity(100.0)
            .mass(1.0)
            .build();
        rigid_body.disable_all_rotations();
        let handle = bodies.insert(rigid_body);
        self.rigid_body_handle = Some(handle);

    }

    pub fn create_collider(&mut self, colliders: &mut DefaultColliderSet<f32>){

        //TODO: find better exception handling
        let handle = self.rigid_body_handle.unwrap();

        let shape = ShapeHandle::new(Ball::new(1.5));
        let collider = ColliderDesc::new(shape)
            .density(1.5)
            .material(MaterialHandle::new(BasicMaterial::new(0.3, 0.5)))
            .margin(0.02)
            .user_data(1791) //id
            .build(BodyPartHandle(handle, 0));
        let collider_handle = colliders.insert(collider);
        self.collider_handle = Some(collider_handle);
    }

    pub fn update(&mut self,  context: &mut Context, bodies: &mut DefaultBodySet<f32>, force_generators: &mut DefaultForceGeneratorSet<f32>) {
        //TODO: update player

        if keyboard::is_key_pressed(context, KeyCode::Left) {
            let player_body = bodies.get_mut(self.rigid_body_handle.unwrap()).unwrap();

            println!("{}",player_body.part(0).unwrap().center_of_mass());

            let mut left_acc = ConstantAcceleration::new(Vector2::new(-50.0, 10.0), 0.0);
            left_acc.add_body_part(BodyPartHandle(self.rigid_body_handle.unwrap(), 0));
            force_generators.insert(Box::new(left_acc));
        }
    }

    pub fn draw(&self, context: &mut Context, bodies: &mut DefaultBodySet<f32>) -> GameResult<i8>{

        let rb_handle = self.rigid_body_handle.unwrap();
        let rb = bodies.rigid_body(rb_handle).unwrap();

        let position: &Isometry2<f32> = rb.position();
        let x :f32 = position.translation.vector.get(0).unwrap().clone();
        let y :f32 = position.translation.vector.get(1).unwrap().clone();

        //TODO: get real radius
        let radius = 30f32;

        let tolerance = 0.00001f32;
        //--
        let p: Point2<f32> =  Point2{
            x,
            y,
        };

        let r2 = graphics::Mesh::new_circle(context, graphics::DrawMode::fill(), p,
            radius, tolerance, graphics::Color::new(0.7, 0.4, 0.9, 0.8))?;
        graphics::draw(context, &r2, DrawParam::default())?;
        Ok(0)
    }
}

