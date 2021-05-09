use ggez::*;
use ggez::conf::{WindowMode,Conf};
use ggez::event::{EventHandler,KeyCode,KeyMods};
use ggez::graphics::{DrawParam,DrawMode,Rect,Color};
use ggez::mint::{Point2,Vector2};
use ggez::audio::SoundSource;

use rand::Rng;
use rand::rngs::ThreadRng;

use std::env;
use std::path;
use procrast::entities::{Chore,Player};
use procrast::assets::Assets;

struct GameState {
    rng:ThreadRng,
    score:u32,
    chores:Vec<Chore>,
    player:Player,
    assets:Assets,
    game_over:bool,
    window_width:f32,
    window_heigth:f32,
    time_till_next_chore:f32,

}

impl GameState {
    fn new(ctx:&mut Context,c:&Conf) -> GameResult<GameState> {
        let gs = Self{
            rng:rand::thread_rng(),
            score:0,
            chores:Vec::new(),
            player:Player::new(),
            assets:Assets::new(ctx)?,
            game_over:false,
            window_width:c.window_mode.width,
            window_heigth:c.window_mode.height,
            time_till_next_chore:0.2,

        };
        Ok(gs)
    }
}

impl EventHandler for GameState {
    fn update(&mut self,_ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self,_ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new().window_mode(WindowMode {
        width: 800.0,
        height: 600.0,
        ..Default::default()
      });
    
      
      let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("project", "simeon")
        .conf(c.clone())
        .build()
        .unwrap();
    
      //  load the assets from resources
      if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
          let mut path = path::PathBuf::from(manifest_dir);
          path.push("resources");
          filesystem::mount( ctx, &path, true);
      }
      
      //load the state
      let state = &mut GameState::new(ctx, &c).unwrap();
     
      //run!
      event::run(ctx, event_loop, state).unwrap();
}
