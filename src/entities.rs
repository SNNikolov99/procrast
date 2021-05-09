use ggez::{Context, GameResult};
use ggez::mint::{Point2,Vector2};
use ggez::graphics;
use crate::assets::Assets;

pub struct Player {
   pub pos:Point2<f32>,
   pub velocity: Vector2<f32>,
    //picture of a character

}


impl Player {
    pub const SPEED: f32 = 500.0;

    pub fn new() ->Self {
        Self {
            pos:Point2{x:0.0,y:0.0},
            velocity: Vector2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn update(&mut self,amount:f32,seconds:f32,max_right:f32) {
        if self.pos.y <= 0.0 && self.pos.y > max_right {
            let new_pos_y = self.pos.y+ Self::SPEED*amount*seconds;
            self.pos.y = new_pos_y;
        }
    }

    pub fn draw(&mut self,ctx:&mut Context,assets:Assets) -> GameResult {
        graphics::draw(ctx,&assets.player,graphics::DrawParam{
            dest:self.pos,
            scale:Vector2{x:0.2, y:0.2},
            offset:Point2{x :1.0, y:0.5},
            ..Default::default()
        })?;
        //graphics::present(ctx)?;
        Ok(())
    }


}

pub struct Chore {
    pub pos:Point2<f32>,
    pub is_alive:bool, //if the "enemy" reaches the left of the screen it dissapears
    pub velocity: Vector2<f32>,
    label:u32, // will be used to represent chores
    //picture of a chore (ex: cleaning - vaccum cleaner,washing the dishes - a dish,etc)
}

impl Chore {
    pub fn new(given_pos:Point2<f32>,given_velocity:Vector2<f32>,given_label:u32) -> Self {
        Self {
            pos:given_pos,
            velocity: given_velocity,
            label:given_label,
            is_alive:true,
        }
    }

    pub fn update(&mut self,ctx:&mut Context) -> GameResult {
        Ok(())
    }

    pub fn draw(&mut self,ctx:&mut Context,assets:Assets) -> GameResult {
        if self.is_alive == true{
            match self.label {
                    1 =>{
                        graphics::draw(ctx,&assets.chore_1,graphics::DrawParam{
                            dest:self.pos,
                            scale:Vector2{x:0.2, y:0.2},
                            offset:Point2{x :1.0, y:0.5},
                            ..Default::default()
                        })?
                    },
                    2 =>{
                        graphics::draw(ctx,&assets.chore_2,graphics::DrawParam{
                            dest:self.pos,
                            scale:Vector2{x:0.2, y:0.2},
                            offset:Point2{x :1.0, y:0.5},
                            ..Default::default()
                        })?
                    },
                    3 =>{
                        graphics::draw(ctx,&assets.chore_3,graphics::DrawParam{
                            dest:self.pos,
                            scale:Vector2{x:0.2, y:0.2},
                            offset:Point2{x :1.0, y:0.5},
                            ..Default::default()
                        })?
                    },
                    4 =>{
                        graphics::draw(ctx,&assets.chore_4,graphics::DrawParam{
                            dest:self.pos,
                            scale:Vector2{x:0.2, y:0.2},
                            offset:Point2{x :1.0, y:0.5},
                            ..Default::default()
                        })?
                    },
                    5 =>{
                        graphics::draw(ctx,&assets.chore_5,graphics::DrawParam{
                            dest:self.pos,
                            scale:Vector2{x:0.2, y:0.2},
                            offset:Point2{x :1.0, y:0.5},
                            ..Default::default()
                        })?
                    },
                    6 =>{
                        graphics::draw(ctx,&assets.chore_6,graphics::DrawParam{
                            dest:self.pos,
                            scale:Vector2{x:0.2, y:0.2},
                            offset:Point2{x :1.0, y:0.5},
                            ..Default::default()
                        })?
                    }
                    _=> ()
            }
            Ok(())
        }
        else {
            return Ok(())
        }
    }
}