use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

use crate::game_object::GameObject;
use crate::master::Master;

mod game_object;
mod master;
mod player;
mod lib;

static gameSize: Int = 600;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    let mut my_game = Master::new(&mut ctx);//.unwrap();

    //spawn starting objects:
    //TODO spawn player(s)
    //spawn other GameObjects
    let pos = (0, 0);
    while my_game.gameObjList.length < 1{ //place GameObjects, until max number of allowed Object is reached.
        my_game.gameObjList.push(GameObject::new(pos, my_game.bodies, my_game.colliders));
        //TODO place at random positions, check that no two objects are placed "into" each other
       // pos.x() + 10;
        //pos.y() + 10;
    }

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
