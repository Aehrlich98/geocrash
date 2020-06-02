use crate::player::Player;
use crate::game_object::GameObject;
use crate::main::gameSize;
use ggez::*;
use ggez::graphics::Mesh;
use ggez::event::{EventHandler};
use nphysics2d::*;
use ncollide2d::*;
extern crate nalgebra as na;

use na::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc};
use nphysics2d::math::{Velocity, Inertia};
use ncollide2d::shape::{ShapeHandle, Ball};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet ,BodySet, ColliderSet, ColliderDesc};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, ForceGenerator};
use nphysics2d::joint::{DefaultJointConstraintSet, JointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};


pub struct Master{
    mechanical_world: DefaultMechanicalWorld<f32>,    //N/M types are somehow not right??? Maybe give specific types???
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,

    gameObjList: Vec<GameObject>,   //list of all objects in game
    player: Player,
    count: i32,                     //test vraible to only the game run a fixed amount of ticks.
}
//TODO: implement structs Player and Enemy

impl Master{
    pub fn new(&mut ctx: Context) -> Self{
        //create Master with all starting conditions
        Master{
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),

            gameObjList: Vec::new(),
            player: Player::new(),
            count: 0,
        }
    }

    pub fn update(){
        //TODO: update all Players, Enemies and moving objects
        //TODO: remove objects that are out of screen and spawn new ones
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<i8>{
        self.player.draw(context);
        return Ok(0)
    }
}


//EventHandler handling events...
impl EventHandler for Master {
    //called for each step of the game
    //TODO: add full game loop.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.mechanical_world.step(     //move the simulation further one step
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        //temporary: end game after count is reached
        println!("Count: {}", self.count);
        if self.count+1 == 10 {
            event::quit(_ctx);  //End game
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        graphics::present(ctx)
    }
}