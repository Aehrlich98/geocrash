use ggez::{Context, GameResult};
use crate::player::Player;
use crate::game_object::GameObject;
use ggez::graphics::Mesh;

pub struct Master{
    objects: Vec<GameObject>,
    player: Player,
    //enemies: Vec<Enemy>,
}
//TODO: implement structs Player and Enemy

impl Master{
    pub fn new() -> Self{
        //TODO: initialize Master with all Objects
        Master{
            objects: Vec::new(),
            player: Player::new(),
        }
    }

    pub fn update(){
        //TODO: update all Players, Enemies and moving objects
        //TODO: remove objects that are out of screen and spawn new ones
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<Mesh>{
        return self.player.draw(context);
    }
}