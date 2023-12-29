use std::collections::HashMap;

use crate::inventory::{self, Grid, Ship, ShipType, GridPoint};

pub struct Session<'a> {
    pub ships: HashMap<ShipType, Ship<'a>>,
    pub shot_history: Vec<GridPoint>,
    points: i32,
    shots: i32,
    player_name: String,
    grid: Option<Grid<'a>>,
    debug: bool
}

impl <'a> Session<'a> {
    pub fn build(player_name: String) -> Session<'a>{
        Session { points: 0, shot_history: Vec::new(), shots: 0, player_name, grid: None, debug: false, ships: Self::create_ships() }
    }

    pub fn build_with_grid(player_name: String, grid: Grid<'static>) -> Session<'a>{
        Session { points: 0, shot_history: Vec::new(), shots: 0, player_name, grid: Some(grid), debug: false, ships: Self::create_ships() }
    }

    pub fn build_with_debug(player_name: String, debug: bool) -> Session<'a>{
        Session { points: 0, shot_history: Vec::new(), shots: 0, player_name: player_name, grid: None, debug, ships: Self::create_ships() }
    }

    fn create_ships() -> HashMap<ShipType, Ship<'a>> {
        let mut ship_yard = HashMap::new();
        
        ship_yard.insert(ShipType::AircraftCarrier, Ship::build(ShipType::AircraftCarrier));
        ship_yard.insert(ShipType::Battleship, Ship::build(ShipType::Battleship));
        ship_yard.insert(ShipType::Submarine, Ship::build(ShipType::Submarine));
        ship_yard.insert(ShipType::Destroyer, Ship::build(ShipType::Destroyer));
        ship_yard.insert(ShipType::PatrolBoat, Ship::build(ShipType::PatrolBoat));

        ship_yard
    }

    fn allocate_ships_on_grid(){

    }
}