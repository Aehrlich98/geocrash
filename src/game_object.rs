use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

extern crate nalgebra as na;

use na::{Vector2, Isometry2};
use na::geometry::Point2;
use nphysics2d::object::{BodyStatus, RigidBodyDesc, Collider, DefaultBodyHandle, Body};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc, BodyPartHandle};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Cuboid, Ball, Shape, ConvexPolygon};
use ggez::graphics::{DrawParam, Color};
use ggez::mint::Point2 as GGEZ_Point2;  //use alias, as nalgebra::geometry::Point2 is also required for ConvexPolygon shapecreation at line 54 ff.
use rand::Rng;
use std::f32::consts::PI;
use ggez::conf::Conf;
use crate::master;

pub struct GameObject {
    handleRigidBody: Option<DefaultBodyHandle>,     //handles for rigid body/ collider
    handleCollider: Option<DefaultBodyHandle>,
    shape_id: i8,                                  //describe the shape this object has: 0 == triangle, 1 == square
}

impl GameObject {
    //create GameObject, add its rigidbody, collider into the sets from Master
    pub fn new(bodies: &mut DefaultBodySet<f32>, colliders: &mut DefaultColliderSet<f32>, shape_id: i8, right_bound: f32, bottom_bound: f32) -> Self{

        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(0.0, right_bound);
        println!("x_pos: {}", x_pos);
        let y_pos = rng.gen_range(0.0, bottom_bound);

        let position = Isometry2::new(Vector2::new(x_pos, y_pos), PI);
            //create the necessary instances for simulation
        let mut rigidBody = RigidBodyDesc::new()
            .mass(10.0)                     //mass, setting the inertia against acceleration
            .position(position)             //starting position of body
            .angular_inertia(10.0)          //setting the bodies inertia against angular motion
            .enable_gravity(false)//set if gravity can affect this body
            .build();
        rigidBody.set_status(BodyStatus::Dynamic);
        rigidBody.set_linear_damping(0.1);          //minimal damping for MAXIMUM FUN
        rigidBody.set_angular_damping(0.1);
        rigidBody.set_user_data(Some(Box::new(master::GAME_OBJECT_ID)));
        let rb_handle = bodies.insert(rigidBody);

        //create Shape depending on shape_id
        //defualt square
        let shape;
        if shape_id == 0 {
            let points = vec![     //describe triangle
                                   Point2::new(1.0, -1.0),
                                   Point2::new(0.0, 1.0),
                                   Point2::new(-1.0, -1.0),
            ];
            shape = ShapeHandle::new(ConvexPolygon::try_new(points).expect("faulty creation of complex polygon triangle"));
        }else {
            let points = vec![     //describe square
                                       Point2::new(1.0, -1.0),  //TODO check "massstab" of collider vs draw()
                                       Point2::new(1.0, 1.0),
                                       Point2::new(-1.0, 1.0),
                                       Point2::new(-1.0, -1.0),
            ];
            shape = ShapeHandle::new(ConvexPolygon::try_new(points).expect("faulty creation of complex polygon square"));
        }

        let collider = ColliderDesc::new(shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(0.4, 0.6)))
            .margin(8f32)
            .user_data(master::GAME_OBJECT_ID)
            .build(BodyPartHandle(rb_handle, 0));
        let col_handle = colliders.insert(collider);

        let go = GameObject {
            //give handles to GameObject
            handleRigidBody: Some(rb_handle),
            handleCollider: Some(col_handle),
            shape_id,
        };

        return go;
    }

    pub fn update(){

    }

    pub fn draw(&self, context: &mut Context, bodies: &mut DefaultBodySet<f32>) -> GameResult<i8>{
        let rb_handle = self.handleRigidBody.unwrap();
        let rb = bodies.rigid_body(rb_handle).unwrap();

        //println!("drawing");
        let position: &Isometry2<f32> = rb.position();
        let x :f32 = position.translation.vector.get(0).unwrap().clone();
        let y :f32 = position.translation.vector.get(1).unwrap().clone();

        let angle: f32 = position.rotation.angle().clone();  //TODO

        //println!("{}", angle);

        let p: GGEZ_Point2<f32> =  GGEZ_Point2{
            x,
            y,
        };

        let r2;
        if self.shape_id == 0 {     //triangle  //TODO implement stuff from square for triangles
            /*let radius = 10f32;
            let tolerance = 0.00001f32;
            //--

            r2 = graphics::Mesh::new_circle(context, graphics::DrawMode::fill(), p, radius, tolerance, graphics::Color::new(0.0, 1.0, 1.0, 0.90))?;
            */

            let polygon = vec![
                GGEZ_Point2{x: (x+ (x+20.0 -x) * position.rotation.cos_angle() - (y-20.0 -y) *position.rotation.sin_angle()), y: (y+ (x+20.0 -x)*position.rotation.sin_angle() + (y-20.0 -y)*position.rotation.cos_angle())},      //TODO convert from size of rigidbody to make fitting size
                GGEZ_Point2{x: (x+ (x-20.0 -x) * position.rotation.cos_angle() - (y-20.0 -y) *position.rotation.sin_angle()), y: (y+ (x-20.0 -x)*position.rotation.sin_angle() + (y-20.0 -y)*position.rotation.cos_angle())},
                GGEZ_Point2{x: (x+ (x -x) * position.rotation.cos_angle() - (y+20.0 -y) *position.rotation.sin_angle()), y: (y+ (x -x)*position.rotation.sin_angle() + (y+20.0 -y)*position.rotation.cos_angle())},
            ];

            r2 = graphics::Mesh::new_polygon(context, graphics::DrawMode::fill(), &polygon,
                                             graphics::Color::new(0.0, 1.0, 1.0, 0.90))?;
        } else {                     //default: square  /TODO fit size of square to colliders
           let polygon = vec![
                GGEZ_Point2{x: (x+ (x+20.0 -x) * position.rotation.cos_angle() - (y-20.0 -y) *position.rotation.sin_angle()), y: (y+ (x+20.0 -x)*position.rotation.sin_angle() + (y-20.0 -y)*position.rotation.cos_angle())},      //TODO convert from size of rigidbody to make fitting size
                GGEZ_Point2{x: (x+ (x+20.0 -x) * position.rotation.cos_angle() - (y+20.0 -y) *position.rotation.sin_angle()), y: (y+ (x+20.0 -x)*position.rotation.sin_angle() + (y+20.0 -y)*position.rotation.cos_angle())},
                GGEZ_Point2{x: (x+ (x-20.0 -x) * position.rotation.cos_angle() - (y+20.0 -y) *position.rotation.sin_angle()), y: (y+ (x-20.0 -x)*position.rotation.sin_angle() + (y+20.0 -y)*position.rotation.cos_angle())},
                GGEZ_Point2{x: (x+ (x-20.0 -x) * position.rotation.cos_angle() - (y-20.0 -y) *position.rotation.sin_angle()), y: (y+ (x-20.0 -x)*position.rotation.sin_angle() + (y-20.0 -y)*position.rotation.cos_angle())},
           ];

            r2 = graphics::Mesh::new_polygon(context, graphics::DrawMode::fill(), &polygon,
                graphics::Color::new(0.0, 1.0, 1.0, 0.90))?;
        }
        graphics::draw(context, &r2, DrawParam::default())?;
        Ok(0)
    }

}