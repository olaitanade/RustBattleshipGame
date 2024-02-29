#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use std::fmt;
use crate::inventory::{ship::{ShipType, Ship}, grid::Grid};


///Gridpoint representation of the 2 dimensional array
/// x > 0, x < 11 ,1-10 inclusive
/// y > 0, y < 11 ,1-10 inclusive
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug,Clone,Copy, PartialEq, Eq, Hash)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

///Display GridPoint
impl fmt::Display for GridPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GridPoint(x = {}, y = {})", &self.x, &self.y)
    }
}


///Default struct value of GridPoint
impl Default for GridPoint {
    ///GridPoint default is all zeros
    fn default() -> GridPoint {
        GridPoint {
            x: 1,
            y: 1
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl GridPoint {
    pub fn new(x: i32, y: i32) -> GridPoint {
        GridPoint{x,y}
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug,Clone, Copy,PartialEq, Eq, Hash)]
pub enum ShotStatus {
    Hit,
    Miss,
    Repeat
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Shot {
    pub status: ShotStatus,
    pub ship_type: Option<ShipType>,
    pub point: Option<i32>
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug,Clone)]
pub struct Session {
    pub debug: bool,
    shot_history: Vec<GridPoint>,
    points: i32,
    remaining_shots: i32,
    player_name: String,
    grid: Grid
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Session {

    pub fn build() -> Session {
        Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name: String::new(), grid: Grid::build(), debug: false }
    }
    pub fn build_from_allocation(player_name: String, grid: Grid) -> Session {
        Session { points: 0, shot_history: Vec::new(), remaining_shots: 10, player_name, grid, debug: false }
    }

    pub fn start(player_name: String) -> Session{
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

    pub fn shoot_ship(&mut self, proj_loc: GridPoint) -> Shot {
        for grid in self.shot_history.iter() {
            if grid.x == proj_loc.x && grid.y == proj_loc.y {
                return Shot{ status: ShotStatus::Repeat, ship_type: None, point: None };
            }
        }
        self.remaining_shots -= 1;
        self.shot_history.push(proj_loc.clone());

        match self.grid.hit_ship(proj_loc.clone()) {
            Shot{ status: ShotStatus::Hit, ship_type, point} => {
                self.points += point.unwrap();
                return Shot{ status: ShotStatus::Hit, ship_type, point};
            },
            _ => Shot{ status: ShotStatus::Miss, ship_type: None, point: None }
        }
    }

    pub fn get_points (&self) -> i32 {
        self.points
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Debug,Clone)]
pub struct Play {
    session: Session,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Play {
    pub fn init(player_name: String) -> Play {
       Play { session: Session::start(player_name)}
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    pub fn get_session_as_mut(&mut self) -> &mut Session {
        &mut self.session
    }

    #[cfg(feature = "wasm-bindgen")]
    pub fn get_session_as_mut(&mut self) -> Session {
        self.session.clone()
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    pub fn get_session_as_ref(&self) -> &Session {
        &self.session
    }

    #[cfg(feature = "wasm-bindgen")]
    pub fn get_session_as_ref(&self) -> Session {
        self.session.clone()
    }
}


#[cfg(test)]
mod tests {

    use crate::GamePlay;

    use super::*;

    #[test]
    fn test_ship_allocation() {
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