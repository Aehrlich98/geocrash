use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

extern crate nalgebra as na;

use na::{Vector2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, Collider, DefaultBodyHandle};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc, BodyPartHandle};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Cuboid};
use ggez::graphics::DrawParam;
use ggez::mint::Point2;
//OUT type Point = (i32, i32);

pub struct GameObject {
    handleRigidBody: Option<DefaultBodyHandle>,    //mutable handles
    handleCollider: Option<DefaultBodyHandle>,

}

impl GameObject {
    //create GameObject, add its rigidbody, collider into the sets from Master
    pub fn new(bodies: &mut DefaultBodySet<f32>, colliders: &mut DefaultColliderSet<f32>) -> Self{
            //create the necessary isntances for simulation
            let rigidBody = RigidBodyDesc::new().
                mass(10.0).
                build();
        let rb_handle = bodies.insert(rigidBody);

        let shape = ShapeHandle::new(Cuboid::new(
                Vector2::new(5.0f32, 5.0)));
        let collider = ColliderDesc::new(shape).
            density(1.0).
            build(BodyPartHandle(rb_handle, 0));
        let col_handle = colliders.insert(collider);

        let go = GameObject {
            //give handles to GameObject
            handleRigidBody: Some(rb_handle),   //insert into set, get handle, save mutable handle
            handleCollider: Some(col_handle),
        };

        return go;
    }

    pub fn update(){

    }

    pub fn draw(&self, context: &mut Context, bodies: &mut DefaultBodySet<f32>) -> GameResult<i8>{
        let rb_handle = self.handleRigidBody.unwrap();
        let rb = bodies.rigid_body(rb_handle).unwrap();

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
            radius, tolerance, graphics::Color::new(0.7, 0.4, 0.9, 0.8))?;
        graphics::draw(context, &r2, DrawParam::default())?;
        Ok(0)
    }

}