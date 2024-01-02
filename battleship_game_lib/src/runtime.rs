use std::{collections::HashMap};

use crate::inventory::{ship::{ShipType, Ship, GridPoint, self}, grid::Grid};

#[derive(Debug,Clone,PartialEq, Eq, Hash)]
pub enum ShotStatus {
    Hit(ShipType, i32),
    Miss,
    Repeat
}

#[derive(Debug,Clone)]
pub struct Session {
    pub shot_history: Vec<GridPoint>,
    pub debug: bool,
    points: i32,
    remaining_shots: i32,
    player_name: String,
    grid: Grid
}

impl Session {

    pub fn build() -> Session {
        Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name: String::new(), grid: Grid::build(), debug: false }
    }
    pub fn build_from_allocation<'s>(player_name: String, grid: Grid) -> Session {
        Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name, grid, debug: false }
    }

    pub fn start<'s>(player_name: String) -> Session{
        let mut grid = Grid::build();
        grid.shuffle_ship_location();

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
            ShotStatus::Hit(ship, point) => {
                self.points += point;
                return ShotStatus::Hit(ship, point);
            },
            _ => return ShotStatus::Miss
        }
    }

    pub fn set_debug(&mut self, debug: bool){
        self.debug = debug;
    }
}

#[derive(Debug,Clone)]
pub struct Play {
    session: Session,
}

impl Play {
    pub fn init(player_name: String) -> Play {
       Play { session: Session::start(player_name)}
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

    use crate::GamePlay;

    use super::*;

    #[test]
    fn test_ship_allocation() {
        let mut ships = Ship::create_ships();
        let mut game_session = Session::start(String::from("Adetayo"));
        
        
        game_session.shoot_ship(GridPoint { x: 7 , y:  7});
        
        println!("{:?}", game_session.get_destroyed_ships());
    }

    #[test]
    fn test_play() {
        let mut game = GamePlay::initialize();
        let play = game.start_new(String::from("Adetayo"));
        
        play.get_session_as_mut().shoot_ship(GridPoint { x: 7 , y:  7});
        
        println!("{:?}", play.get_session_as_ref().get_destroyed_ships());
    }
}