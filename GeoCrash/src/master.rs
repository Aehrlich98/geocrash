use ggez::{Context, GameResult};
use crate::player::Player;
use crate::game_object::GameObject;
use ggez::graphics::{Mesh};
use ncollide2d::shape::{ShapeHandle, Cuboid, Ball};
use ggez::nalgebra::Isometry2;
use rand::prelude::*;

pub struct Master{
    objects: Vec<GameObject>,
    player: Player,
    //enemies: Vec<Enemy>,
}
//TODO: implement structs Player and Enemy

impl Master{
    pub fn new() -> Self{
        let mut rng = rand::thread_rng();

        let player1 = ShapeHandle::new(Ball::new(0.5f32));
        let mut objects: Vec<ShapeHandle<f32>> = Vec::new();
        let mut objectsPos = Vec::new();

        for x in 0..10 {
            let obj = ShapeHandle::new(Ball::new(0.2f32));
            objects.push(obj);
            objectsPos.push(Isometry2::new(
                Vector2::new(rng.gen_range(-10.0, 10.0), rng.gen_range(-10.0, 10.0)), na::zero()))
        }

        let player1_pos = Isometry2::new(Vector2::new(5.0, 5.0), na::zero());

        //TODO: add collision groups

        Master{
            objects: Vec::new(),
            player: Player::new(),
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