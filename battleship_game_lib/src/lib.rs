use std::collections::HashMap;

use inventory::ship::{Ship, ShipType};
use runtime::{Session, Play};
use storage::Store;

pub mod runtime;
mod inventory;
mod storage;

pub trait App {
    fn exit(&self);
}

pub struct GamePlay{
    play: Option<Play>,
    store: Store
}

impl GamePlay {
    
    pub fn initialize()-> Self {
        GamePlay{ play: None, store: Store::build() }
    }

    pub fn start_new(&mut self, player_name: String) -> &mut Play {
        self.play = Some(Play::init(player_name));

        self.play.as_mut().unwrap()
    }

    pub fn load(&mut self, player_name: String) -> Option<&mut Play> {
        self.play = self.store.get_play(&player_name);

        self.play.as_mut()
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