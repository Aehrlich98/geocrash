use ggez::Context;

type Point = (i32, i32);

pub struct GameObject {
    position: Point,
    speed: u32,
    direction: u32,
    mass: u32,
}

impl GameObject {

    pub fn new() -> Self{
        GameObject {
            position: (10,10),
            speed: 0,
            direction: 0,
            mass: 0
        }
    }

    pub fn update(){

    }

    pub fn draw(&mut self, context: Context){
        //TODO: render Object at current position
    }
}