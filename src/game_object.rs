use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

extern crate nalgebra as na;

use na::{Vector2, Point2, Isometry2};
use ggez::graphics::{DrawParam, BlendMode, Mesh};
use nphysics2d::object::{BodyStatus, RigidBodyDesc};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc, BodyPartHandle, DefaultBodyHandle, RigidBody, Collider};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Cuboid};

//OUT type Point = (i32, i32);

pub struct GameObject {
    handleRigidBody: DefaultBodyHandle,    //handles (unmutable)
    handleCollider: DefaultBodyHandle,
    isPlayer: bool,

}

impl GameObject {
    //create GameObject, add its rigidbody, collider into the sets from Master
    pub fn new(pos: Point2<i32>, bodies: DefaultBodySet<f32>, colliders: DefaultColliderSet<f32>, isPlayer: bool) -> Self{
            //create the necessary isntances for simulation
            let rigidBody = RigidBodyDesc::new().
                mass(10.0).
                build();
            let parent_handle = bodies.insert(rigidBody);

            let shape = ShapeHandle::new(Cuboid::new(
                Vector2::new(5.0f32, 5.0)));
            let collider = ColliderDesc::new(shape).
                density(1.0).
                build(
                BodyPartHandle(parent_handle, 0));
        GameObject {
            //give handles to GameObject
            handleRigidBody: parent_handle,   //save handles
            handleCollider: colliders.insert(collider),
            isPlayer: isPlayer,
        } //return go
    }

    pub fn update(&self){

    }

    pub fn draw(&mut self, context: Context) -> GameResult<i8>{

        //these should later be changed to get the real values out of the player struct
        let pos = (0,0); //Point2<f32>
        let radius = 10f32; //TODO everything!
        let tolerance = 5f32;

        let r2 = graphics::Mesh::new_circle(&mut context, graphics::DrawMode::fill(), pos,
            radius, tolerance, graphics::Color::new(0.7, 0.4, 0.9, 0.8));
        graphics::draw(&mut context, &r2, DrawParam::default());
        Ok(0)
    }
}