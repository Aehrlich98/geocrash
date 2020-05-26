use ggez::graphics::{DrawParam, BlendMode, Mesh};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

pub struct Player{
    //TODO: implement player attributes
}

/*impl Drawable for Player{

    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameState{
        let rect = graphics::Rect::new(100.0, 100.0, 50.0, 50.0);
        return rect;
    }

    fn dimensions(&self, ctx: &mut Context){

    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>){

    }

    fn blend_mode(&self){

    }
}*/

impl Player{

    pub fn new() -> Self {
        //TODO: create a new player in the center of the screen
        Player{}
    }

    pub fn update(){
        //TODO: update player
    }

    pub fn draw(&self, context: &mut Context) -> GameResult<Mesh>{

        let rect = graphics::Rect::new(100f32, 100f32, 50f32, 50f32);
        let r1 =
            graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(),
                                          rect, graphics::BLACK)?;
        return GameResult::Ok(r1);
    }

