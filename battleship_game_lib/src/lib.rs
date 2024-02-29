#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use runtime::Play;
use storage::Store;

pub mod runtime;
mod inventory;
mod storage;


pub trait App {
    fn exit(&self);
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct GamePlay{
    play: Option<Play>,
    store: Store
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl GamePlay {
    
    pub fn initialize()-> Self {
        GamePlay{ play: None, store: Store::build() }
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    pub fn start_new(&mut self, player_name: String) -> &mut Play {
        self.play = Some(Play::init(player_name));

        self.play.as_mut().unwrap()
    }

    #[cfg(feature = "wasm-bindgen")]
    pub fn start_new(&mut self, player_name: String) -> Play {
        self.play = Some(Play::init(player_name));

        self.play.clone().unwrap()
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    pub fn load(&mut self, player_name: String) -> Option<&mut Play> {
        self.play = self.store.get_play(&player_name);

        self.play.as_mut()
    }

    #[cfg(feature = "wasm-bindgen")]
    pub fn load(&mut self, player_name: String) -> Play {
        self.play = self.store.get_play(&player_name);

        self.play.clone().unwrap()
    }

    pub fn list_saved(&self) -> Vec<String> {
        self.store.get_player_names()
    }

    pub fn delete(&mut self,player_name: String) -> Option<Play> {
        self.store.pop_play(&player_name)
    }

    pub fn save(&mut self) {
        match self.play.clone() {
            Some(play) => self.store.save_play(play),
            None => (),
        }
        
    }

    pub fn save_and_exit(&mut self) {
        self.save();
        self.play = None;
    }
}


#[cfg(test)]
mod tests {
    use crate::runtime::GridPoint;

    use super::*;

    #[test]
    fn test_play() {
        let mut game = GamePlay::initialize();
        let play = game.start_new(String::from("Adetayo"));

        play.get_session_as_mut().shoot_ship(GridPoint { x: 1 , y:  1});
        play.get_session_as_mut().shoot_ship(GridPoint { x: 2 , y:  2});
        play.get_session_as_mut().shoot_ship(GridPoint { x: 3 , y:  3});
        play.get_session_as_mut().shoot_ship(GridPoint { x: 4 , y:  4});
        play.get_session_as_mut().shoot_ship(GridPoint { x: 5 , y:  5});

        println!("{:?}", play.get_session_as_ref().get_destroyed_ships());
        
        play.get_session_as_mut().shoot_ship(GridPoint { x: 6 , y:  6});
        play.get_session_as_mut().shoot_ship(GridPoint { x: 7 , y:  7});
        play.get_session_as_mut().shoot_ship(GridPoint { x: 8 , y:  8});
        play.get_session_as_mut().shoot_ship(GridPoint { x: 9 , y:  9});
        
        println!("{:?}", play.get_session_as_ref().get_destroyed_ships());
    }
}