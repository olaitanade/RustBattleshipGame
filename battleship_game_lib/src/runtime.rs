use std::{collections::HashMap};

use crate::inventory::{ship::{ShipType, Ship, GridPoint, self}, grid::Grid};

#[derive(Debug,Clone,PartialEq, Eq, Hash)]
pub enum ShotStatus {
    Hit(Ship),
    Miss,
    Repeat
}

#[derive(Debug,Clone)]
pub struct Session<'a> {
    pub shot_history: Vec<GridPoint>,
    points: i32,
    remaining_shots: i32,
    player_name: String,
    grid: Grid<'a>,
    debug: bool
}

impl <'a> Session<'a> {

    pub fn build_from_allocation<'s>(player_name: String, grid: Grid<'s>) -> Session<'s> {
        Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name, grid, debug: false }
    }

    pub fn start<'s>(player_name: String, ships:&'s mut HashMap<ShipType, Ship>) -> Session<'s>{
        let mut grid = Grid::build();

        for (_key, ship) in ships.iter_mut() {
            grid.shuffle_ship_location(ship);
        }

        Self::build_from_allocation(player_name, grid)
    }

    pub fn get_player_name(&self) -> String {
        self.player_name.clone()
    }

    pub fn display_ships_location(&self) -> String {
        self.grid.display_ships_location()
    }

    pub fn is_any_ship_left(&self) -> bool{
        self.grid.is_any_ship_left()
    }

    pub fn is_shot_available(&self) -> bool{
        self.remaining_shots > 0
    }

    pub fn get_remaining_shots(&self) -> i32{
        self.remaining_shots
    }

    pub fn get_destroyed_ships(&self) -> Vec<Ship>{
        self.grid.get_destroyed_ships()
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
}

#[derive(Debug,Clone)]
pub struct Play <'a>{
    ships: HashMap<ShipType,Ship>,
    session: Session<'a>,
}

impl Play<'_> {
    pub fn build(player_name: String) -> Self {
        let mut ships = Ship::create_ships();
        let mut session: Session<'_> = Session::start(player_name, &mut ships);

        Play { ships, session }
    }

    pub fn get_session_as_mut(&mut self) -> &mut Session {
        &mut self.session
    }

    pub fn get_session_as_ref(&self) -> &Session {
        &self.session
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ship_allocation() {
        let mut ships = Ship::create_ships();
        let mut game_session: Session<'_> = Session::start(String::from("Adetayo"), &mut ships);
        
        
        game_session.shoot_ship(GridPoint { x: 7 , y:  7});
        
        println!("{:?}", game_session.get_destroyed_ships());
    }
}