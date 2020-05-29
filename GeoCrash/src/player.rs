use ggez::graphics::{DrawParam, BlendMode, Mesh};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;

pub struct Player{
    //TODO: implement player attributes
}

impl Player {
    pub fn new() -> Self {
        //TODO: create a new player in the center of the screen
        Player {}
    }

    pub fn update() {
        //TODO: update player
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<i8>{

        //these should later be changed to get the real values out of the player struct
        let x_pos = 200f32;
        let y_pos = 200f32;
        let radius = 50f32;
        let tolerance = 0.001f32;
        //--
        let p: Point2<f32> =  Point2{
            x: x_pos,
            y: y_pos,
        };

        let r2 = graphics::Mesh::new_circle(context, graphics::DrawMode::fill(), p,
            radius, tolerance, graphics::BLACK)?;
        graphics::draw(context, &r2, DrawParam::default())?;
        Ok(0)
    }
}

