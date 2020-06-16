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
use std::ops::{Index, Deref, DerefMut};
use ggez::conf::Conf;
use std::borrow::{Borrow, BorrowMut};
use ggez::input::keyboard::KeyboardContext;
use ggez::input::keyboard;
use nphysics2d::algebra::ForceType::Force;
use nphysics2d::algebra::{ForceType, Force2};
use nphysics2d::force_generator::{ConstantAcceleration, DefaultForceGeneratorSet, DefaultForceGeneratorHandle};
use std::collections::HashMap;
use crate::{master, constants};


pub struct Player{
    //TODO: implement player attributes
    score: i32,
    //stores a reference to the RigidBodyObject representing the player
    rigid_body_handle:  Option<DefaultBodyHandle>,
    collider_handle: Option<DefaultColliderHandle>,
    sensor_collider_handle: Option<DefaultColliderHandle>,
    acc_handles: HashMap<i8, DefaultForceGeneratorHandle>,
}

impl Player {
    pub fn new() -> Self {

        //TODO: create a new player in the center of the screen
        let mut p = Player {
            score: 0,
            rigid_body_handle: None,
            collider_handle: None,
            sensor_collider_handle: None,
            acc_handles: HashMap::with_capacity(4),
        };
        return p;
    }

    pub fn createRigidBody(&mut self, bodies: &mut DefaultBodySet<f32>){

        //TODO: use context object to make bounds fitted to window
        let left_bound = 0.0;
        let right_bound = 800.0;
        let top_bounds = 0.0;
        let bottom_bounds = 600.0;

        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(left_bound, right_bound);
        println!("x_pos: {}", x_pos);
        let y_pos = rng.gen_range(top_bounds, bottom_bounds);

        let position = Isometry2::new(Vector2::new(x_pos, y_pos), PI);

        let mut rigid_body= RigidBodyDesc::new()
            .position(position)
            .gravity_enabled(false)
            .status(BodyStatus::Dynamic)
            .max_linear_velocity(500.0)
            .mass(0.1)
            .linear_damping(1.0)
            .user_data(constants::PLAYER_ID)
            .build();
        rigid_body.disable_all_rotations();
        let handle = bodies.insert(rigid_body);
        self.rigid_body_handle = Some(handle);

    }

    pub fn create_collider(&mut self, colliders: &mut DefaultColliderSet<f32>){

        //TODO: find better exception handling
        let handle = self.rigid_body_handle.unwrap();

        let shape = ShapeHandle::new(Ball::new(1.5));
        let colliderPattern = ColliderDesc::new(shape)
            .density(1.5)
            .material(MaterialHandle::new(BasicMaterial::new(0.3, 0.5)))
            .margin(35.00)
            .user_data(constants::PLAYER_ID);
        let collider = colliderPattern.build(BodyPartHandle(handle, 0));
        let sensor = colliderPattern
            .sensor(true)
            .linear_prediction(150.0) //this defines the maximum distance between a player and a game object that still triggers a proximity event
            .build(BodyPartHandle(handle, 0));
        let collider_handle = colliders.insert(collider);
        self.sensor_collider_handle = Some(colliders.insert(sensor));
        self.collider_handle = Some(collider_handle);
    }

    pub fn update(&mut self,  context: &mut Context, bodies: &mut DefaultBodySet<f32>, force_generators: &mut DefaultForceGeneratorSet<f32>) {
        //TODO: update player


        if keyboard::is_key_pressed(context, KeyCode::Left) {
            if !self.acc_handles.contains_key(&constants::LEFT){
                let mut left_acc= ConstantAcceleration::new(Vector2::new(-180.0f32, 0.0), 0.0);
                left_acc.add_body_part(BodyPartHandle(self.rigid_body_handle.unwrap(), 0));
                let left_handle = force_generators.insert(Box::new(left_acc));
                self.acc_handles.insert(constants::LEFT, left_handle);
            }

        } else {
            if let Some(h) = self.acc_handles.remove(&constants::LEFT){
                force_generators.remove(h);
            }
        }
        if keyboard::is_key_pressed(context, KeyCode::Right) {
            if !self.acc_handles.contains_key(&constants::RIGHT){
                let mut right_acc= ConstantAcceleration::new(Vector2::new(180.0f32, 0.0), 0.0);
                right_acc.add_body_part(BodyPartHandle(self.rigid_body_handle.unwrap(), 0));
                let right_handle = force_generators.insert(Box::new(right_acc));
                self.acc_handles.insert(constants::RIGHT, right_handle);
            }

        } else {
            if let Some(h) = self.acc_handles.remove(&constants::RIGHT){
                force_generators.remove(h);
            }
        }
        if keyboard::is_key_pressed(context, KeyCode::Up) {
            if !self.acc_handles.contains_key(&constants::UP){
                let mut up_acc= ConstantAcceleration::new(Vector2::new(0.0, -180.0f32), 0.0);
                up_acc.add_body_part(BodyPartHandle(self.rigid_body_handle.unwrap(), 0));
                let up_handle = force_generators.insert(Box::new(up_acc));
                self.acc_handles.insert(constants::UP, up_handle);
            }

        } else {
            if let Some(h) = self.acc_handles.remove(&constants::UP){
                force_generators.remove(h);
            }
        }
        if keyboard::is_key_pressed(context, KeyCode::Down) {
            if !self.acc_handles.contains_key(&constants::DOWN){
                let mut down_acc= ConstantAcceleration::new(Vector2::new(0.0, 180.0f32), 0.0);
                down_acc.add_body_part(BodyPartHandle(self.rigid_body_handle.unwrap(), 0));
                let down_handle = force_generators.insert(Box::new(down_acc));
                self.acc_handles.insert(constants::DOWN, down_handle);
            }

        } else {
            if let Some(h) = self.acc_handles.remove(&constants::DOWN){
                force_generators.remove(h);
            }
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

