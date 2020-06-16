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
    //create boundaries for game world: (left, right, upper, lower)
    create_bound(&mut my_game, Vector2::new(0.0, window_mode.height/2.0), Vector2::new(0.1, window_mode.height));
    create_bound(&mut my_game, Vector2::new(window_mode.width, window_mode.height/2.0), Vector2::new(0.1, window_mode.height));
    create_bound(&mut my_game, Vector2::new(window_mode.width/2.0, 0.0), Vector2::new(window_mode.width, 0.1));
    create_bound(&mut my_game, Vector2::new(window_mode.width/2.0, window_mode.height), Vector2::new(window_mode.width, 0.1));
    
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


//create a boundary for the game world at pos position. The boundary is a cuboid with rigid body and collider
//with the dimensions of size: Vector2 at position pos: Vector2
//the rbs and colliders are then added into the BodySet and ColliderSet of master
fn create_bound(master: &mut Master, pos: Vector2<f32>, size: Vector2<f32>) -> (DefaultBodyHandle, DefaultColliderHandle) {
    let bound_rb = RigidBodyDesc::new()
        .position(Isometry2::new(pos, 0.0))
        .gravity_enabled(false)
        .status(BodyStatus::Kinematic)                      //make rigid body immovable
        .kinematic_translations(Vector2::new(false, false))
        .kinematic_rotations(false)
        .build();
    let bound_rb_handle = master.bodies.insert(bound_rb);
    let bound_shape = ShapeHandle::new(Cuboid::new(size));
    let collider = ColliderDesc::new(bound_shape)
        .density(1.0)
        .material(MaterialHandle::new(BasicMaterial::new(0.4, 0.6)))    //TODO (?) fine tune values
        .margin(0.1f32)
        .user_data(0i8)
        .build(BodyPartHandle(bound_rb_handle, 0));
    let collider_handle = master.colliders.insert(collider);

    return (bound_rb_handle, collider_handle);
}
