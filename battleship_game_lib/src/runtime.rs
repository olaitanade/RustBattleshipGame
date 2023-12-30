use std::{collections::HashMap};

use crate::inventory::{ship::{ShipType, Ship, GridPoint, self}, grid::Grid};

#[derive(Debug,Clone,Copy,PartialEq, Eq, Hash)]
pub enum ShotStatus<'a> {
    Hit(Ship<'a>),
    Miss,
    Repeat
}

#[derive(Debug,Clone)]
pub struct Session<'a> {
    pub ships: HashMap<ShipType, Ship<'static>>,
    pub shot_history: Vec<GridPoint>,
    points: i32,
    remaining_shots: i32,
    player_name: String,
    grid: Grid<'a>,
    debug: bool
}

impl Session<'_> {
    pub fn build(player_name: String) -> Self{
        Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name, grid: Grid::build(), debug: false, ships: Self::create_ships() }
    }

    pub fn build_from_store(player_name: String, grid: Grid<'static>, ships: HashMap<ShipType, Ship<'static>>) -> Self {
        Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name, grid, debug: false, ships }
    }

    pub fn build_with_debug(player_name: String, debug: bool) -> Self{
        Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name: player_name, grid: Grid::build(), debug, ships: Self::create_ships() }
    }

    pub fn get_player_name(&self) -> String {
        self.player_name.clone()
    }

    pub fn display_ships_location(&self) -> String {
        let mut display = String::new();

        for (_key, ship) in self.ships.iter() {
            display.push_str(&format!("{} \n", ship.get_debug_mode_string()))
        }

        display
    }

    pub fn is_any_ship_left(&self) -> bool{
        for (_key, ship) in self.ships.iter() {
            if !ship.is_destroyed() {
                return true;
            }
        }
        false
    }

    pub fn is_shot_available(&self) -> bool{
        self.remaining_shots > 0
    }

    pub fn get_remaining_shots(&self) -> i32{
        self.remaining_shots
    }

    pub fn get_destroyed_ships(&self) -> Vec<Ship>{
        let ships: Vec<Ship> = self.ships.values().cloned().filter(|&ship| ship.is_destroyed()).collect();
        ships
    }

    pub fn shoot_ship(&mut self, proj_loc: GridPoint) -> ShotStatus {
        for grid in self.shot_history.iter() {
            if grid.x == proj_loc.x && grid.y == proj_loc.y {
                return ShotStatus::Repeat;
            }
        }
        self.remaining_shots -= 1;
        self.shot_history.push(proj_loc.clone());

        match self.grid.hit_ship(proj_loc.clone()) {
            ShotStatus::Hit(ship) => {
                self.points += ship.get_point();
                return ShotStatus::Hit(ship);
            },
            _ => return ShotStatus::Miss
        }
    }

    pub fn set_debug(&mut self, debug: bool){
        self.debug = debug;
    }

    fn create_ships() -> HashMap<ShipType, Ship<'static>> {
        let mut ship_yard = HashMap::new();
        
        ship_yard.insert(ShipType::AircraftCarrier, Ship::build(ShipType::AircraftCarrier));
        ship_yard.insert(ShipType::Battleship, Ship::build(ShipType::Battleship));
        ship_yard.insert(ShipType::Submarine, Ship::build(ShipType::Submarine));
        ship_yard.insert(ShipType::Destroyer, Ship::build(ShipType::Destroyer));
        ship_yard.insert(ShipType::PatrolBoat, Ship::build(ShipType::PatrolBoat));

        ship_yard
    }

    
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    #[test]
    fn test_ship_allocation() {
        let mut game_session = Session::build_with_debug(String::from("Adetayo"), true);
        
        println!("{:?}", game_session.display_ships_location());
    }
}