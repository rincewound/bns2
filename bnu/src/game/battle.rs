use std::collections::{HashMap, VecDeque};

use rand::prelude::*;


use sdl2::rect::Rect;
use sdl2::render::{Texture, BlendMode};

use sdl2::{rect::Point, render::Canvas, video::Window};

use std::time::Instant;

use crate::draw;
use crate::vecmath::Vec2d;

use super::GameState;

const px:   i32 = 40;
const aix:  i32 = 420;
const offy: i32 = 60;

// if uni doesn't know anything else to say
const RANDOM_INSULTS: [&str; 5] = [
    "I will end you",
    "Your puny fleet is no match for me",
    "Mess with the best and die like the rest",
    "Don't bring a knife to a gunfight",
    "Stop playing hide and seek"
];

// After UNi hit a player ship
const INSULTS_AFTER_HIT: [&str; 4] = [
    "There's nothing like the smell of napalm in the morning",
    "You should've stayed at home",
    "Go home and be a family man!",
    "I salute my fallen enemy!"
];

// After UNi was hit
const INSULTS_AFTER_BEING_HIT: [&str; 4] = [
    "There's nothing like the smell of napalm in the morning",
    "You should've stayed at home",
    "Go home and be a family man!",
    "I salute my fallen enemy!"
];

#[derive(PartialEq, Clone, Copy)]
enum TurnEvents
{
    NoEvent,
    RandomTaunt,
    UniWasHit,
    PlayerWasHit,
    UniLostShip,
    PlayerLostShip
}

#[derive(Clone, PartialEq)]
enum State
{
    PlayerTurn,
    WaitingForPlayerProjectile,
    AITurn,
    WaitingForAiProjectile,
    UniLost,
    PlayerLost
}

#[derive(Copy, Clone, PartialEq)]
pub enum TileState
{
    Empty,
    ShotAt,
    ShotAndHit,
    HasShip
}

#[derive(Copy, Clone)]
pub struct playfield
{
    pub tiles: [TileState; 10 * 10]
}

#[derive(Clone)]
pub struct Battle
{
    turn_event: TurnEvents,
    time_state_entered : Instant,
    state: State,
    text: VecDeque<String>,
    has_click: bool,
    cursor_pos: Vec2d,
    // the field storing the player's ships, the unicorn shoots at this field
    player_field: playfield,
    // the field storing the unicorns's ships, the player shoots at this field
    unicorn_field: playfield,
    // The last place, where the ai hit a ship
    last_hit: Option<Vec2d>
}

impl Battle {
    pub fn new() -> Self {        
        let mut b = Self {   
            turn_event: TurnEvents::NoEvent,
            time_state_entered: Instant::now(),   
            state: State::PlayerTurn,      
            text: VecDeque::new(),
            has_click: false,
            cursor_pos: Vec2d { x: 0.0, y: 0.0 },
            player_field: playfield { tiles: [TileState::Empty; 10 * 10] },
            unicorn_field: playfield { tiles: [TileState::Empty; 10 * 10] },
            last_hit: None
        };

        Battle::distribute_ships(&mut b.player_field);
        Battle::distribute_ships(&mut b.unicorn_field);

        b
    }

    fn vec2index(v: &Vec2d) -> usize
    {
        (v.x as i32 + v.y as i32 * 10) as usize
    }

    fn index2vec(i: usize) -> Vec2d
    {
        Vec2d::from_ints((i % 10) as i32, (i / 10) as i32)
    }

    fn distribute_ships(field: &mut playfield)
    {
        let ship_sizes = vec![2, 3, 3, 4, 5];
        for s in ship_sizes
        {
            loop 
            {
                let x = rand::thread_rng().gen_range(0..10) as usize;
                let y = rand::thread_rng().gen_range(0..10) as usize;
                let dir = rand::thread_rng().gen_bool(0.5);
                let step;
                if dir 
                {
                    step = Vec2d::from_ints(1, 0);
                }
                else {
                    step = Vec2d::from_ints(0, 1);
                }

                let len = step * s as f32;
                let start = Vec2d::new(x as f32, y as f32);
                let end = start + len;
                if end.x > 9.0 || end.y > 9.0
                {
                    continue;
                }

                for i in 0..s
                {
                    let cur_pos = start + (step * i as f32);
                    let index = Battle::vec2index(&cur_pos);
                    field.tiles[index] = TileState::HasShip;
                }
                break;
            }

        }

    }

    fn draw_overlay(sdl_context: &mut Canvas<Window>, overlay_raster_pos: Vec2d, source_tiles: &playfield, overlay_pixel_pos: Vec2d)
    {
        let green= sdl2::pixels::Color {
            r: 0,
            g: 128,
            b: 0,
            a: 128,
        };

        let red= sdl2::pixels::Color {
            r: 128,
            g: 0,
            b: 0,
            a: 128,
        };

        let grey = sdl2::pixels::Color {
            r: 128,
            g: 128,
            b: 128,
            a: 128,
        };

        let field_index = (overlay_raster_pos.x as i32 + overlay_raster_pos.y as i32 * 10) as usize;
        let used_color;
        match source_tiles.tiles[field_index]
        {
            TileState::Empty => return,
            TileState::ShotAt => used_color = green,
            TileState::ShotAndHit => used_color = red,
            TileState::HasShip => used_color = grey,
        }

        sdl_context.set_blend_mode(BlendMode::Add);
        sdl_context.set_draw_color(used_color);
        let _ = sdl_context.fill_rect(Rect::new(overlay_pixel_pos.x as i32, overlay_pixel_pos.y as i32, 32, 32));

    }

    pub fn render(&self, sdl_context: &mut Canvas<Window>, resources: &HashMap<String, Texture>) {
        let _ = draw::draw_text(
            sdl_context,
            "A raging battle against a unicorn",
            16,
            Point::new(10, 10),
            sdl2::pixels::Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        );

        let uni = resources.get("unicorn").unwrap();
        let water = resources.get("water").unwrap();

        let _ = sdl_context.copy(uni, None, Rect::new(40, 400, 150, 150));
        let white = sdl2::pixels::Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };

        let dialogbox_corner = Vec2d::new(200.0, 400.0);
        let cursor_raster_pos = ((self.cursor_pos.clone() - Vec2d::new(aix as f32, offy as f32)) / 32.0).truncate();
        // Draw 2 playfields, one for the player's ships and one for the targets:
        self.draw_playfields(sdl_context, water);
        self.draw_cursor(sdl_context, cursor_raster_pos);        
        self.draw_dialog_box(dialogbox_corner, sdl_context, white);

    }

    fn draw_playfields(&self, sdl_context: &mut Canvas<Window>, water: &Texture) {        
        sdl_context.set_blend_mode(BlendMode::None);
        for y in 0..=9
        {
            for x in 0..=9
            {                
                let _ = sdl_context.copy(water, None, Rect::new(px + x * 32, offy + y * 32, 32, 32));                
                let _ = sdl_context.copy(water, None, Rect::new(aix + x * 32, offy + y * 32, 32, 32));

                // Draw overlay for player and unicorn:
                let raster_pos = Vec2d::from_ints(x,y);
                let pixel_pos_player = Vec2d::from_ints(px + x * 32, offy + y * 32);
                let pixel_pos_uni = Vec2d::from_ints(aix + x * 32, offy + y * 32);

                Battle::draw_overlay(sdl_context, raster_pos, &self.player_field, pixel_pos_player);
                Battle::draw_overlay(sdl_context, raster_pos, &self.unicorn_field, pixel_pos_uni);                            
            }            
        }
    }

    fn draw_dialog_box(&self, dialogbox_corner: Vec2d, sdl_context: &mut Canvas<Window>, white: sdl2::pixels::Color) {
        let _ = draw::draw_rect(sdl_context, &dialogbox_corner, 580, 150, white, false);
        let mut org = dialogbox_corner.clone();
        for t in self.text.iter()
        {
            let _= draw::draw_text(sdl_context, t, 16, org.to_point(), white);
            org = org + Vec2d::new(0.0, 24.0);
        }
    }

    fn draw_cursor(&self, sdl_context: &mut Canvas<Window>, cursor_raster_pos: Vec2d) {
        if self.state == State::PlayerTurn
        {
            sdl_context.set_blend_mode(BlendMode::Add);
            sdl_context.set_draw_color(sdl2::pixels::Color::RGBA(128, 128,128,128));
            let _ = sdl_context.fill_rect(Rect::new(aix + cursor_raster_pos.x as i32 * 32, offy + cursor_raster_pos.y as i32 * 32, 32, 32));
        }
    }

    fn count_alive_fields(&self, field: &playfield) -> usize
    {
        field.tiles.iter().fold(0, |acc, x| {
            if *x == TileState::HasShip
            {
                return acc + 1
            }
            acc
        })
    }

    pub fn tick(&mut self) -> GameState
    {
        self.cull_texts();
        let s = self.state.clone();
        match s
        {
            State::PlayerTurn => {
                if self.player_turn()
                {
                    self.state = State::WaitingForPlayerProjectile;
                    self.time_state_entered = Instant::now();
                }
            },
            State::WaitingForPlayerProjectile => 
            {
                if self.waiting_for_player_projectile()
                {
                    // check if player won:
                    if self.count_alive_fields(&self.unicorn_field) <= 0
                    {
                        self.state = State::UniLost;
                        self.text.push_back("You definitely cheated.".to_string());
                    }
                    else
                    {
                        self.state = State::AITurn;
                        if self.turn_event == TurnEvents::NoEvent
                        {
                            self.turn_event = TurnEvents::RandomTaunt;
                        }
                    }
                    self.time_state_entered = Instant::now();
                }
            },
            State::AITurn =>             {
                if self.aiturn()
                {
                    self.state = State::WaitingForAiProjectile;
                    self.time_state_entered = Instant::now();
                }
            },
            State::WaitingForAiProjectile =>             {
                if self.waiting_for_ai_projectile()
                {
                    if self.count_alive_fields(&self.unicorn_field) <= 0
                    {
                        self.state = State::PlayerLost;
                        self.text.push_back("I broke all of your toys. You cryin' now?.".to_string());
                    }
                    else
                    {
                        self.state = State::PlayerTurn;                     
                        self.has_click = false;
                    }
                    self.time_state_entered = Instant::now();
                }
            },
            State::UniLost => {
                if self.time_state_entered.elapsed().as_secs() >= 1
                {
                    return GameState::Outtro{playerLost: false};
                }
            },
            State::PlayerLost => 
            {
                if self.time_state_entered.elapsed().as_secs() >= 1
                {
                    return GameState::Outtro{playerLost: true};
                }
            },
        }
        GameState::Battle(self.clone())
    }

    fn player_turn(&mut self) -> bool
    {
        if self.has_click
        {
            let cursor_raster_pos = ((self.cursor_pos.clone() - Vec2d::new(aix as f32, offy as f32)) / 32.0).truncate();    
            let field_index = (cursor_raster_pos.x as i32 + cursor_raster_pos.y as i32 * 10) as usize;

            let next_state = match self.unicorn_field.tiles[field_index]
            {
                TileState::Empty => TileState::ShotAt,
                TileState::ShotAt => TileState::ShotAt,
                TileState::ShotAndHit => TileState::ShotAndHit,
                TileState::HasShip => {
                    self.turn_event = TurnEvents::UniWasHit;
                    TileState::ShotAndHit
                }
            };
            
            self.unicorn_field.tiles[field_index] = next_state;
        }
        self.has_click
    }

    fn waiting_for_player_projectile(&mut self) -> bool {
        true
    }

    fn select_taunt(&mut self, list: &[&str])
    {
        let index = rand::thread_rng().gen_range(0..list.len()) as usize;
        self.text.push_back(list[index].to_string());
        self.turn_event = TurnEvents::NoEvent;
    }

    fn aiturn(&mut self) -> bool {

        // Select insult, based on gamestate
        if self.turn_event != TurnEvents::NoEvent
        {
            self.cull_texts();
        }

        match self.turn_event{
            TurnEvents::NoEvent => {},
            TurnEvents::UniWasHit => self.select_taunt(&INSULTS_AFTER_BEING_HIT),
            TurnEvents::PlayerWasHit => self.select_taunt(&INSULTS_AFTER_HIT),
            TurnEvents::UniLostShip => todo!(),
            TurnEvents::PlayerLostShip => todo!(),
            TurnEvents::RandomTaunt => self.select_taunt(&RANDOM_INSULTS),
        }
        

        // Fire shot
            // Select firing position:
            // if no recent shot hit anything, select random location
            // otherwise, try to hit close to the last shot

        true
    }
    
    fn can_shoot_at(&self, v: &Vec2d) -> bool
    {
        let idx = Battle::vec2index(v);
        if self.player_field.tiles[idx] != TileState::ShotAndHit &&
        self.player_field.tiles[idx] != TileState::ShotAt
        {
            return true
        }
        false
    }

    fn has_ship(&self, v: &Vec2d) -> bool
    {
        let idx = Battle::vec2index(v);
        self.player_field.tiles[idx] == TileState::HasShip
    }

    fn select_shooting_pos(&self) -> Option<Vec2d>
    {
        let hit_pos = self.last_hit.clone().unwrap();

        // check surroundings of hitpos for empty space and return
        // that as next pos:

        // check row:
        let check_start_row = hit_pos - Vec2d::new(1.0, 0.0);
        for x in check_start_row.x as i32..=(check_start_row.x as i32 + 2)
        {
            if x < 0 || x > 9
            {
                continue;
            }
            let possible_target =Vec2d::from_ints(x, hit_pos.y as i32);
            if self.can_shoot_at(&possible_target)
            {
                return Some(possible_target)
            }
        }

        let check_start_col = hit_pos - Vec2d::new(0.0, 1.0);
        for y in check_start_col.y as i32..=(check_start_col.y as i32 + 2)
        {
            if y < 0 || y > 9
            {
                continue;
            }
            let possible_target =Vec2d::from_ints(hit_pos.x as i32, y  );
            if self.can_shoot_at(&possible_target)
            {
                return Some(possible_target)
            }
        }
        None
    }

    fn select_random_shooting_pos(&mut self) -> usize
    {
        // with a probability of 10% we'll allow the unicorn to
        // cheat and just find a ship, if it has no clue where one
        // cuold be.
        let let_it_cheat  =rand::thread_rng().gen_bool(0.5);
        if let_it_cheat
        {
            for i in 0..100
            {
                if self.has_ship(&Battle::index2vec(i))
                {
                    return i;
                }
            }
        }

        loop {
            let index = rand::thread_rng().gen_range(0..100) as usize;
            if self.player_field.tiles[index] == TileState::Empty ||
               self.player_field.tiles[index] == TileState::HasShip
            {
                return index;
            } 
        } 
    }

    fn waiting_for_ai_projectile(&mut self) -> bool
    {
        // We wait for 2 seconds before the projectile "hits"
        // if a player ship was hit and/or sunk, we issue another
        // insult, before exiting this state.
        if self.time_state_entered.elapsed().as_secs() >= 1
        {
            let tile_to_target;
            if self.last_hit.is_some()
            {
                let sp = self.select_shooting_pos();                
                if let Some(pos) = sp
                {
                    tile_to_target = Battle::vec2index(&pos);
                }
                else 
                {
                    tile_to_target = self.select_random_shooting_pos();
                }
            }
            else
            {
                tile_to_target = self.select_random_shooting_pos();
            }


            if self.player_field.tiles[tile_to_target] == TileState::Empty
            {
                self.player_field.tiles[tile_to_target] = TileState::ShotAt;
                self.turn_event = TurnEvents::RandomTaunt;
            }
            else 
            {
                self.player_field.tiles[tile_to_target] = TileState::ShotAndHit;
                self.turn_event = TurnEvents::PlayerWasHit;
                self.last_hit = Some(Battle::index2vec(tile_to_target));

                // ToDo: Check if the ship was actually sunk and select
                // a different taunt.
            }                                    
            return true              
        }
        false
    }

    fn cull_texts(&mut self)
    {
        while self.text.len() > 4
        {
            self.text.pop_front();
        }
    }

    pub fn mouseevent(&mut self, event: super::MouseEvent) {
        if self.state == State::PlayerTurn
        {
            match event
            {
                super::MouseEvent::Motion { x, y } => 
                {
                    if x >= 420 && x < (420 + 320)
                    {
                        if y >= 60 && y <= 380
                        {
                            self.cursor_pos = Vec2d::new(x as f32, y as f32);
                        }
                    }
                },
                super::MouseEvent::Click { x, y } => 
                {
                    if x >= 420 && x < (420 + 320)
                    {
                        if y >= 60 && y <= 380
                        {
                            self.cursor_pos = Vec2d::new(x as f32, y as f32);
                            self.has_click = true;
                        }
                    }                    
                },
            }
        }
    }

}
