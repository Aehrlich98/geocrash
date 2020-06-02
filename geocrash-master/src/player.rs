use ggez::graphics::{DrawParam, BlendMode, Mesh};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;

use crate::master::Master;
use crate::main::gameSize;
use crate::game_object::GameObject;

pub struct Player{
    //TODO: implement player attributes
    score: i32,
    threshold: i32,     //min force required to "kill" the player
    playerBody: GameObject, //gameObject associated with the player
}

impl Player {
    pub fn new(my_game: Master) -> Self {
        //TODO: create a new player in the center of the screen
        Player {
            score: 0,
            threshold: 100, //TODO fine tune values
            playerBody: GameObject::new(Point2::new(gameSize/2, gameSize/2), Master::bodies, Master::colliders),
        }
    }

    pub fn update() {
        //TODO: update player
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<i8>{

        //these should later be changed to get the real values out of the player struct
        let x_pos = 200f32;
        let y_pos = 200f32;
        let radius = 30f32;
        let tolerance = 0.00001f32;
        //--
        let p: Point2<f32> =  Point2{
            x: x_pos,
            y: y_pos,
        };

        let r2 = graphics::Mesh::new_circle(context, graphics::DrawMode::fill(), p,
            radius, tolerance, graphics::Color::new(0.7, 0.4, 0.9, 0.8))?;
        graphics::draw(context, &r2, DrawParam::default())?;
        Ok(0)
    }
}
