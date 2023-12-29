use std::{collections::HashMap, borrow::BorrowMut, cell::RefCell};
use rand::prelude::*;
use crate::inventory::{self, Grid, Ship, ShipType, GridPoint, Orientation};

pub struct Session<'a> {
    pub ships: HashMap<ShipType, Ship<'a>>,
    pub shot_history: Vec<GridPoint>,
    points: i32,
    remaining_shots: i32,
    player_name: String,
    grid: Grid<'a>,
    debug: bool
}

impl <'a> Session<'a> {
    // pub fn build(player_name: String) -> Session<'a>{
    //     Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name, grid: RefCell::new(Grid::build()), debug: false, ships: Self::create_ships() }
    // }

    // pub fn build_with_grid(player_name: String, grid: Grid<'static>) -> Session<'a>{
    //     Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name, grid: RefCell::new(grid), debug: false, ships: Self::create_ships() }
    // }

    // pub fn build_with_debug(player_name: String, debug: bool) -> Session<'a>{
    //     Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name: player_name, grid: RefCell::new(Grid::build()), debug, ships: Self::create_ships() }
    // }

    pub fn create_ships() -> HashMap<ShipType, Ship<'a>> {
        let mut ship_yard = HashMap::new();
        
        ship_yard.insert(ShipType::AircraftCarrier, Ship::build(ShipType::AircraftCarrier));
        ship_yard.insert(ShipType::Battleship, Ship::build(ShipType::Battleship));
        ship_yard.insert(ShipType::Submarine, Ship::build(ShipType::Submarine));
        ship_yard.insert(ShipType::Destroyer, Ship::build(ShipType::Destroyer));
        ship_yard.insert(ShipType::PatrolBoat, Ship::build(ShipType::PatrolBoat));

        ship_yard
    }

    pub fn start(){

    }

    pub fn display_ships_location(){

    }

    pub fn is_any_ship_left(){

    }

    pub fn is_shot_available(){

    }

    pub fn get_remaining_shots(){

    }

    pub fn get_destroyed_ships(){

    }

    pub fn shoot_ship(){

    }

    fn destroy_ship(){

    }

    fn allocate_ships_on_grid(&'a mut self){
        self.grid.allocate_ships(&mut self.ships);
    }
}