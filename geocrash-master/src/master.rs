use crate::player::Player;
use crate::game_object::GameObject;
use ggez::*;
use ggez::graphics::Mesh;
use ggez::event::{EventHandler};
use ggez::conf::WindowMode;
use nphysics2d::*;
use ncollide2d::*;
extern crate nalgebra as na;
use rand::prelude::*;


use na::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, BodyPartHandle};
use nphysics2d::math::Velocity;
use nphysics2d::math::Inertia;
use ncollide2d::shape::{ShapeHandle, Ball};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator, DefaultForceGeneratorHandle, ConstantAcceleration};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::pipeline::CollisionWorld;
use nphysics2d::algebra::ForceType::Force;
use nphysics2d::algebra::{Force2, ForceType};
use crate::constants;
use std::collections::HashMap;
use std::thread::spawn;
use ncollide2d::query::Proximity;


pub struct Master{
    window_mode: WindowMode,                            //holds data about window size
    
    mechanical_world: DefaultMechanicalWorld<f32>,    //N/M types are somehow not right??? Maybe give specific types???
    geometrical_world: DefaultGeometricalWorld<f32>,

    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,

    pub gameObjList: HashMap<i8, GameObject>,   //list of all objects in game
    pub player1: Player,
    pub player2: Player,
    count: i32,                     //test vraible to only the game run a fixed amount of ticks.

}

impl Master{
    pub fn new(ctx: &mut Context, window_mode: WindowMode) -> Self{
        
        let mut force_generators = DefaultForceGeneratorSet::new();

        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();


        let mut player1 = Player::new(true);
        //init player
        player1.createRigidBody( &mut bodies);
        player1.create_collider(&mut colliders);

        let mut player2 = Player::new(false);
        //init player
        player2.createRigidBody( &mut bodies);
        player2.create_collider(&mut colliders);

        let mut master = Master{
            window_mode,
             
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: bodies,
            colliders: colliders,
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: force_generators,

            gameObjList: HashMap::with_capacity(20),
            player1: player1,
            player2: player2,
            count: 0,
        };
        master.spawn_game_objects();
        return master;
    }

    fn spawn_game_objects(&mut self){
        let mut id = 50i8;
        while self.gameObjList.len() < 20{ //place GameObjects, until max number of allowed Object is reached.
            self.gameObjList.insert(id, GameObject::new(&mut self.bodies, id, &mut self.colliders, self.window_mode.height, self.window_mode.width));
            id += 1;
        }
    }
}

//EventHandler handling events...
impl EventHandler for Master {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.player1.update(_ctx, &mut self.bodies, &mut self.force_generators);
        self.player2.update(_ctx, &mut self.bodies, &mut self.force_generators);

         //get player position
        let player1_pos: Isometry2<f32> = self.colliders.get(self.player1.collider_handle.unwrap()).unwrap().position().clone();
        let player2_pos: Isometry2<f32> = self.colliders.get(self.player2.collider_handle.unwrap()).unwrap().position().clone();



        for (_, go) in &mut self.gameObjList{
            go.update(&mut self.colliders, &mut self.bodies, &player1_pos, &player2_pos);
        };
        self.mechanical_world.step(     //move the simulation further one step
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        for proximity in self.geometrical_world.proximity_events(){
            //handle proximity events
            println!("Proximity detected");
            let data1 = self.colliders.get(proximity.collider1).unwrap().user_data().unwrap();
            let data2= self.colliders.get(proximity.collider2).unwrap().user_data().unwrap();
            let id1 = data1.downcast_ref::<i8>().unwrap();
            let id2 = data2.downcast_ref::<i8>().unwrap();

            //case player 1 is close to game object
            if *id1 == constants::PLAYER1_ID && *id2 >= 50 && *id2 <= 100 || *id1 >= 50 && *id1 <= 100 && *id2 == constants::PLAYER1_ID{
                println!("Player is close to a game object");
                //register player at game object
                let go_id = match *id1 {
                    constants::PLAYER1_ID => *id2,
                    _ => *id1,
                };

                if proximity.new_status == Proximity::Disjoint{
                    self.gameObjList.get_mut(&go_id).unwrap().deregisterPlayer(constants::PLAYER1_ID);
                }
                else {
                    self.gameObjList.get_mut(&go_id).unwrap().registerPlayer(constants::PLAYER1_ID);
                }

            }
        }

        for contact in self.geometrical_world.contact_events(){
            println!("Contact happened");
        }

        if self.count+1 == 10 {
            event::quit(_ctx);  //End game
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.player1.draw(ctx, &mut self.bodies);
        self.player2.draw(ctx, &mut self.bodies);

        for (_, go) in &self.gameObjList{
            go.draw(ctx, &mut self.bodies);
        }
        graphics::present(ctx)
    }
}
