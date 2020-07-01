use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

extern crate nalgebra as na;

use na::{Vector2, Isometry2};
use na::geometry::Point2;

use nphysics2d::object::{BodyStatus, RigidBodyDesc, Collider, DefaultBodyHandle, Body};
use nphysics2d::math::{Velocity, Inertia};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, ColliderDesc, BodyPartHandle};
use nphysics2d::force_generator::{DefaultForceGeneratorSet};
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{ShapeHandle, Cuboid, Ball, ConvexPolygon};
use ggez::graphics::{DrawParam, Color};
use std::f32::consts::PI;
use ggez::conf::Conf;
use ggez::mint::Point2 as GGEZ_Point2;  //use alias, as nalgebra::geometry::Point2 is also required for ConvexPolygon shapecreation at line  ff.

use crate::master;
use crate::constants;
use nphysics2d::algebra::{Force2, ForceType};
use rand::Rng;
use ggez::nalgebra::UnitComplex;

pub struct GameObject {
    ready: i8,
    leftgrav: bool,
    pub owned_by: i8,
    handleRigidBody: Option<DefaultBodyHandle>,    //mutable handles
    pub handleCollider: Option<DefaultBodyHandle>,
    registeredPlayerID: Option<i8>,
    pub id: i8,
    color: graphics::Color,
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

        let rb_handle = create_rigid_body(position, id, bodies);
        let col_handle = create_collider(rb_handle, id, colliders);

        let go = GameObject {
            //give handles to GameObject
            ready: 0,
            leftgrav: true,
            owned_by: 6,
            handleRigidBody: Some(rb_handle),   //insert into set, get handle, save mutable handle
            handleCollider: Some(col_handle),
            registeredPlayerID: None, 
            id: id,
            color: constants::DEFAULT_COLOR,
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
        //println!("force applied");
        
    }

    pub fn timer(&mut self) {
        
        if self.ready > 0 && self.leftgrav {
            self.ready -= 1;
        } else if self.ready == 0 {
            self.owned_by = constants::DEFAULT_ID;
            self.color = constants::DEFAULT_COLOR;
        }
        
    }

    pub fn registerPlayer(&mut self, id: i8){
        //only register player if no player is registered
        if self.registeredPlayerID == None && self.ready == 0 {
            self.registeredPlayerID = Some(id);
            if id == constants::PLAYER1_ID {
                self.color = constants::PLAYER1_COLOR;
            } else {
                self.color = constants::PLAYER2_COLOR;
            }
            self.owned_by = id;
            self.ready = 1;
            self.leftgrav = false;
        }
    }

    pub fn deregisterPlayer(&mut self, id: i8){
        if self.registeredPlayerID == Some(id) {
            self.registeredPlayerID = None;
            if self.ready > 0 {
                
                self.ready = 100;
                println!("Left Player {}",self.ready);
                self.leftgrav = true;
            }
        }

    }

    

    pub fn draw(&self, context: &mut Context, bodies: &mut DefaultBodySet<f32>) -> GameResult<i8>{
        let rb_handle = self.handleRigidBody.unwrap();
        let rb = bodies.rigid_body(rb_handle).unwrap();

        //println!("drawing");
        let position: &Isometry2<f32> = rb.position();
        let x :f32 = position.translation.vector.get(0).unwrap().clone();
        let y :f32 = position.translation.vector.get(1).unwrap().clone();
        let angle: f32 = position.rotation.angle().clone();

        let r2;
        let polygon;
        //create polygon in shape of triangle
        if self.id%2 ==0{
            polygon = vec![
                /*
                    set up points in clockwise order, use the power of math to calculate rotation of shapes depending on given rotation angle of collider:
                    px' = pox + (px - pox)*cos(angle) - (py - poy)*sin(angle), py' = poy + (px - pox)*sin(angle) + (py - poy)*cos(angle);
                */
                GGEZ_Point2{x: (x+ (x+20.0 -x) * position.rotation.cos_angle() - (y-20.0 -y) *position.rotation.sin_angle()), y: (y+ (x+20.0 -x)*position.rotation.sin_angle() + (y-20.0 -y)*position.rotation.cos_angle())},      //TODO convert from size of rigidbody to make fitting size
                GGEZ_Point2{x: (x+ (x-20.0 -x) * position.rotation.cos_angle() - (y-20.0 -y) *position.rotation.sin_angle()), y: (y+ (x-20.0 -x)*position.rotation.sin_angle() + (y-20.0 -y)*position.rotation.cos_angle())},
                GGEZ_Point2{x: (x+ (x -x) * position.rotation.cos_angle() - (y+20.0 -y) *position.rotation.sin_angle()), y: (y+ (x -x)*position.rotation.sin_angle() + (y+20.0 -y)*position.rotation.cos_angle())},
            ];
        } else{ //... or in default case as square
            polygon = vec![
                GGEZ_Point2{x: (x+ (x+20.0 -x) * position.rotation.cos_angle() - (y-20.0 -y) *position.rotation.sin_angle()), y: (y+ (x+20.0 -x)*position.rotation.sin_angle() + (y-20.0 -y)*position.rotation.cos_angle())},      //TODO convert from size of rigidbody to make fitting size
                GGEZ_Point2{x: (x+ (x+20.0 -x) * position.rotation.cos_angle() - (y+20.0 -y) *position.rotation.sin_angle()), y: (y+ (x+20.0 -x)*position.rotation.sin_angle() + (y+20.0 -y)*position.rotation.cos_angle())},
                GGEZ_Point2{x: (x+ (x-20.0 -x) * position.rotation.cos_angle() - (y+20.0 -y) *position.rotation.sin_angle()), y: (y+ (x-20.0 -x)*position.rotation.sin_angle() + (y+20.0 -y)*position.rotation.cos_angle())},
                GGEZ_Point2{x: (x+ (x-20.0 -x) * position.rotation.cos_angle() - (y-20.0 -y) *position.rotation.sin_angle()), y: (y+ (x-20.0 -x)*position.rotation.sin_angle() + (y-20.0 -y)*position.rotation.cos_angle())},
            ];
        }
        r2 = graphics::Mesh::new_polygon(context, graphics::DrawMode::fill(), &polygon,
                                         self.color)?;

        ggez::graphics::draw(context, &r2, DrawParam::default())?;
        Ok(0)
    }
}
fn create_rigid_body(position: Isometry2<f32>, id: i8, bodies: &mut DefaultBodySet<f32>) -> DefaultBodyHandle{
    //create the necessary isntances for simulation
    let mut rigidBody = RigidBodyDesc::new()
        .mass(1.0)                     //mass, setting the inertia against acceleration
        .position(position)             //starting position of body
        .angular_inertia(1.0)          //setting the bodies inertia against angular motion
        .enable_gravity(false)
        .build();
    rigidBody.set_status(BodyStatus::Dynamic);
    rigidBody.set_linear_damping(0.1);
    rigidBody.set_angular_damping(0.1);
    rigidBody.set_user_data(Some(Box::new(id)));
    bodies.insert(rigidBody)
}

fn create_collider(rb_handle: DefaultBodyHandle, id: i8, colliders: &mut DefaultColliderSet<f32>) -> DefaultBodyHandle{
    let shape;

    //create both triangle and square shapes in equal numbers.
    if id%2 ==0 {
        let points = vec![     //describe triangle
                                Point2::new(1.0, -1.0),    //TODO check size of collider vs draw()
                                Point2::new(0.0, 1.0),
                                Point2::new(-1.0, -1.0),
        ];
        shape = ShapeHandle::new(ConvexPolygon::try_new(points).expect("faulty creation of complex polygon triangle"));
    }else {
        let points = vec![     //describe square
                                Point2::new(1.0, -1.0),  //TODO as above, find a scaling transaltion between collider size and this points
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
        .user_data(id)
        .build(BodyPartHandle(rb_handle, 0));
    colliders.insert(collider)
}
