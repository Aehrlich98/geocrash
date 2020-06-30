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
use nphysics2d::algebra::ForceType::Force;
use nphysics2d::algebra::{Force2, ForceType};
use ggez::conf::WindowMode;

pub const PLAYER_ID: i8 = 1;
pub const GAME_OBJECT_ID: i8 = 2;

pub struct Master{
    pub(crate) window_mode: WindowMode,                            //holds data about window size

    mechanical_world: DefaultMechanicalWorld<f32>,    //N/M types are somehow not right??? Maybe give specific types???
    geometrical_world: DefaultGeometricalWorld<f32>,

    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,

    pub gameObjList: Vec<GameObject>,   //list of all objects in game
    pub player: Player,

}

impl Master{
    pub fn new(ctx: &mut Context, window_mode: WindowMode) -> Self{

        let mut force_generators = DefaultForceGeneratorSet::new();

        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();


        let mut player = Player::new();
        //init player
        player.createRigidBody( &mut bodies);
        player.create_collider(&mut colliders);

        let mut master = Master{
            window_mode,

            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: bodies,
            colliders: colliders,
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: force_generators,

            gameObjList: Vec::new(),
            player,
        };
        return master;
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

        for proximity in self.geometrical_world.proximity_events(){
            //handle proximity events
            println!("Proximity detected");
            let data1 = self.colliders.get(proximity.collider1).unwrap().user_data().unwrap();
            let data2= self.colliders.get(proximity.collider2).unwrap().user_data().unwrap();
            let id1 = data1.downcast_ref::<i8>().unwrap();
            let id2 = data2.downcast_ref::<i8>().unwrap();
            if *id1 == PLAYER_ID && *id2 == GAME_OBJECT_ID || *id1 == GAME_OBJECT_ID && *id2 == PLAYER_ID{

                /*TODO: there on, we need a repeating impulse applied to the gameobject pulling it towards
                the player. Right now, the game object gets a one time impulse. I suggest registering the
                player in all close gameobjects and implementing an update method for game objects,
                where that force is applied.
                */
                println!("Player is close to a game object");

                let c = match *id2 {
                    GAME_OBJECT_ID => proximity.collider2,
                    _ => proximity.collider1,
                };
                let p = match *id1 {
                    PLAYER_ID => proximity.collider1,
                    _ => proximity.collider2,
                };
                //get player position
                let player_pos: &Isometry2<f32> = self.colliders.get(p).unwrap().position();
                let player_vec = &player_pos.translation.vector;

                let go_pos : &Isometry2<f32> = self.colliders.get(c).unwrap().position();
                let go_vec = &go_pos.translation.vector;

                //we have the position of the player and the  game object -> calc player - game object to get force vector
                let force_vec = Vector2::new(10f32*(player_vec.get(0).unwrap() - go_vec.get(0).unwrap()), 10f32*(player_vec.get(1).unwrap() - go_vec.get(1).unwrap()));
                let f = Force2::new(force_vec, 0.0f32);

                //apply force to game object
                let mut object = self.bodies.get_mut(self.colliders.get(c).unwrap().body()).unwrap();
                object.apply_force(0, &f, ForceType::Impulse, true);
                println!("force applied");

            }
        }

        for contact in self.geometrical_world.contact_events(){
            println!("Contact happened");
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.player.draw(ctx, &mut self.bodies);

        for go in &self.gameObjList{
            go.draw(ctx, &mut self.bodies);
        }
        graphics::present(ctx)
    }
}