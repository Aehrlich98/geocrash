use ggez::*;
use nphysics2d::*;
use ncollide2d::*;

use crate::game_object::GameObject;
use crate::master::Master;

mod game_object;
mod master;
mod player;
mod lib;

static gameSize: i32 = 600;

fn main() {
    // custom WindowMode struct holding size, and original state of the game window.
    let window_mode = WindowMode {
        width: 800.0,
        height: 600.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 800.0,
        max_width: 0.0,
        min_height: 600.0,
        max_height: 0.0,
        resizable: true,
    };
    
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    let mut my_game = Master::new(&mut ctx);

    //spawn starting objects:
    //TODO spawn player(s)
    //spawn other GameObjects
    let pos = (0, 0);
    while my_game.gameObjList.len() < 10{ //place GameObjects, until max number of allowed Object is reached.
        my_game.gameObjList.push(GameObject::new(&mut my_game.bodies, &mut my_game.colliders, window_mode.height, window_mode.width));
        //TODO place at random positions, check that no two objects are placed "into" each other
       // pos.x() + 10;
        //pos.y() + 10;
    }

    /*
    loop {
        my_game.update();
        my_game.draw(&mut ctx);
    }
*/

    //--------------------------------------
    //ggez loop. MUST NOT BE USED UNLESS GGEZ IS USED FOR FRONTEND

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
