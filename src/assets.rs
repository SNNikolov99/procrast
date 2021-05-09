use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets {
    pub player:graphics::Image,
    pub chore_1:graphics::Image,
    pub chore_2:graphics::Image,
    pub chore_3:graphics::Image,
    pub chore_4:graphics::Image,
    pub chore_5:graphics::Image,
    pub chore_6:graphics::Image,
}

impl Assets{
    pub fn new(ctx:&mut Context)-> GameResult<Assets> {
       
            let player = graphics::Image::new(ctx,"/ferris.png")?;
            let chore_1 = graphics::Image::new(ctx,"/cleaning.png")?;
            let chore_2 = graphics::Image::new(ctx,"/doing the laundry.png")?;
            let chore_3 = graphics::Image::new(ctx,"/go to the market.png")?;
            let chore_4 = graphics::Image::new(ctx,"/stying.png")?;
            let chore_5 = graphics::Image::new(ctx,"/walking the dog.png")?;
            let chore_6 = graphics::Image::new(ctx,"/washing the dishes.png")?;
        
        Ok(Assets{player,chore_1,chore_2,chore_3,chore_4,chore_5,chore_6})
    }
}