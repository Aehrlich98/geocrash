use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

extern crate nalgebra as na;

use na::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Cuboid};

//OUT type Point = (i32, i32);

pub struct GameObject {
    rigidBody: RigidBody<f32>,
    shape: ShapeHandle<f32>,
    collider: Collider<f32>,

}

impl GameObject {
    //create GameObject, add its rigidbody, collider into the sets from Master
    pub fn new(&self, pos: Point2<i32>, bodies: DefaultBodySet<f32>, colliders: DefaultColliderSet<f32>) -> Self{
       let go = GameObject {
            rigidBody: RigidBodyDesc::new().
                mass(10.0).
                build(),
            shape: ShapeHandle::new(Cuboid::new(
                Vector2::new(2.0f32, 1.0))),
            collider: ColliderDesc::new().
                build(
                BodyParentHandle(self.rigidBody, 0)),
        };
        let handleRB = bodies.insert(go.rigidBody);
    }

    pub fn update(){

    }

    pub fn draw(&mut self, context: Context){
        //TODO: render Object at current position
    }
}