use ggez::*;
use nphysics2d::*;
use ncollide2d::*;
use ggez::event::{self, EventHandler};

extern crate nalgebra as na;

use na::Vector3;
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, RigidBodyDesc};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::*;
use ncollide2d::shape::{ShapeHandle, Ball};
use ggez::graphics::DrawParam;
use crate::game_object::GameObject;
use crate::master::Master;

mod game_object;
mod master;
mod player;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    let mut my_game = MyGame::new(&mut ctx);

//Create game objects:
    //put a rigidbody into bodies BodySet of my_game and push to gamObjList
    /*gameObjList.push( my_game.bodies.insert(RigidBodyDesc::new()
            .translation(Vector2::x() * 2.0)
            .mass(10.0)
            .build()
        )
    );
    //create collider for rigidbody:
    let shape = ShapeHandle::new(Ball::new(2.0)); //Traingle??


*/
    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    /*mechanical_world: DefaultMechanicalWorld<N>,    //N/M types are somehow not right??? Maybe give specific types???
    geometrical_world: DefaultGeometricalWorld<N>,
    bodies: DefaultBodySet<M>,
    colliders: DefaultColliderSet<M>,
    joint_constraints: DefaultJointConstraintSet<M>,
    force_generators: DefaultForceGeneratorSet<M>,*/
    master: Master,
    gameObjList: Vec<GameObject>,   //list of all object in game
    count: i32,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
           /* mechanical_world: DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),*/

            gameObjList: Vec::new(),
            master: Master::new(),
            count: 0,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        /*mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators
        );


        println!("Count: {}", count);
        if count+1 == 10 {
            event::quit(_ctx);  //End game
        }*/
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let r1 = self.master.draw(ctx);
        graphics::present(ctx)
    }
}