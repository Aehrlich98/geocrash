use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

extern crate nalgebra as na;

use na::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, Collider, DefaultBodyHandle};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc, BodyPartHandle};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Cuboid};
use ggez::graphics::DrawParam;

//OUT type Point = (i32, i32);

pub struct GameObject {
    handleRigidBody: Option<DefaultBodyHandle>,    //mutable handles
    handleCollider: Option<DefaultBodyHandle>,

}

impl GameObject {
    //create GameObject, add its rigidbody, collider into the sets from Master
    pub fn new(&self, pos: Point2<i32>, bodies: DefaultBodySet<f32>, colliders: DefaultColliderSet<f32>) -> Self{
            //create the necessary isntances for simulation
            let rigidBody = RigidBodyDesc::new().
                mass(10.0).
                build();
            let shape = ShapeHandle::new(Cuboid::new(
                Vector2::new(5.0f32, 5.0)));
            let collider = ColliderDesc::new(shape).
                density(1.0).
                build(
                BodyPartHandle(self.rigidBody, 0));
        let go = GameObject {
            //give handles to GameObject
            handleRigidBody: colliders.get_mut(bodies.insert(rigidBody)),   //insert into set, get handle, save mutable handle
            handleCollider: colliders.get_mut(colliders.insert(collider)),
        };
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
        Ok(0)
    */
    }

}