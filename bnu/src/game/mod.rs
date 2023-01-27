use std::collections::HashMap;

use sdl2::render::Texture;

mod battle;

#[derive(Clone)]
pub enum GameState {
    Intro,
    Tile,    
    Battle(battle::Battle),
    Outtro{playerLost: bool},
}

pub enum MouseEvent
{
    Motion{x: u32, y: u32},
    Click{x: u32, y: u32}
}

pub struct Game {
    g: GameState,
}

impl Game {
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        Self {
            g: GameState::Battle(battle::Battle::new()),
        }
    }

    pub fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, resources: &HashMap<String, Texture>) {
        match self.g {
            GameState::Intro => todo!(),
            GameState::Tile => todo!(),
            GameState::Battle(ref b) => b.render(canvas, resources),
            GameState::Outtro{playerLost}  => todo!(),
        }
    }

    pub fn tick(&mut self) {
        let nextGameState;
        match self.g {
            GameState::Intro => nextGameState = GameState::Intro,
            GameState::Tile => nextGameState = GameState::Tile,
            GameState::Battle(ref mut b) => nextGameState = b.tick(),
            GameState::Outtro{playerLost} => {nextGameState = GameState::Outtro { playerLost }},
        }
        self.g = nextGameState;
    }

    pub fn mouseeveent(&mut self, event: MouseEvent)
    {
        match self.g {
            GameState::Intro => todo!(),
            GameState::Tile => todo!(),
            GameState::Battle(ref mut b) => b.mouseevent(event),
            GameState::Outtro{playerLost}  => {},
        }        
    }
}
