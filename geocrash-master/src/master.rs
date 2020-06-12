use crate::player::Player;
use crate::game_object::GameObject;
use ggez::*;
use ggez::graphics::Mesh;
use ggez::event::{EventHandler};
use nphysics2d::*;
use ncollide2d::*;
extern crate nalgebra as na;
use rand::prelude::*;


use na::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, BodyPartHandle};
use nphysics2d::math::{Velocity, Inertia};
use ncollide2d::shape::{ShapeHandle, Ball};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator, DefaultForceGeneratorHandle, ConstantAcceleration};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::pipeline::CollisionWorld;


pub struct Master{
    mechanical_world: DefaultMechanicalWorld<f32>,    //N/M types are somehow not right??? Maybe give specific types???
    geometrical_world: DefaultGeometricalWorld<f32>,

    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,

    pub gameObjList: Vec<GameObject>,   //list of all objects in game
    pub player: Player,
    count: i32,                     //test vraible to only the game run a fixed amount of ticks.

    //control accelerations - are applied to the player, when a certain key is pressed (for example
    //left arrow key for left_acc

}
//TODO: implement structs Player and Enemy

impl Master{
    pub fn new(ctx: &mut Context) -> Self{


        let mut force_generators = DefaultForceGeneratorSet::new();

        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();


        let mut player = Player::new();
        //init player
        player.createRigidBody( &mut bodies);
        player.create_collider(&mut colliders);

        let mut master = Master{
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: bodies,
            colliders: colliders,
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: force_generators,

            gameObjList: Vec::new(),
            player: player,
            count: 0,
        };
        return master;
    }


    pub fn draw(&mut self, context: &mut Context) -> GameResult<i8>{
        self.player.draw(context, &mut self.bodies);
        return Ok(0)
    }
}


//EventHandler handling events...
impl EventHandler for Master {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.player.update(_ctx, &mut self.bodies, &mut self.force_generators);

        self.mechanical_world.step(     //move the simulation further one step
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        //temporary
        //println!("Count: {}", self.count);
        if self.count+1 == 10 {
            event::quit(_ctx);  //End game
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.player.draw(ctx, &mut self.bodies);

        /*for go in &self.gameObjList{
            go.draw(ctx, &mut self.bodies);
        }*/
        graphics::present(ctx)
    }
}