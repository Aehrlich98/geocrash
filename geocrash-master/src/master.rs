use crate::player::Player;
use crate::game_object::GameObject;
use ggez::graphics::{Mesh, DrawParam};
use ggez::event::{KeyCode,EventHandler};
use ggez::conf::WindowMode;
use nphysics2d::*;
extern crate nalgebra as na;
use rand::prelude::*;


use na::{Vector2, Point2, Isometry2};
use nphysics2d::object::{BodyStatus, RigidBodyDesc, BodyPartHandle};
use ncollide2d::shape::{ShapeHandle, Ball};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, ColliderDesc};
use nphysics2d::force_generator::{DefaultForceGeneratorSet, DefaultForceGeneratorHandle, ConstantAcceleration};
use nphysics2d::joint::{DefaultJointConstraintSet};
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::pipeline::CollisionWorld;
use nphysics2d::algebra::{Force2, ForceType};
use crate::{constants, create_bound};
use main;
use std::collections::HashMap;
use std::thread::spawn;
use ncollide2d::query::Proximity;
use ggez::{Context, GameResult, event, graphics};
use crate::constants::GAME_SIZE;
use ggez::mint;
use ggez::input::keyboard;


pub struct Master{
    window_mode: WindowMode,                            //holds data about window size
    
    mechanical_world: DefaultMechanicalWorld<f32>,    //N/M types are somehow not right??? Maybe give specific types???
    geometrical_world: DefaultGeometricalWorld<f32>,

    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,

    pub gameObjList: HashMap<i8, GameObject>,   //list of all objects in game
    pub player1: Player,
    pub player2: Player,
    over: bool,
    winnerId: i8,
    background_image: graphics::Image,

}

impl Master{

    pub fn new(ctx: &mut Context, window_mode: WindowMode) -> Self{
        
        let mut force_generators = DefaultForceGeneratorSet::new();

        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();


        let mut player1 = Player::new(true);
        //init player
        player1.createRigidBody( &mut bodies);
        player1.create_collider(&mut colliders);

        let mut player2 = Player::new(false);
        //init player
        player2.createRigidBody( &mut bodies);
        player2.create_collider(&mut colliders);

        let background_image = graphics::Image::new(ctx, "/bg.jpg").unwrap();

        let mut master = Master{
            window_mode,
             
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: bodies,
            colliders: colliders,
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: force_generators,

            gameObjList: HashMap::with_capacity(20),
            player1: player1,
            player2: player2,
            over: false,
            winnerId: 6,
            background_image: background_image,
        };
        return master;
    }

    pub(crate) fn spawn_game_objects(&mut self){
        self.gameObjList.clear();
        let mut id = 50i8;
        while self.gameObjList.len() < (GAME_SIZE / 20) as usize { //place GameObjects, until max number of allowed Objects is reached.
            self.gameObjList.insert(id, GameObject::new(&mut self.bodies, id, &mut self.colliders, self.window_mode.height, self.window_mode.width));
            id += 1;
        }
    }

    fn handle_proximity_events(&mut self){
        for proximity in self.geometrical_world.proximity_events(){
            //handle proximity events
            let data1 = self.colliders.get(proximity.collider1).unwrap().user_data().unwrap();
            let data2= self.colliders.get(proximity.collider2).unwrap().user_data().unwrap();
            let id1 = data1.downcast_ref::<i8>().unwrap();
            let id2 = data2.downcast_ref::<i8>().unwrap();

            //case player 1 is close to game object
            if *id1 == constants::PLAYER1_ID && *id2 >= 50 && *id2 <= 100 || *id1 >= 50 && *id1 <= 100 && *id2 == constants::PLAYER1_ID{
                //register player at game object
                let go_id = match *id1 {
                    constants::PLAYER1_ID => *id2,
                    _ => *id1,
                };
                if proximity.new_status == Proximity::Disjoint{
                    self.gameObjList.get_mut(&go_id).unwrap().deregisterPlayer(constants::PLAYER1_ID);
                }
                else {
                    self.gameObjList.get_mut(&go_id).unwrap().registerPlayer(constants::PLAYER1_ID);
                }

            }
            //case player 2 is close to game object
            if *id1 == constants::PLAYER2_ID && *id2 >= 50 && *id2 <= 100 || *id1 >= 50 && *id1 <= 100 && *id2 == constants::PLAYER2_ID{
                //register player at game object
                let go_id = match *id1 {
                    constants::PLAYER2_ID => *id2,
                    _ => *id1,
                };
                if proximity.new_status == Proximity::Disjoint{
                    self.gameObjList.get_mut(&go_id).unwrap().deregisterPlayer(constants::PLAYER2_ID);
                }
                else {
                    self.gameObjList.get_mut(&go_id).unwrap().registerPlayer(constants::PLAYER2_ID);
                }

            }
        }
    }

    fn handle_contact_events(&mut self){
        //check whether player has been shot
        for (_, go) in &mut self.gameObjList {
            if let Some(tuple) = self.geometrical_world.contact_pair(&self.colliders, self.player1.collider_handle.unwrap(), go.handleCollider.unwrap(), true) {
                if go.owned_by == constants::PLAYER2_ID {
                    self.player1.score -= 1;
                    println!("Hallo {}",self.player1.score);
                    go.owned_by = constants::PLAYER1_ID;
                    if self.player1.score == 0 {
                        println!("Player2 won");
                        self.over = true;
                        self.winnerId = constants::PLAYER2_ID;
                    }
                }
            }
            if let Some(tuple) = self.geometrical_world.contact_pair(&self.colliders, self.player2.collider_handle.unwrap(), go.handleCollider.unwrap(), true) {
                if go.owned_by == constants::PLAYER1_ID {
                    self.player2.score -= 1;
                    println!("Hallo {}",self.player2.score);
                    go.owned_by = constants::PLAYER2_ID;
                    if self.player2.score == 0 {
                        println!("Player1 won");
                        self.over = true;
                        self.winnerId = constants::PLAYER1_ID;
                    }
                }
            }
        }
        if let Some(tuple) = self.geometrical_world.contact_pair(&self.colliders, self.player1.collider_handle.unwrap(), self.player2.collider_handle.unwrap(), true) {
            println!("Hallo");
        }
        for contact in self.geometrical_world.contact_events(){
            
        }
        for contact in self.geometrical_world.contacts_with(&self.colliders, self.player1.collider_handle.unwrap(), true) {
            //let go = self.colliders.get(contact).unwrap().user_data().unwrap();
            //println!("Contact with Player one")
        }
    }

    fn update_game_objects(&mut self){
        //get player position
        let player1_pos: Isometry2<f32> = self.colliders.get(self.player1.collider_handle.unwrap()).unwrap().position().clone();
        let player2_pos: Isometry2<f32> = self.colliders.get(self.player2.collider_handle.unwrap()).unwrap().position().clone();

        for (_, go) in &mut self.gameObjList{
            go.update(&mut self.colliders, &mut self.bodies, &player1_pos, &player2_pos);
            go.timer();
        };
    }

    fn reset(&mut self, ctx: &mut Context) {
       // event::quit(ctx)

        //mach alles neu
        self.mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0));
        self.geometrical_world = DefaultGeometricalWorld::new();
        self.force_generators = DefaultForceGeneratorSet::new();
        self.bodies = DefaultBodySet::new();
        self.colliders = DefaultColliderSet::new();

        self.player1 = Player::new(true);
        //init player
        self.player1.createRigidBody( &mut self.bodies);
        self.player1.create_collider(&mut self.colliders);

        self.player2 = Player::new(false);
        //init player
        self.player2.createRigidBody( &mut self.bodies);
        self.player2.create_collider(&mut self.colliders);

        graphics::clear(ctx, graphics::WHITE);

        create_bound(self, Vector2::new(0.0, self.window_mode.height/2.0), Vector2::new(0.1, self.window_mode.height));
        create_bound(self, Vector2::new(self.window_mode.width, self.window_mode.height/2.0), Vector2::new(0.1, self.window_mode.height));
        create_bound(self, Vector2::new(self.window_mode.width/2.0, 0.0), Vector2::new(self.window_mode.width, 0.1));
        create_bound(self, Vector2::new(self.window_mode.width/2.0, self.window_mode.height), Vector2::new(self.window_mode.width, 0.1));

        self.spawn_game_objects();
    }

    pub fn get_over(self) -> (bool){  //return over
        self.over
    }
}

//EventHandler handling events...
impl EventHandler for Master {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        //handle end of game
        if self.over {
            if keyboard::is_key_pressed(_ctx, KeyCode::Space) {
                self.over = false;
                self.reset(_ctx);
            }
            return Ok(());
        }

        self.player1.update(_ctx, &mut self.bodies, &mut self.force_generators);
        self.player2.update(_ctx, &mut self.bodies, &mut self.force_generators);

        self.update_game_objects();

        self.mechanical_world.step(     //move the simulation further one step
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );

        self.handle_proximity_events();

        self.handle_contact_events();
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        //draw background image
        graphics::draw(ctx, &self.background_image, DrawParam::default())?;
        let mut text = graphics::Text::new(format!("Lives PLayer1 = {}",self.player1.score));
        text.set_bounds(mint::Point2 {x: 800., y: 50.}, graphics::Align::Left);
        graphics::draw(ctx, &text, DrawParam::default())?;
        text = graphics::Text::new(format!("Lives PLayer2 = {}",self.player2.score));
        text.set_bounds(mint::Point2 {x: 800., y: 50.}, graphics::Align::Right);
        graphics::draw(ctx, &text, DrawParam::default())?;
        self.player1.draw(ctx, &mut self.bodies)?;
        self.player2.draw(ctx, &mut self.bodies)?;

        if self.over {
            let winnerColor = match  self.winnerId {
                constants::PLAYER1_ID => constants::PLAYER1_COLOR,
                _ => constants::PLAYER2_COLOR,
            };
            let mut overText = graphics::Text::new("GAME OVER");
            overText.set_bounds(mint::Point2 {x: 800.,y: 600.}, graphics::Align::Center);
            overText.set_font(graphics::Font::default(), graphics::Scale { x: 100., y: 100. });
            graphics::draw(ctx, &overText, DrawParam::default().dest(mint::Point2 {x: 0., y: 200.}).color(winnerColor))?;
            let mut winnerText = match self.winnerId {
                constants::PLAYER1_ID => graphics::Text::new("Player 1 won this round"),
                _ => graphics::Text::new("Player 2 won this round"),
            };
            winnerText.set_bounds(mint::Point2 {x: 800.,y: 600.}, graphics::Align::Center);
            winnerText.set_font(graphics::Font::default(), graphics::Scale { x: 50., y: 50. });
            graphics::draw(ctx, &winnerText, DrawParam::default().dest(mint::Point2 {x: 0., y: 300.}).color(winnerColor))?;
        }

        for (_, go) in &self.gameObjList{
            go.draw(ctx, &mut self.bodies)?;
        }
        graphics::present(ctx)
    }
}