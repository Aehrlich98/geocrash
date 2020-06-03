use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

extern crate nalgebra as na;

use na::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, Collider, DefaultBodyHandle,
                         DefaultBodySet, DefaultColliderSet , ColliderDesc, BodyPartHandle, DefaultColliderHandle};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::force_generator::{DefaultForceGeneratorSet};
use nphysics2d::joint::{DefaultJointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Cuboid};
use std::ops::Index;


//OUT type Point = (i32, i32);

pub struct GameObject {
    handleRigidBody: Option<DefaultBodyHandle>,    //mutable handles
    handleCollider: Option<DefaultColliderHandle>,

}

impl GameObject {
    //create GameObject, add its rigidbody, collider into the sets from Master
    pub fn new(bodies: &mut DefaultBodySet<f32>, colliders: &mut DefaultColliderSet<f32>) -> Self{
            //create the necessary isntances for simulation
            let rigidBody = RigidBodyDesc::new().
                mass(10.0).
                build();
        let rigid_body_handle: DefaultBodyHandle = bodies.insert(rigidBody);

        let shape = ShapeHandle::new(Cuboid::new(
                Vector2::new(5.0f32, 5.0)));
        let collider = ColliderDesc::new(shape)
            .density(1.0)
            .build(BodyPartHandle(rigid_body_handle, 0));

        let go = GameObject {
            //give handles to GameObject
            //NOTE: I changed it to actually change the handle which is a reference
            //Previous implementation stored mutable references to the bodies itself
            handleRigidBody: Some(rigid_body_handle),   //insert into set, get handle, save mutable handle
            handleCollider: Some(colliders.insert(collider)),
        };
        return go;
    }

    pub fn update(){

    }

    pub fn draw(&mut self, context: Context){
/*
        //these should later be changed to get the real values out of the player struct
        let x_pos = handleRigidBody.position;
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
        Ok(0)*/
    }
}