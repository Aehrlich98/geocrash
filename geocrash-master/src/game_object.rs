use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

extern crate nalgebra as na;

use na::{Vector2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, Collider, DefaultBodyHandle, Body};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc, BodyPartHandle};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Cuboid, Ball};
use ggez::graphics::{DrawParam, Color};
use ggez::mint::Point2;
use rand::Rng;
use std::f32::consts::PI;
use ggez::conf::Conf;
use crate::master;
use crate::constants;
use nphysics2d::algebra::{Force2, ForceType};
//OUT type Point = (i32, i32);

pub struct GameObject {
    handleRigidBody: Option<DefaultBodyHandle>,    //mutable handles
    handleCollider: Option<DefaultBodyHandle>,
    registeredPlayerID: Option<i8>,
    id: i8,
}

impl GameObject {
    //create GameObject, add its rigidbody, collider into the sets from Master
    pub fn new(bodies: &mut DefaultBodySet<f32>, id: i8, colliders: &mut DefaultColliderSet<f32>, right_bound: f32, bottom_bound: f32) -> Self{

        //TODO: use context object to make bounds fitted to window
        let left_bound = 0.0;
        let top_bounds = 0.0;

        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(left_bound, right_bound);
        println!("x_pos: {}", x_pos);
        let y_pos = rng.gen_range(top_bounds, bottom_bound);

        let position = Isometry2::new(Vector2::new(x_pos, y_pos), PI);
            //create the necessary isntances for simulation
        let mut rigidBody = RigidBodyDesc::new()
            .mass(0.01)
            .position(position)
            .enable_gravity(false)
            .build();
        rigidBody.set_status(BodyStatus::Dynamic);
        rigidBody.set_linear_damping(1.0);
        rigidBody.set_user_data(Some(Box::new(id)));
        let rb_handle = bodies.insert(rigidBody);

        //let shape = ShapeHandle::new(Cuboid::new(
                //Vector2::new(5.0f32, 5.0)));
        let shape = ShapeHandle::new(Ball::new(1.0));
        let collider = ColliderDesc::new(shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.4, 0.6)))
            .margin(8f32)
            .user_data(id)
            .build(BodyPartHandle(rb_handle, 0));
        let col_handle = colliders.insert(collider);

        let go = GameObject {
            //give handles to GameObject
            handleRigidBody: Some(rb_handle),   //insert into set, get handle, save mutable handle
            handleCollider: Some(col_handle),
            registeredPlayerID: None,
            id: id,
        };

        return go;
    }

    pub fn update(&self, colliders: &mut DefaultColliderSet<f32>, bodies: &mut DefaultBodySet<f32>, pos1: &Isometry2<f32>, pos2: &Isometry2<f32>){
        if self.registeredPlayerID == None {
            return;
        }
        let player_vec = match self.registeredPlayerID {
            Some(constants::PLAYER1_ID) => &pos1.translation.vector,
            Some(constants::PLAYER2_ID) => &pos2.translation.vector,
            _ => return
        };


        let go_pos : &Isometry2<f32> = colliders.get(self.handleCollider.unwrap()).unwrap().position();
        let go_vec = &go_pos.translation.vector;

        let force_multiplier: f32 = 1.5;
        //we have the position of the player and the  game object -> calc player - game object to get force vector
        let force_vec = Vector2::new(force_multiplier*(player_vec.get(0).unwrap() - go_vec.get(0).unwrap()), force_multiplier*(player_vec.get(1).unwrap() - go_vec.get(1).unwrap()));
        let f = Force2::new(force_vec, 0.0f32);

        //apply force to game object
        let mut object = bodies.get_mut(colliders.get(self.handleCollider.unwrap()).unwrap().body()).unwrap();
        object.apply_force(0, &f, ForceType::Impulse, true);
        println!("force applied");
    }

    pub fn registerPlayer(&mut self, id: i8){
        //only register player if no player is registered
        if self.registeredPlayerID == None {
            self.registeredPlayerID = Some(id);
        }
    }

    pub fn deregisterPlayer(&mut self, id: i8){
        if self.registeredPlayerID == Some(id) {
            self.registeredPlayerID = None;
        }
    }

    pub fn draw(&self, context: &mut Context, bodies: &mut DefaultBodySet<f32>) -> GameResult<i8>{
        let rb_handle = self.handleRigidBody.unwrap();
        let rb = bodies.rigid_body(rb_handle).unwrap();

        //println!("drawing");
        let position: &Isometry2<f32> = rb.position();
        let x :f32 = position.translation.vector.get(0).unwrap().clone();
        let y :f32 = position.translation.vector.get(1).unwrap().clone();

        let radius = 10f32;
        let tolerance = 0.00001f32;
        //--
        let p: Point2<f32> =  Point2{
            x,
            y,
        };

        let r2 = graphics::Mesh::new_circle(context, graphics::DrawMode::fill(), p,
            radius, tolerance, graphics::Color::new(0.0, 1.0, 1.0, 0.90))?;
        graphics::draw(context, &r2, DrawParam::default())?;
        Ok(0)
    }

}
